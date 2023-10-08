use std::ops::ControlFlow;

pub type ProgressReporter<TResult> = Box<dyn Fn(f32, TResult) -> ControlFlow<()>>;

pub struct ProgressTracker<TResult: Clone> {
    /// The progress reporter that should be called when the progress changes.
    progress_reporter: Option<ProgressReporter<TResult>>,
    /// count and total for each progress level
    progress_stack: Vec<(usize, usize)>,

    max_depth_encountered: usize,
}

impl<TResult: Clone> ProgressTracker<TResult> {
    /// The amount of depth levels we don't report progress in, to optimize performance.
    const REPORT_DEPTH: usize = 3;

    pub fn new(progress_reporter: Option<ProgressReporter<TResult>>) -> Self {
        Self {
            progress_reporter,
            progress_stack: vec![],
            max_depth_encountered: 0,
        }
    }

    pub fn start_loop(&mut self, count: usize) {
        self.progress_stack.push((0, count));
        self.max_depth_encountered = self.max_depth_encountered.max(self.progress_stack.len());
    }

    pub fn end_loop(&mut self) {
        // pop the top-level item (consider it finished) and increment the next top-level item
        let popped = self.progress_stack.pop();
        if let Some(popped) = popped {
            debug_assert_eq!(popped.0, popped.1, "popped loop before it was finished");
        }
    }

    pub fn bump_loop_progress(&mut self) {
        if let Some((progress, _)) = self.progress_stack.last_mut() {
            *progress += 1;
        }
    }

    pub fn report_progress(&mut self, result: &TResult) -> ControlFlow<()> {
        if self.progress_stack.len()
            > (self
                .max_depth_encountered
                .saturating_sub(Self::REPORT_DEPTH))
        {
            return ControlFlow::Continue(());
        }

        let Some(progress_reporter) = &self.progress_reporter else {
            return ControlFlow::Continue(());
        };

        progress_reporter(self.get_current_progress(), result.clone())
    }

    fn get_current_progress(&self) -> f32 {
        let (current_progress, _) =
            self.progress_stack
                .iter()
                .cloned()
                .fold((0., 1.), |(total, mult), (p_cur, p_max)| {
                    let current_progress = p_cur as f32 / p_max as f32;
                    let progress_delta = current_progress * mult;
                    (total + progress_delta, mult * (1. / p_max as f32))
                });

        current_progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_counts_up_to_1() {
        let mut tracker = ProgressTracker::<()>::new(None);

        tracker.start_loop(5);
        for _ in 0..5 {
            tracker.report_progress(&());
            tracker.start_loop(4);
            for _ in 0..4 {
                tracker.report_progress(&());
                tracker.start_loop(3);
                for _ in 0..3 {
                    tracker.report_progress(&());
                    tracker.start_loop(2);
                    for _ in 0..2 {
                        tracker.report_progress(&());
                        tracker.start_loop(1);
                        for _ in 0..1 {
                            tracker.report_progress(&());
                            tracker.start_loop(0);
                            tracker.end_loop();

                            tracker.bump_loop_progress();
                        }
                        tracker.end_loop();

                        tracker.bump_loop_progress();
                    }
                    tracker.end_loop();

                    tracker.bump_loop_progress();
                }
                tracker.end_loop();

                tracker.bump_loop_progress();
            }
            tracker.end_loop();

            tracker.bump_loop_progress();
        }
        // tracker.pop_depth();

        assert_eq!(tracker.progress_stack.len(), 1);

        const ROUND_AMOUNT: f32 = 0.0001;
        let rounded_progress =
            (tracker.get_current_progress() / ROUND_AMOUNT).round() * ROUND_AMOUNT;
        assert_eq!(rounded_progress, 1.);
    }
}
