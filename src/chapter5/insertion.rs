/// Trick is to build a mask that clears all of the affected bits at once.
/// Then shift m to be in line with n and then or them together.
pub fn update_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
    let all_ones = !0;
    let left = all_ones << (j + 1);
    let right = (i << i) - 1;
    let mask = left | right;

    let n_cleared = n & mask;
    let m_shifted = m << i;

    n_cleared | m_shifted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_bits() {
        let updated_bits = update_bits(1024, 19, 2, 6);
        assert_eq!(updated_bits, 1100);
    }
}
