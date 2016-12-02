fn is_substring(word: &str, s: &str) -> bool {
    s.contains(word)
}

pub fn is_string_rot(s1: &str, s2: &str) -> bool {
    let concat = format!("{}{}", s2, s2);
    is_substring(s1, &concat[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rot() {
        let s1 = "waterbottle";
        let s2 = "erbottlewat";

        assert_eq!(is_string_rot(s1, s2), true);
        assert_eq!(is_string_rot(s2, s1), true);

        let s1 = "waterbottle";
        let s2 = "rebottlewat";
        assert_eq!(is_string_rot(s1, s2), false);
    }
}
