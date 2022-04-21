/// The sorted strings should be equal if the two strings
/// are anagrams.
pub fn is_anagram_sort(s1: &mut Vec<u8>, s2: &mut Vec<u8>) -> bool {
    s1.sort_unstable();
    s2.sort_unstable();

    *s1 == *s2
}

/// Keep a map of character to count, build the count
/// with the first string and then subtract the count for
/// the second string. If the count becomes negative or
/// if all of the counts are not zero a t the end, the
/// two strings aren't anagrams.
pub fn is_anagram_map(s1: &Vec<u8>, s2: &Vec<u8>) -> bool {
    let mut counts = [0; 256];
    for ch in s1 {
        counts[*ch as usize] += 1;
    }
    for ch in s2 {
        counts[*ch as usize] -= 1;
        if counts[*ch as usize] < 0 {
            return false;
        }
    }

    for count in counts.iter() {
        if *count != 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_is_anagram_sort() {
    let mut s1 = "cinema".to_string().into_bytes();
    let mut s2 = "iceman".to_string().into_bytes();
    assert_eq!(is_anagram_sort(&mut s1, &mut s2), true);

    let mut s1 = "cinema".to_string().into_bytes();
    let mut s2 = "movie".to_string().into_bytes();
    assert_eq!(is_anagram_sort(&mut s1, &mut s2), false);

    let mut s1 = "aabbb".to_string().into_bytes();
    let mut s2 = "aabbaa".to_string().into_bytes();
    assert_eq!(is_anagram_sort(&mut s1, &mut s2), false);

    let mut s1 = "a".to_string().into_bytes();
    let mut s2 = "b".to_string().into_bytes();
    assert_eq!(is_anagram_sort(&mut s1, &mut s2), false);
}

#[test]
fn test_is_anagram_map() {
    let s1 = "cinema".to_string().into_bytes();
    let s2 = "iceman".to_string().into_bytes();
    assert_eq!(is_anagram_map(&s1, &s2), true);

    let s1 = "cinema".to_string().into_bytes();
    let s2 = "movie".to_string().into_bytes();
    assert_eq!(is_anagram_map(&s1, &s2), false);

    let s1 = "aabbb".to_string().into_bytes();
    let s2 = "aabbaa".to_string().into_bytes();
    assert_eq!(is_anagram_map(&s1, &s2), false);

    let s1 = "a".to_string().into_bytes();
    let s2 = "b".to_string().into_bytes();
    assert_eq!(is_anagram_map(&s1, &s2), false);
}
