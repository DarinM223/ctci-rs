use std::collections::HashSet;

/// The trick is to minimize the space by only keeping track
/// of the rows and columns that will be zeroed out instead of
/// every point that needs to be zeroed out.
/// Time complexity: O(n^2)
/// Space complexity: O(n)
/// It could be done with O(1) space by keeping rows count and
/// the columns count in the first row and column.
pub fn zero_matrix(matrix: &mut [Vec<i32>]) {
    if matrix.is_empty() || matrix[0].len() != matrix.len() {
        return;
    }

    let mut zero_rows = HashSet::with_capacity(matrix.len());
    let mut zero_cols = HashSet::with_capacity(matrix[0].len());
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 0 {
                if !zero_rows.contains(&row) {
                    zero_rows.insert(row);
                }
                if !zero_cols.contains(&col) {
                    zero_cols.insert(col);
                }
            }
        }
    }

    for row in zero_rows {
        for col in matrix[row].iter_mut() {
            *col = 0;
        }
    }
    for col in zero_cols {
        for row in matrix.iter_mut() {
            row[col] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::zero_matrix;

    #[test]
    fn test_zero_matrix() {
        let mut matrix = [
            vec![0, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 0, 0],
            vec![1, 1, 1, 1],
        ];

        zero_matrix(&mut matrix[..]);

        let zero_matrix = [
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
        ];

        assert_eq!(matrix, zero_matrix);
    }
}
