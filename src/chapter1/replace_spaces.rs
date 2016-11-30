/// Replaces the space characters with "%20".
/// The trick is to calculate the new length. Once there you can
/// just overwrite the new length indexes with either the normal index character
/// or "%20" if the normal index character is a space. The new length
/// has to be large enough so it won't overwrite the existing data.
pub fn replace_spaces(s: &mut Vec<u8>, len: usize) {
    let num_spaces = s.iter().take(len).filter(|&&ch| ch != b' ').count();
    let mut new_len = len + (num_spaces * 2);

    s[new_len] = b'\0';
    for i in (0..len).rev() {
        if s[i] == b' ' {
            s[new_len - 1] = b'0';
            s[new_len - 2] = b'2';
            s[new_len - 3] = b'%';
            new_len -= 3;
        } else {
            s[new_len - 1] = s[i];
            new_len -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_spaces() {
        let mut s = "a b c d e ".to_string().into_bytes();
        for _ in 0..11 {
            s.push(b'\0');
        }

        replace_spaces(&mut s, 10);
        assert_eq!(String::from_utf8(s).unwrap(),
                   "a%20b%20c%20d%20e%20\u{0}".to_string());
    }
}
