pub type Point = (usize, usize);

#[derive(Clone, Copy)]
pub struct Dim {
    pub rows: usize,
    pub cols: usize,
}

pub fn get_path(start: Point, end: Point, dim: Dim, offlimits: &[Vec<bool>]) -> Option<Vec<Point>> {
    if start == end {
        Some(vec![start])
    } else if out_of_bounds(start, dim) || off_limits(offlimits, start) {
        None
    } else {
        get_path((start.0 + 1, start.1), end, dim, offlimits)
            .or_else(|| get_path((start.0, start.1 + 1), end, dim, offlimits))
            .map(|mut path| {
                path.push(start);
                path
            })
    }
}

pub fn get_path_memo(
    start: Point,
    end: Point,
    dim: Dim,
    offlimits: &[Vec<bool>],
    cache: &mut Vec<Vec<Option<Option<Vec<Point>>>>>,
) -> Option<Vec<Point>> {
    if start == end {
        Some(vec![start])
    } else if out_of_bounds(start, dim) || off_limits(offlimits, start) {
        None
    } else if let Some(cached) = cache[start.0][start.1].take() {
        // If there is a cached value return the cached value.
        cached
    } else {
        let result = get_path_memo((start.0 + 1, start.1), end, dim, offlimits, cache)
            .or_else(|| get_path_memo((start.0, start.1 + 1), end, dim, offlimits, cache))
            .map(|mut path| {
                path.push(start);
                path
            });
        // Cache the computation.
        cache[start.0][start.1] = Some(result.clone());
        result
    }
}

fn out_of_bounds(point: Point, dim: Dim) -> bool {
    point.0 >= dim.rows || point.1 >= dim.cols
}

fn off_limits(offlimits: &[Vec<bool>], pos: Point) -> bool {
    offlimits.get(pos.0).and_then(|row| row.get(pos.1)) == Some(&true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot() {
        let dim = Dim { rows: 4, cols: 4 };
        let mut offlimits = vec![vec![false; dim.cols]; dim.rows];
        offlimits[2][0] = true;
        offlimits[1][1] = true;
        offlimits[2][3] = true;

        let mut cache = vec![vec![None; dim.cols]; dim.rows];

        assert_eq!(
            get_path((0, 0), (3, 3), dim, &offlimits),
            Some(vec![(3, 3), (3, 2), (2, 2), (1, 2), (0, 2), (0, 1), (0, 0)])
        );
        assert_eq!(
            get_path_memo((0, 0), (3, 3), dim, &offlimits, &mut cache),
            Some(vec![(3, 3), (3, 2), (2, 2), (1, 2), (0, 2), (0, 1), (0, 0)])
        );
    }
}
