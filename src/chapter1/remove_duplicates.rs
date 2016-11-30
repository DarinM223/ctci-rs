/// The trick is instead of "removing" the duplicates you are really
/// moving nonduplicates to the front, so you look for nonduplicates and insert
/// them into the current incrementing index.
pub fn remove_duplicates(s: &mut Vec<u8>) -> usize {
    let mut set = [false; 256];
    let mut last_updated_index = 0;
    let mut len = 0;

    if s.len() < 2 {
        return s.len();
    }
    for curr_index in 1..s.len() {
        if set[s[last_updated_index] as usize] {
            if !set[s[curr_index] as usize] {
                set[s[curr_index] as usize] = true;
                s[last_updated_index] = s[curr_index];
                last_updated_index += 1;
                len += 1;
            }
        } else {
            set[s[last_updated_index] as usize] = true;
            last_updated_index += 1;
            len += 1;
        }
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut s = "abcdaaag".to_string().into_bytes();
        assert_eq!(remove_duplicates(&mut s), 5);
        assert_eq!(&s[..5], &"abcdg".to_string().into_bytes()[..]);

        let mut s = "abcdaaga".to_string().into_bytes();
        assert_eq!(remove_duplicates(&mut s), 5);
        assert_eq!(&s[..5], &"abcdg".to_string().into_bytes()[..]);

        let mut s = "".to_string().into_bytes();
        assert_eq!(remove_duplicates(&mut s), 0);

        let mut s = "aaaaa".to_string().into_bytes();
        assert_eq!(remove_duplicates(&mut s), 1);

        let mut s = "a".to_string().into_bytes();
        assert_eq!(remove_duplicates(&mut s), 1);
    }
}
