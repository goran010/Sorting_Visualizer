use super::{Reasons, Sorter};
use crate::sound::play_beep;

/// Represents the state of the Cocktail Shaker Sort algorithm.
pub struct CocktailSort {
    start: usize,   // Start index of the array
    end: usize,     // End index of the array
    swapped: bool,  // Indicates whether elements were swapped
    forward: bool,  // Sorting direction (true = forward, false = backward)
    finished: bool, // Indicates if sorting is complete
}

impl CocktailSort {
    /// Creates a new instance of the algorithm.
    pub fn new() -> Self {
        Self {
            start: 0,
            end: usize::MAX, // Will be set correctly when sorting starts
            swapped: true,
            forward: true,
            finished: false,
        }
    }

    /// Properly initializes sorting parameters based on the array length.
    fn initialize(&mut self, array_len: usize) {
        if array_len == 0 {
            self.finished = true;
            return;
        }
        self.start = 0;
        self.end = array_len - 1;
        self.swapped = true;
        self.forward = true;
        self.finished = false;
    }
}

impl Sorter for CocktailSort {
    /// Initializes sorting.
    fn new() -> Self {
        Self::new()
    }

    /// Returns the indices currently being compared or swapped.
    fn special(&self) -> (usize, usize) {
        if self.finished || self.start >= self.end {
            (usize::MAX, usize::MAX)
        } else if self.forward {
            (self.end.saturating_sub(1), self.end) // Prevents underflow
        } else {
            (self.start, self.start + 1)
        }
    }

    /// Returns the reason for the current action.
    fn reason(&self) -> Reasons {
        if self.finished {
            Reasons::Comparing
        } else {
            Reasons::Switching
        }
    }

    /// Executes a single step of the Cocktail Shaker Sort algorithm.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.finished {
            return true;
        }

        if self.end == usize::MAX {
            // Ensure proper initialization
            self.initialize(array.len());
        }

        play_beep();

        // If no swaps occurred in the last full pass, sorting is done
        if !self.swapped {
            self.finished = true;
            return true;
        }

        self.swapped = false;

        if self.forward {
            // Moves from left to right (Bubble Sort style)
            for i in self.start..self.end {
                if array[i] > array[i + 1] {
                    array.swap(i, i + 1);
                    self.swapped = true;
                }
            }
            if self.end > 0 {
                self.end -= 1; // Prevent underflow
            }
        } else {
            // Moves from right to left
            for i in (self.start + 1..=self.end).rev() {
                if array[i - 1] > array[i] {
                    array.swap(i - 1, i);
                    self.swapped = true;
                }
            }
            self.start += 1;
        }

        // If no swaps occurred in the entire forward and backward pass, finish sorting
        if !self.swapped {
            self.finished = true;
        }

        // Change direction
        self.forward = !self.forward;
        false
    }

    /// Resets the algorithm state.
    fn reset_state(&mut self) {
        self.end = usize::MAX; // This will force proper initialization
        self.finished = false;
        self.swapped = true;
    }

    /// Checks if sorting is complete.
    fn is_finished(&self) -> bool {
        self.finished
    }
}
