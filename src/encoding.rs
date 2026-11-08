use std::fmt::Write as _;

fn is_unreserved(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

fn push_percent(out: &mut String, byte: u8) {
    let _ = write!(out, "%{byte:02X}");
}

pub fn quote_plus(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for &byte in value.as_bytes() {
        if is_unreserved(byte) {
            out.push(byte as char);
        } else if byte == b' ' {
            out.push('+');
        } else {
            push_percent(&mut out, byte);
        }
    }
    out
}

pub fn quote_path(path: &str) -> String {
    fn path_safe(byte: u8) -> bool {
        (0x20..0x7F).contains(&byte)
            && !matches!(
                byte,
                b' ' | b'"' | b'#' | b'<' | b'>' | b'?' | b'`' | b'{' | b'}'
            )
    }

    let bytes = path.as_bytes();
    let mut out = String::with_capacity(path.len());
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte == b'%'
            && i + 2 < bytes.len()
            && bytes[i + 1].is_ascii_hexdigit()
            && bytes[i + 2].is_ascii_hexdigit()
        {
            out.push_str(&path[i..i + 3]);
            i += 3;
            continue;
        }
        if byte != b'%' && (is_unreserved(byte) || path_safe(byte)) {
            out.push(byte as char);
        } else {
            push_percent(&mut out, byte);
        }
        i += 1;
    }
    out
}

pub fn urlencode<'a>(pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> String {
    let mut out = String::new();
    for (key, value) in pairs {
        if !out.is_empty() {
            out.push('&');
        }
        out.push_str(&quote_plus(key));
        out.push('=');
        out.push_str(&quote_plus(value));
    }
    out
}

pub fn py_is_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n'
            | '\x0b'
            | '\x0c'
            | '\r'
            | '\x1c'
            | '\x1d'
            | '\x1e'
            | '\x1f'
            | ' '
            | '\u{85}'
            | '\u{a0}'
            | '\u{1680}'
            | '\u{2000}'
            ..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}'
    )
}

pub fn py_strip(value: &str) -> &str {
    value.trim_matches(py_is_space)
}

pub fn py_splitlines(value: &str) -> Vec<&str> {
    fn is_break(c: char) -> bool {
        matches!(
            c,
            '\n' | '\r'
                | '\x0b'
                | '\x0c'
                | '\x1c'
                | '\x1d'
                | '\x1e'
                | '\u{85}'
                | '\u{2028}'
                | '\u{2029}'
        )
    }

    let mut lines = Vec::new();
    let mut start = 0;
    let mut chars = value.char_indices().peekable();
    while let Some((index, c)) = chars.next() {
        if !is_break(c) {
            continue;
        }
        lines.push(&value[start..index]);
        let mut end = index + c.len_utf8();
        if c == '\r'
            && let Some(&(next_index, '\n')) = chars.peek()
        {
            chars.next();
            end = next_index + 1;
        }
        start = end;
    }
    if start < value.len() {
        lines.push(&value[start..]);
    }
    lines
}
