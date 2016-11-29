use std::collections::HashSet;

pub fn remove_duplicates(s: &mut Vec<u8>) -> usize {
    let mut set = HashSet::new();
    let mut index = 0;
    let mut len = s.len();
    while index < s.len() {
        if set.contains(&s[index]) {
            len -= 1;

            let mut second_index = index + 1;
            while second_index < s.len() && set.contains(&s[second_index]) {
                second_index += 1;
            }
            // If there is no nonduplicate after the duplicate character, end here.
            if second_index >= s.len() {
                return index;
            }

            // Swap the characters denoted by the second and first indexes.
            let temp = s[index];
            s[index] = s[second_index];
            s[second_index] = temp;
        } else {
            set.insert(s[index]);
        }
        index += 1; 
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
        assert_eq!(s, "abcdgaaa".to_string().into_bytes());
    }
}
