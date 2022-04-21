fn rotate_swap(matrix: &mut [Vec<i32>], indexes: &[(usize, usize); 4]) {
    let top_left = matrix[indexes[0].0][indexes[0].1];
    let top_right = matrix[indexes[1].0][indexes[1].1];
    let bottom_right = matrix[indexes[2].0][indexes[2].1];
    let bottom_left = matrix[indexes[3].0][indexes[3].1];

    matrix[indexes[1].0][indexes[1].1] = top_left;
    matrix[indexes[2].0][indexes[2].1] = top_right;
    matrix[indexes[3].0][indexes[3].1] = bottom_right;
    matrix[indexes[0].0][indexes[0].1] = bottom_left;
}

fn rotate_image_rec(matrix: &mut [Vec<i32>], n: usize, mut indexes: [(usize, usize); 4]) {
    if n <= 1 {
        return;
    }

    let mut old_indexes = indexes;

    for _ in 0..n - 1 {
        rotate_swap(matrix, &indexes);

        indexes[0].1 += 1;
        indexes[1].0 += 1;
        indexes[2].1 -= 1;
        indexes[3].0 -= 1;
    }

    old_indexes[0].0 += 1;
    old_indexes[0].1 += 1;
    old_indexes[1].0 += 1;
    old_indexes[1].1 -= 1;
    old_indexes[2].0 -= 1;
    old_indexes[2].1 -= 1;
    old_indexes[3].0 -= 1;
    old_indexes[3].1 += 1;

    rotate_image_rec(matrix, n - 2, old_indexes);
}

/// My initial rotate image algorithm was a recursive one that recursed over every layer.
/// There is a lot of unecessary work in moving the four corner indexes every time it recurses.
pub fn my_rotate_image(matrix: &mut [Vec<i32>]) {
    if matrix.is_empty() || matrix.len() != matrix[0].len() {
        return;
    }

    let n = matrix.len();
    let mut indexes = [(0, 0); 4];
    indexes[0] = (0, 0);
    indexes[1] = (0, n - 1);
    indexes[2] = (n - 1, n - 1);
    indexes[3] = (n - 1, 0);

    rotate_image_rec(matrix, n, indexes);
}

pub fn good_rotate_image(matrix: &mut [Vec<i32>]) {
    if matrix.is_empty() || matrix.len() != matrix[0].len() {
        return;
    }

    let n = matrix.len();
    for layer in 0..n / 2 {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            let top = matrix[first][i];

            matrix[first][i] = matrix[last - offset][first];
            matrix[last - offset][first] = matrix[last][last - offset];
            matrix[last][last - offset] = matrix[i][last];
            matrix[i][last] = top;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_image() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        good_rotate_image(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);

        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        good_rotate_image(&mut matrix);
        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];
        assert_eq!(matrix, expected);
    }
}
