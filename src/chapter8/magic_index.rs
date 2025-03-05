use std::cmp;

pub fn magic_distinct(arr: &Vec<i32>) -> i32 {
    let len = arr.len() as i32;
    magic_distinct_rec(arr, 0, len - 1)
}

fn magic_distinct_rec(arr: &Vec<i32>, start: i32, end: i32) -> i32 {
    if end < start {
        return -1;
    }

    let mid = (start + end) / 2;
    match arr[mid as usize].cmp(&mid) {
        cmp::Ordering::Equal => mid,
        cmp::Ordering::Greater => magic_distinct_rec(arr, start, mid - 1),
        cmp::Ordering::Less => magic_distinct_rec(arr, mid + 1, end),
    }
}

pub fn magic_nondistinct(arr: &Vec<i32>) -> i32 {
    let len = arr.len() as i32;
    magic_nondistinct_rec(arr, 0, len - 1)
}

fn magic_nondistinct_rec(arr: &Vec<i32>, start: i32, end: i32) -> i32 {
    if end < start {
        return -1;
    }

    let mid_index = (start + end) / 2;
    let value = arr[mid_index as usize];
    if mid_index == value {
        return mid_index;
    }

    let left_index = cmp::min(mid_index - 1, value);
    let left = magic_nondistinct_rec(arr, start, left_index);
    if left >= 0 {
        return left;
    }

    let right_index = cmp::max(mid_index + 1, value);
    magic_nondistinct_rec(arr, right_index, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct() {
        let elems = vec![-40, -20, -1, 1, 2, 3, 5, 7, 9, 12, 13];

        assert_eq!(magic_distinct(&elems), 7);
        assert_eq!(magic_nondistinct(&elems), 7);
    }

    #[test]
    fn test_nondistinct() {
        let elems = vec![-10, -5, 2, 2, 2, 3, 4, 7, 9, 12, 13];

        assert_eq!(magic_nondistinct(&elems), 2);
    }
}
