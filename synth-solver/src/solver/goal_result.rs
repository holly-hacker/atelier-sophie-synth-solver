use tinyvec::ArrayVec;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalResult {
    /// The amount of thresholds that are met for each goal.
    pub scores: ArrayVec<[usize; MAX_GOALS]>,
}

impl GoalResult {
    pub fn from_scores(scores: &[u32], goals: &[Goal]) -> Self {
        debug_assert_eq!(scores.len(), goals.len());
        Self {
            scores: scores
                .iter()
                .zip(goals.iter())
                .map(|(s, g)| g.effect_value_thresholds.iter().filter(|t| s >= t).count())
                .collect(),
        }
    }

    pub fn is_strictly_better(&self, other: &Self) -> bool {
        debug_assert_eq!(self.scores.len(), other.scores.len());
        self.scores
            .iter()
            .zip(other.scores.iter())
            .all(|(a, b)| a >= b)
    }
}

#[cfg(test)]
mod tests {
    use tinyvec::array_vec;

    use super::*;

    macro_rules! is_strictly_better {
        (better: $(($a:expr, $b:expr),)*) => {
            $(
                assert!(GoalResult { scores: $a }.is_strictly_better(&GoalResult { scores: $b }));
            )*
        };
        (not better: $(($a:expr, $b:expr),)*) => {
            $(
                assert!(!GoalResult { scores: $a }.is_strictly_better(&GoalResult { scores: $b }));
            )*
        };
    }

    #[test]
    fn test_strictly_better() {
        is_strictly_better![
            better:
            (array_vec![1, 1, 1], array_vec![1, 0, 1]),
            (array_vec![1, 1, 1], array_vec![0, 1, 1]),
            (array_vec![1, 1, 1], array_vec![0, 0, 1]),
            (array_vec![1, 0, 0], array_vec![0, 0, 0]),
        ];
        is_strictly_better![
            not better:
            (array_vec![1, 1, 1], array_vec![2, 0, 0]),
            (array_vec![1, 0, 1], array_vec![1, 1, 0]),
            (array_vec![0, 0, 1], array_vec![2, 0, 0]),
        ];
    }
}
