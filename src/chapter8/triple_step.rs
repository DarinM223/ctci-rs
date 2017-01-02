/// Completely recursive way which is around O(3^n) time complexity.
pub fn triple_step_rec(steps: i32) -> i32 {
    if steps < 0 {
        0
    } else if steps == 0 {
        1
    } else {
        triple_step_rec(steps - 1) + triple_step_rec(steps - 2) + triple_step_rec(steps - 3)
    }
}

/// Bottom up method which doesn't use any recursion and runs in around O(n) time.
pub fn triple_step_bottom_up(steps: i32) -> i32 {
    let mut cached_steps = vec![0; steps as usize + 1];
    for curr_steps in 0..(steps + 1) {
        if curr_steps == 0 {
            cached_steps[curr_steps as usize] = 1;
        } else {
            let mut count_steps = 0;
            let possible_steps = [1, 2, 3];
            for step_amount in possible_steps.into_iter() {
                if curr_steps - step_amount >= 0 {
                    count_steps += cached_steps[(curr_steps - step_amount) as usize];
                }
            }
            cached_steps[curr_steps as usize] = count_steps;
        }
    }

    cached_steps[steps as usize]
}

/// Memoized method which is also around O(n) time.
pub fn triple_step_memo(steps: i32) -> i32 {
    let mut memo = vec![None; steps as usize + 1];
    triple_step_memo_rec(steps, &mut memo)
}

fn triple_step_memo_rec(steps: i32, memo: &mut Vec<Option<i32>>) -> i32 {
    if steps < 0 {
        0
    } else if steps == 0 {
        1
    } else if let Some(cached) = memo[steps as usize] {
        cached
    } else {
        let count = triple_step_memo_rec(steps - 1, memo) + triple_step_memo_rec(steps - 2, memo) +
                    triple_step_memo_rec(steps - 3, memo);
        memo[steps as usize] = Some(count);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple_step() {
        for steps in 0..10 {
            assert_eq!(triple_step_rec(steps), triple_step_bottom_up(steps));
            assert_eq!(triple_step_rec(steps), triple_step_memo(steps));
        }
    }
}
