use base64::Engine as _;
use base64::alphabet;
use base64::engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig};
use serde_json::{Map, Value};

use crate::encoding::py_strip;

pub const DEFAULT_MIN_REMAINING_SECONDS: i64 = 3600;

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

const LENIENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new()
        .with_decode_padding_mode(DecodePaddingMode::Indifferent)
        .with_decode_allow_trailing_bits(true),
);

pub fn decode_payload(token: &str) -> Option<Map<String, Value>> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    // Lenient decode: map the URL-safe alphabet and drop anything outside the standard one.
    let cleaned: String = parts[1]
        .chars()
        .map(|c| match c {
            '-' => '+',
            '_' => '/',
            other => other,
        })
        .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '/')
        .collect();
    let bytes = LENIENT.decode(cleaned.as_bytes()).ok()?;
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(&bytes);
    let text = std::str::from_utf8(bytes).ok()?;
    match serde_json::from_str::<Value>(text).ok()? {
        Value::Object(map) if !map.is_empty() => Some(map),
        _ => None,
    }
}

fn exp_as_int(value: &Value) -> Option<i64> {
    match value {
        Value::Bool(flag) => Some(i64::from(*flag)),
        Value::Number(number) => {
            if let Some(int) = number.as_i64() {
                Some(int)
            } else if let Some(uint) = number.as_u64() {
                Some(i64::try_from(uint).unwrap_or(i64::MAX))
            } else {
                // Truncate toward zero, saturating.
                number
                    .as_f64()
                    .filter(|f| f.is_finite())
                    .map(|f| f.trunc() as i64)
            }
        }
        _ => None,
    }
}

pub fn remaining_seconds_at(cookie: Option<&str>, now: i64) -> Option<i64> {
    let cookie = py_strip(cookie?);
    if cookie.is_empty() {
        return None;
    }
    let payload = decode_payload(cookie)?;
    let exp = exp_as_int(payload.get("exp")?)?;
    Some(exp.saturating_sub(now))
}

pub fn remaining_seconds(cookie: Option<&str>) -> Option<i64> {
    remaining_seconds_at(cookie, now_secs())
}

pub fn needs_refresh_at(cookie: Option<&str>, min_remaining_seconds: i64, now: i64) -> bool {
    match remaining_seconds_at(cookie, now) {
        None => true,
        Some(remaining) => remaining < min_remaining_seconds,
    }
}

pub fn needs_refresh(cookie: Option<&str>) -> bool {
    needs_refresh_at(cookie, DEFAULT_MIN_REMAINING_SECONDS, now_secs())
}
