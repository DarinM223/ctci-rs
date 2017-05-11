pub fn make_change(amount: i32) -> i32 {
    let denoms = vec![25, 10, 5, 1];
    let mut cache = vec![vec![0; denoms.len()]; amount as usize + 1];
    make_change_rec(amount, &denoms, 0, &mut cache)
}

fn make_change_rec(amount: i32, denom: &Vec<i32>, index: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
    if index >= denom.len() - 1 {
        return 1;
    } else if cache[amount as usize][index] > 0 {
        return cache[amount as usize][index];
    }

    let coin_amount = denom[index];
    let mut coins = 0;
    let mut ways = 0;

    while coins * coin_amount <= amount {
        let remaining = amount - coins * coin_amount;
        ways += make_change_rec(remaining, denom, index + 1, cache);
        coins += 1;
    }

    cache[amount as usize][index] = ways;
    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_change() {
        assert_eq!(make_change(7), 2);
        assert_eq!(make_change(15), 6);
    }
}
