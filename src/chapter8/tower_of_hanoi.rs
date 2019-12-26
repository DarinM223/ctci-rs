pub fn move_disks(
    n: i32,
    origin: &mut Vec<i32>,
    destination: &mut Vec<i32>,
    buffer: &mut Vec<i32>,
) {
    if n <= 0 {
        return;
    }

    move_disks(n - 1, origin, buffer, destination);

    let last = origin.pop().unwrap();
    destination.push(last);

    move_disks(n - 1, buffer, destination, origin);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi() {
        let mut tower1 = vec![5, 4, 3, 2, 1];
        let mut tower2 = vec![];
        let mut tower3 = vec![];

        move_disks(5, &mut tower1, &mut tower3, &mut tower2);

        assert_eq!(tower1, vec![]);
        assert_eq!(tower2, vec![]);
        assert_eq!(tower3, vec![5, 4, 3, 2, 1]);
    }
}
