/// Trick: only need to know if the letter has even or odd
/// amount of occurrences so can keep a bitstring and toggle
/// between 0 for even and 1 for odd, then check if there is more
/// than one 1 in the resulting bitstring.
pub fn is_palindrome_permutation(s: String) -> bool {
    // 26 bits with 0 for even and 1 for odd.
    let mut even_map: i32 = 0;

    for byte in s.bytes() {
        if (b'a'..=b'z').contains(&byte) {
            let letter = byte - b'a';
            toggle(&mut even_map, letter as i32);
        }
    }

    // Check that at most 1 bit was set.
    even_map == 0 || ((even_map & (even_map - 1)) == 0)
}

fn toggle(num: &mut i32, position: i32) {
    let is_odd = (*num & (1 << position)) != 0;

    if is_odd {
        *num &= !(1 << position);
    } else {
        *num |= 1 << position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_permutation() {
        let s = "tact coa".to_string();
        assert_eq!(is_palindrome_permutation(s), true);

        let s = "aaabbb".to_string();
        assert_eq!(is_palindrome_permutation(s), false);
    }
}
