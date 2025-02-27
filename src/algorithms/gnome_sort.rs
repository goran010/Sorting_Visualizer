use super::{Reasons, Sorter};
use crate::sound::play_beep;

/// Represents the state of the Gnome Sort algorithm.
pub struct GnomeSort {
    index: usize, // Current index in the array
    finished: bool,
    comparisons: usize, // counts the number of comparisons
    swaps: usize,       // Indicates whether sorting is complete
}

impl GnomeSort {
    /// Creates a new instance of the algorithm.
    pub fn new() -> Self {
        Self {
            index: 1,        // Starts at the second element, like in the C++ version
            finished: false, // Sorting is not finished initially.
            comparisons: 0,
            swaps: 0,
        }
    }
}

impl Sorter for GnomeSort {
    /// Initializes a new sorter instance.
    fn new() -> Self {
        Self::new()
    }

    /// Returns the indices currently being compared or swapped.
    fn special(&self) -> (usize, usize) {
        if self.finished {
            (usize::MAX, usize::MAX)
        } else {
            (self.index.saturating_sub(1), self.index) // Prevents underflow
        }
    }

    /// Returns the reason for the current action (comparing or switching).
    fn reason(&self) -> Reasons {
        if self.finished {
            Reasons::Comparing
        } else {
            Reasons::Switching
        }
    }

    /// Executes a single step of the Gnome Sort algorithm.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.finished {
            return true;
        }

        // If we reach the end, sorting is complete
        if self.index >= array.len() {
            self.finished = true;
            return true;
        }

        self.comparisons += 1;

        if self.index == 0 {
            self.index += 1;
        } else if array[self.index] >= array[self.index - 1] {
            self.index += 1; // Move forward
        } else {
            array.swap(self.index, self.index - 1); // Swap elements
            self.index -= 1; // Move backward
            self.swaps += 1;
            play_beep();
        }

        false
    }

    /// Resets the algorithm state.
    fn reset_state(&mut self) {
        *self = Self::new(); // Reset all fields to their initial state.
    }

    /// Checks if sorting is complete.
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
