use std::os::raw::c_char;

/// Does an in-place reverse of a C-style string
/// where the string is a pointer array of chars
/// including the null byte.
pub unsafe fn reverse_c_string(s: *mut c_char) {
    let mut begin_ptr = s;
    let mut end_ptr = s;

    while *end_ptr != 0 {
        end_ptr = end_ptr.offset(1);
    }
    // Don't forget to go back again once we are at the null byte.
    end_ptr = end_ptr.offset(-1);

    // Swap elements with begin_ptr going forward and end_ptr going backward.
    while begin_ptr < end_ptr {
        let temp = *begin_ptr;
        *begin_ptr = *end_ptr;
        *end_ptr = temp;

        begin_ptr = begin_ptr.offset(1);
        end_ptr = end_ptr.offset(-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_reverse_c_string() {
        unsafe {
            let ptr = CString::new("Hello").unwrap().into_raw();
            reverse_c_string(ptr);
            let reversed = CString::from_raw(ptr);
            assert_eq!(reversed.to_string_lossy(), "olleH".to_string());
        }
    }
}
