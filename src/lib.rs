
#[derive(PartialEq, Debug, Clone)]
pub struct Interval {
    lower: usize,
    upper: usize,
}

impl Interval {
    pub fn new(lower: usize, upper: usize) -> Self {
        Interval {
            lower,
            upper,
        }
    }

    pub fn split(&self, points: &Vec<usize>) -> Vec<Interval> {
        let mut result = Vec::new();
        let len = points.len();
        if len > 0 {
            let first = 0;
            let last = len - 1;
            for idx in 0..points.len() {
                let root = points[idx] as f32;
                let from = {
                    if idx == first {
                        self.lower
                    } else {
                        let previous = points[idx - 1] as f32;
                        let from = f32::floor(root - (root - previous) / 2.0) + 1.0;
                        from as usize
                    }
                };
                let to = {
                    if idx == last {
                        self.upper
                    } else {
                        let next = points[idx + 1] as f32;
                        let to = f32::floor(root + (next - root) / 2.0);
                        to as usize
                    }
                };
                result.push(Interval::new(from, to));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_0() {
        let range = Interval::new(0, 15);

        let intervals = range.split(&vec![]);
        assert_eq!(intervals, vec![]);
    }

    #[test]
    fn test_interval_1() {
        let range = Interval::new(0, 15);

        let intervals = range.split(&vec![4]);
        assert_eq!(intervals, vec![
            Interval::new(0, 15),
        ]);
    }

    #[test]
    fn test_interval_2() {
        let range = Interval::new(0, 15);

        let intervals = range.split(&vec![2, 8]);
        assert_eq!(intervals, vec![
            Interval::new(0, 5),
            Interval::new(6, 15),
        ]);
    }

    #[test]
    fn test_interval_2_rational() {
        let range = Interval::new(0, 15);

        let intervals = range.split(&vec![3, 8]);
        assert_eq!(intervals, vec![
            Interval::new(0, 5),
            Interval::new(6, 15),
        ]);
    }

    #[test]
    fn test_interval_2_near() {
        let range = Interval::new(0, 15);

        let intervals = range.split(&vec![3, 4]);
        assert_eq!(intervals, vec![
            Interval::new(0, 3),
            Interval::new(4, 15),
        ]);
    }

    #[test]
    fn test_interval_close() {
        let range = Interval::new(0, 15);

        let intervals = range.split(&vec![4, 4]);
        assert_eq!(intervals, vec![
            Interval::new(0, 4),
            Interval::new(5, 15),
        ]);
    }

    #[test]
    fn test_interval_3() {
        let range = Interval::new(0, 20);

        let intervals = range.split(&vec![2, 10, 15]);
        assert_eq!(intervals, vec![
            Interval::new(0, 6),
            Interval::new(7, 12),
            Interval::new(13, 20),
        ]);
    }
}
