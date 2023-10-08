use tinyvec::ArrayVec;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalResult {
    /// The amount of thresholds that are met for each goal.
    pub achieved_goals: ArrayVec<[usize; MAX_GOALS]>,
}

impl GoalResult {
    pub fn from_scores(score: &[u32], goals: &[Goal]) -> Self {
        debug_assert_eq!(score.len(), goals.len());
        Self {
            achieved_goals: score
                .iter()
                .zip(goals.iter())
                .map(|(s, g)| g.effect_value_thresholds.iter().filter(|t| s >= t).count())
                .collect(),
        }
    }

    pub fn is_strictly_better(&self, other: &Self) -> bool {
        debug_assert_eq!(self.achieved_goals.len(), other.achieved_goals.len());
        self.achieved_goals
            .iter()
            .zip(other.achieved_goals.iter())
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
                assert!(GoalResult { achieved_goals: $a }.is_strictly_better(&GoalResult { achieved_goals: $b }));
            )*
        };
        (not better: $(($a:expr, $b:expr),)*) => {
            $(
                assert!(!GoalResult { achieved_goals: $a }.is_strictly_better(&GoalResult { achieved_goals: $b }));
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
