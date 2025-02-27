use super::{Reasons, Sorter};
use crate::sound::play_beep;

pub struct CombSort {
    gap: usize,
    swapped: bool,
    i: usize,
    finished: bool,
    swaps: usize, // Indicates if the sorting is finished.
    comparisons: usize,
}

impl CombSort {
    pub fn new() -> Self {
        Self {
            gap: 0,
            swapped: true,
            i: 0,
            finished: false, // Sorting is not finished initially.
            comparisons: 0,
            swaps: 0,
        }
    }

    fn get_next_gap(gap: usize) -> usize {
        let new_gap = (gap * 10) / 13;
        if new_gap < 1 { 1 } else { new_gap }
    }
}

impl Sorter for CombSort {
    fn new() -> Self {
        Self::new()
    }

    fn special(&self) -> (usize, usize) {
        if self.finished {
            (usize::MAX, usize::MAX)
        } else {
            (self.i, self.i + self.gap.min(1)) // Highlights compared elements
        }
    }

    fn reason(&self) -> Reasons {
        if self.finished {
            Reasons::Comparing
        } else {
            Reasons::Switching
        }
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.finished {
            return true;
        }

        // Initialize gap on first run
        if self.gap == 0 {
            self.gap = array.len();
            self.i = 0;
        }

        if self.gap != 1 || self.swapped {
            self.gap = Self::get_next_gap(self.gap);
            self.swapped = false;
            self.i = 0;
        }

        if self.i + self.gap < array.len() {
            self.comparisons += 1;

            if array[self.i] > array[self.i + self.gap] {
                play_beep();
                self.swaps += 1;
                array.swap(self.i, self.i + self.gap);
                self.swapped = true;
            }

            self.i += 1;
        } else if self.gap == 1 && !self.swapped {
            self.finished = true;
        }

        false
    }

    fn reset_state(&mut self) {
        *self = Self::new(); // Reset all fields to their initial state.
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
    fn comparisons(&self) -> usize {
        self.comparisons
    }

    fn swaps(&self) -> usize {
        self.swaps
    }
}
