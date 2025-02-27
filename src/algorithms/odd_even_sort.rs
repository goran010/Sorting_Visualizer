use super::{Reasons, Sorter};
use crate::sound::play_beep;

pub struct OddEvenSort {
    is_sorted: bool,
    odd_phase: bool,
    i: usize,
    finished: bool,
    swaps: usize, // Indicates if the sorting is finished.
    comparisons: usize,
}

impl OddEvenSort {
    pub fn new() -> Self {
        Self {
            is_sorted: false,
            odd_phase: true,
            i: 1,
            finished: false, // Sorting is not finished initially.
            comparisons: 0,
            swaps: 0,
        }
    }
}

impl Sorter for OddEvenSort {
    fn new() -> Self {
        Self::new()
    }

    fn special(&self) -> (usize, usize) {
        if self.finished {
            (usize::MAX, usize::MAX)
        } else {
            (self.i, self.i + 1)
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

        if self.i >= array.len() - 1 {
            // If a full pass is done, check if sorted
            if self.is_sorted {
                self.finished = true;
                return true;
            }

            // Switch phase (odd/even) and reset index
            self.is_sorted = true;
            self.odd_phase = !self.odd_phase;
            self.i = if self.odd_phase { 1 } else { 0 };
        }

        if self.i < array.len() - 1 {
            play_beep();
            self.swaps += 1; // Sound effect for visualization

            if array[self.i] > array[self.i + 1] {
                array.swap(self.i, self.i + 1);
                self.is_sorted = false;
            }

            self.i += 2;
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
