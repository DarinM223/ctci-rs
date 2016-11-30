/// The sorted strings should be equal if the two strings
/// are anagrams.
pub fn is_anagram_sort(s1: Vec<u8>, s2: Vec<u8>) -> bool {
    let (mut s1, mut s2) = (s1, s2);
    s1.sort();
    s2.sort();

    s1 == s2
}

/// Keep a map of character to count, build the count
/// with the first string and then subtract the count for
/// the second string. If the count becomes negative or
/// if all of the counts are not zero a t the end, the
/// two strings aren't anagrams.
pub fn is_anagram_map(s1: Vec<u8>, s2: Vec<u8>) -> bool {
    let mut counts = [0; 256];
    for ch in s1 {
        counts[ch as usize] = counts[ch as usize] + 1;
    }
    for ch in s2 {
        counts[ch as usize] = counts[ch as usize] - 1;
        if counts[ch as usize] < 0 {
            return false;
        }
    }

    for i in 0..counts.len() {
        if counts[i] != 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_is_anagram_sort() {
    let s1 = "cinema".to_string();
    let s2 = "iceman".to_string();
    assert_eq!(is_anagram_sort(s1.into_bytes(), s2.into_bytes()), true);

    let s1 = "cinema".to_string();
    let s2 = "movie".to_string();
    assert_eq!(is_anagram_sort(s1.into_bytes(), s2.into_bytes()), false);

    let s1 = "aabbb".to_string();
    let s2 = "aabbaa".to_string();
    assert_eq!(is_anagram_sort(s1.into_bytes(), s2.into_bytes()), false);

    let s1 = "a".to_string();
    let s2 = "b".to_string();
    assert_eq!(is_anagram_sort(s1.into_bytes(), s2.into_bytes()), false);
}

#[test]
fn test_is_anagram_map() {
    let s1 = "cinema".to_string();
    let s2 = "iceman".to_string();
    assert_eq!(is_anagram_map(s1.into_bytes(), s2.into_bytes()), true);

    let s1 = "cinema".to_string();
    let s2 = "movie".to_string();
    assert_eq!(is_anagram_map(s1.into_bytes(), s2.into_bytes()), false);

    let s1 = "aabbb".to_string();
    let s2 = "aabbaa".to_string();
    assert_eq!(is_anagram_map(s1.into_bytes(), s2.into_bytes()), false);

    let s1 = "a".to_string();
    let s2 = "b".to_string();
    assert_eq!(is_anagram_map(s1.into_bytes(), s2.into_bytes()), false);
}
