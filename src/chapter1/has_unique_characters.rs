/// Returns true if all characters are unique, false otherwise.
/// The trick for this is to use an array instead of a HashSet since
/// the size is fixed (256 possible characters assuming ASCII).
pub fn has_all_unique_chars(s: String) -> bool {
    let mut set = [false; 256];
    for byte in s.bytes() {
        if set[byte as usize] == true {
            return false;
        }
        set[byte as usize] = true;
    }

    true
}

/// The trick for this is to use bit operations to store
/// the data in a single integer with each bit indicating true or false.
pub fn has_all_unique_chars_no_data(s: String) -> bool {
    // 256 bits = 32 bytes so the storage needs to be a 32 byte integer.

    let mut set: i32 = 0;
    for byte in s.bytes() {
        let letter = byte - b'a';
        let contains = (set >> letter) & 1;
        if contains == 1 {
            return false;
        }

        set |= 1 << letter;
    }

    true
}

#[test]
fn test_all_unique_chars() {
    let unique = "abcdefg".to_string();
    assert_eq!(has_all_unique_chars(unique), true);

    let nonunique = "abcdefga".to_string();
    assert_eq!(has_all_unique_chars(nonunique), false);
}

#[test]
fn test_all_unique_chars_no_data() {
    let unique = "abcdefg".to_string();
    assert_eq!(has_all_unique_chars_no_data(unique), true);

    let nonunique = "abcdefga".to_string();
    assert_eq!(has_all_unique_chars_no_data(nonunique), false);
}
