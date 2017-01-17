pub fn min_product(a: i32, b: i32) -> i32 {
    let (bigger, smaller) = if a < b { (b, a) } else { (a, b) };

    min_product_helper(smaller, bigger)
}

fn min_product_helper(smaller: i32, bigger: i32) -> i32 {
    if smaller == 0 {
        return 0;
    } else if smaller == 1 {
        return bigger;
    }

    let s = smaller >> 1;
    let side1 = min_product_helper(s, bigger);
    let mut side2 = side1;

    if smaller % 2 == 1 {
        side2 = min_product_helper(smaller - s, bigger);
    }

    side1 + side2
}

pub fn min_product_memo(a: i32, b: i32) -> i32 {
    let (bigger, smaller) = if a < b { (b, a) } else { (a, b) };

    let mut memo = vec![None; smaller as usize + 1];
    min_product_helper_memo(smaller, bigger, &mut memo)
}

fn min_product_helper_memo(smaller: i32, bigger: i32, memo: &mut Vec<Option<i32>>) -> i32 {
    if smaller == 0 {
        return 0;
    } else if smaller == 1 {
        return bigger;
    } else if let Some(cached) = memo[smaller as usize] {
        return cached;
    }

    let s = smaller >> 1;
    let side1 = min_product_helper_memo(s, bigger, memo);
    let mut side2 = side1;

    if smaller % 2 == 1 {
        side2 = min_product_helper_memo(smaller - s, bigger, memo);
    }

    let result = side1 + side2;
    memo[smaller as usize] = Some(result);
    result
}
