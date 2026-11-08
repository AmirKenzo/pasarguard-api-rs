use std::fmt;

use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PanelDateTime {
    naive: NaiveDateTime,
    offset: Option<FixedOffset>,
}

impl PanelDateTime {
    pub fn naive(naive: NaiveDateTime) -> Self {
        Self {
            naive: truncate_micros(naive),
            offset: None,
        }
    }

    pub fn aware<Tz: TimeZone>(value: DateTime<Tz>) -> Self {
        let fixed = value.fixed_offset();
        Self {
            naive: truncate_micros(fixed.naive_local()),
            offset: Some(*fixed.offset()),
        }
    }

    pub fn from_unix_utc(secs: i64) -> Option<Self> {
        DateTime::<Utc>::from_timestamp(secs, 0).map(Self::aware)
    }

    pub fn naive_local(&self) -> NaiveDateTime {
        self.naive
    }

    pub fn offset(&self) -> Option<FixedOffset> {
        self.offset
    }

    pub fn is_aware(&self) -> bool {
        self.offset.is_some()
    }

    pub fn to_utc(&self) -> DateTime<Utc> {
        match self.offset {
            Some(offset) => offset
                .from_local_datetime(&self.naive)
                .single()
                .map(|value| value.with_timezone(&Utc))
                .unwrap_or_else(|| Utc.from_utc_datetime(&self.naive)),
            None => Utc.from_utc_datetime(&self.naive),
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.to_utc().timestamp()
    }

    pub fn isoformat(&self) -> String {
        let mut out = format_naive(&self.naive);
        if let Some(offset) = self.offset {
            out.push_str(&format_offset(offset));
        }
        out
    }

    pub fn json_format(&self) -> String {
        let mut out = format_naive(&self.naive);
        match self.offset {
            Some(offset) if offset.local_minus_utc() == 0 => out.push('Z'),
            Some(offset) => out.push_str(&format_offset(offset)),
            None => {}
        }
        out
    }

    pub fn parse_str(value: &str) -> Option<Self> {
        let trimmed = value.trim();
        if !trimmed.is_empty()
            && trimmed
                .bytes()
                .all(|b| b.is_ascii_digit() || b == b'.' || b == b'-')
            && let Ok(number) = trimmed.parse::<f64>()
        {
            return Self::from_unix_number(number);
        }
        parse_iso(trimmed)
    }

    pub fn from_unix_number(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let seconds = if value.abs() > 2e10 {
            value / 1000.0
        } else {
            value
        };
        let whole = seconds.floor();
        let micros = ((seconds - whole) * 1_000_000.0).round() as i64;
        let base = DateTime::<Utc>::from_timestamp(whole as i64, 0)?;
        Some(Self::aware(base + chrono::Duration::microseconds(micros)))
    }
}

impl fmt::Display for PanelDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.isoformat())
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for PanelDateTime {
    fn from(value: DateTime<Tz>) -> Self {
        Self::aware(value)
    }
}

impl From<NaiveDateTime> for PanelDateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self::naive(value)
    }
}

impl Serialize for PanelDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.json_format())
    }
}

impl<'de> Deserialize<'de> for PanelDateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DateTimeVisitor;

        impl Visitor<'_> for DateTimeVisitor {
            type Value = PanelDateTime;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an ISO 8601 datetime string or a unix timestamp")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                PanelDateTime::parse_str(value)
                    .ok_or_else(|| E::custom(format!("invalid datetime: {value:?}")))
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
                PanelDateTime::from_unix_number(value as f64)
                    .ok_or_else(|| E::custom("timestamp out of range"))
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                PanelDateTime::from_unix_number(value as f64)
                    .ok_or_else(|| E::custom("timestamp out of range"))
            }

            fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
                PanelDateTime::from_unix_number(value)
                    .ok_or_else(|| E::custom("timestamp out of range"))
            }
        }

        deserializer.deserialize_any(DateTimeVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateTimeOrInt {
    Int(i64),
    DateTime(PanelDateTime),
}

impl DateTimeOrInt {
    pub fn timestamp(&self) -> i64 {
        match self {
            Self::Int(value) => *value,
            Self::DateTime(value) => value.timestamp(),
        }
    }
}

impl From<i64> for DateTimeOrInt {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<PanelDateTime> for DateTimeOrInt {
    fn from(value: PanelDateTime) -> Self {
        Self::DateTime(value)
    }
}

impl Serialize for DateTimeOrInt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Int(value) => serializer.serialize_i64(*value),
            Self::DateTime(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DateTimeOrInt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UnionVisitor;

        impl Visitor<'_> for UnionVisitor {
            type Value = DateTimeOrInt;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an integer timestamp or a datetime")
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(DateTimeOrInt::Int(value))
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                i64::try_from(value)
                    .map(DateTimeOrInt::Int)
                    .map_err(|_| E::custom("integer out of range"))
            }

            fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
                PanelDateTime::from_unix_number(value)
                    .map(DateTimeOrInt::DateTime)
                    .ok_or_else(|| E::custom("timestamp out of range"))
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                PanelDateTime::parse_str(value)
                    .map(DateTimeOrInt::DateTime)
                    .ok_or_else(|| E::custom(format!("invalid datetime: {value:?}")))
            }
        }

        deserializer.deserialize_any(UnionVisitor)
    }
}

fn truncate_micros(value: NaiveDateTime) -> NaiveDateTime {
    let micros = value.nanosecond() / 1_000 % 1_000_000;
    value.with_nanosecond(micros * 1_000).unwrap_or(value)
}

fn format_naive(value: &NaiveDateTime) -> String {
    let mut out = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        value.year(),
        value.month(),
        value.day(),
        value.hour(),
        value.minute(),
        value.second()
    );
    let micros = value.nanosecond() / 1_000 % 1_000_000;
    if micros != 0 {
        out.push_str(&format!(".{micros:06}"));
    }
    out
}

fn format_offset(offset: FixedOffset) -> String {
    let total = offset.local_minus_utc();
    let sign = if total < 0 { '-' } else { '+' };
    let total = total.abs();
    let (hours, minutes, seconds) = (total / 3600, total / 60 % 60, total % 60);
    if seconds != 0 {
        format!("{sign}{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{sign}{hours:02}:{minutes:02}")
    }
}

fn parse_iso(value: &str) -> Option<PanelDateTime> {
    if value.len() == 10 {
        let date = NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()?;
        return Some(PanelDateTime::naive(date.and_hms_opt(0, 0, 0)?));
    }
    let normalised = value.replacen(' ', "T", 1);
    let (body, offset) = split_offset(&normalised)?;
    let naive = ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%dT%H:%M"]
        .iter()
        .find_map(|format| NaiveDateTime::parse_from_str(body, format).ok())?;
    Some(match offset {
        Some(offset) => PanelDateTime {
            naive: truncate_micros(naive),
            offset: Some(offset),
        },
        None => PanelDateTime::naive(naive),
    })
}

fn split_offset(value: &str) -> Option<(&str, Option<FixedOffset>)> {
    if let Some(body) = value.strip_suffix('Z').or_else(|| value.strip_suffix('z')) {
        return Some((body, FixedOffset::east_opt(0)));
    }
    let time_start = value.find('T')?;
    let Some(sign_index) = value[time_start..]
        .rfind(['+', '-'])
        .map(|i| i + time_start)
    else {
        return Some((value, None));
    };
    let sign = if value.as_bytes()[sign_index] == b'-' {
        -1
    } else {
        1
    };
    let digits: String = value[sign_index + 1..]
        .chars()
        .filter(|c| *c != ':')
        .collect();
    if digits.len() != 4 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let hours: i32 = digits[..2].parse().ok()?;
    let minutes: i32 = digits[2..].parse().ok()?;
    let offset = FixedOffset::east_opt(sign * (hours * 3600 + minutes * 60))?;
    Some((&value[..sign_index], Some(offset)))
}
