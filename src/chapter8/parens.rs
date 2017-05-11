pub fn parens(n: i32) -> Vec<String> {
    let mut s = vec![0; n as usize * 2];
    let mut list = vec![];
    add_parens(&mut list, n, n, &mut s, 0);
    list
}

fn add_parens(list: &mut Vec<String>, left_rem: i32, right_rem: i32, s: &mut Vec<u8>, index: i32) {
    if left_rem < 0 || right_rem < left_rem {
        return;
    }

    if left_rem == 0 && right_rem == 0 {
        list.push(String::from_utf8(s.clone()).unwrap());
    } else {
        s[index as usize] = b'(';
        add_parens(list, left_rem - 1, right_rem, s, index + 1);

        s[index as usize] = b')';
        add_parens(list, left_rem, right_rem - 1, s, index + 1);
    }
}
