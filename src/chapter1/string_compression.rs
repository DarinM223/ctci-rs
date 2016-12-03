/// Store the current character and the count
/// and write it to the new string only when a different character appears
/// or when the string is at the end.
pub fn compress(s: String) -> String {
    let mut compressed = Vec::with_capacity(s.len());
    let mut state = None;

    for byte in s.bytes() {
        if let Some((ch, count)) = state.take() {
            if ch != byte {
                write_bytes(&mut compressed, ch, count);
                state = Some((byte, 1));
            } else {
                state = Some((byte, count + 1));
            }
        } else {
            state = Some((byte, 1));
        }
    }

    if let Some((ch, count)) = state.take() {
        write_bytes(&mut compressed, ch, count);
    }

    if compressed.len() < s.len() {
        String::from_utf8_lossy(&compressed[..]).to_string()
    } else {
        s
    }
}

fn write_bytes(s: &mut Vec<u8>, ch: u8, count: i32) {
    let format_str = format!("{}{}", ch as char, count);
    for byte in format_str.bytes() {
        s.push(byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression() {
        let s = "aabcccccaaa".to_string();
        assert_eq!(compress(s), "a2b1c5a3".to_string());

        let s = "abc".to_string();
        assert_eq!(compress(s), "abc".to_string());
    }
}
