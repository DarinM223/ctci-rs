/// Two cases: either when the strings have a length difference of 1
/// in which only one skipped character is allowed, or when the strings
/// have the same length, in which only one different character is allowed.
pub fn one_away(s1: String, s2: String) -> bool {
    let len_diff = s1.len() as i32 - s2.len() as i32;
    if len_diff.abs() > 1 {
        return false;
    }

    if len_diff.abs() == 1 {
        let (bigger, smaller) = if s1.len() > s2.len() {
            (s1, s2)
        } else {
            (s2, s1)
        };

        check_no_replace(smaller.as_bytes(), bigger.as_bytes())
    } else {
        check_one_replace(s1, s2)
    }
}

/// Checks if there is no more than one difference between
/// the characters and when a difference is detected it
/// only increments the larger string index.
fn check_no_replace(smaller: &[u8], bigger: &[u8]) -> bool {
    let mut skipped = false;
    let (mut i, mut j) = (0, 0);
    while i < smaller.len() && j < bigger.len() {
        if smaller[i] != bigger[j] && !skipped {
            j += 1;
            skipped = true;
        } else if smaller[i] != bigger[j] {
            return false;
        } else {
            i += 1;
            j += 1;
        }
    }

    true
}

/// Checks if there is no more than one difference between the
/// characters.
fn check_one_replace(s1: String, s2: String) -> bool {
    let mut changed = false;
    for (b1, b2) in s1.bytes().zip(s2.bytes()) {
        if b1 != b2 && !changed {
            changed = true;
        } else if b1 != b2 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_away() {
        let s1 = "pale".to_string();
        let s2 = "ple".to_string();
        assert_eq!(one_away(s1, s2), true);

        let s1 = "pales".to_string();
        let s2 = "pale".to_string();
        assert_eq!(one_away(s1, s2), true);

        let s1 = "pale".to_string();
        let s2 = "bale".to_string();
        assert_eq!(one_away(s1, s2), true);

        let s1 = "pale".to_string();
        let s2 = "bake".to_string();
        assert_eq!(one_away(s1, s2), false);

        let s1 = "".to_string();
        let s2 = "a".to_string();
        assert_eq!(one_away(s1, s2), true);
    }
}
