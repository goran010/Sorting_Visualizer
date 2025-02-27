use super::{Reasons, Sorter};
use crate::sound::play_beep;

/// Represents the InsertionSort algorithm and its state.
pub struct InsertionSort {
    current_index: usize, // Tracks the current index in the array being processed.
    sorted_index: usize,  // Tracks the index of the last sorted element.
    reason: Reasons,      // The reason for the current action (e.g., Comparing or Inserting).
    is_sorted: bool,      // Indicates whether the sorting is complete.
    swaps: usize,         // Counts the number of swaps performed.
    comparisons: usize,   // Counts the number of comparisons performed.
}

impl Sorter for InsertionSort {
    /// Creates a new instance of the InsertionSort algorithm with initial values.
    fn new() -> Self {
        InsertionSort {
            current_index: 1,           // Sorting starts from the second element in the array.
            sorted_index: 0,            // The first element is initially considered sorted.
            reason: Reasons::Comparing, // Initial action is set to "Comparing".
            is_sorted: false,           // Initially, the sorting process is not complete.
            comparisons: 0,
            swaps: 0,
        }
    }

    /// Returns the indices currently being compared or moved.
    fn special(&self) -> (usize, usize) {
        (self.current_index, self.sorted_index) // Return the indices of the current and sorted elements.
    }

    /// Returns the reason for the current operation.
    fn reason(&self) -> Reasons {
        self.reason // Return the current reason for the operation.
    }

    /// Executes a single step of the InsertionSort algorithm.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // If sorting is finished, return true.
        if self.is_sorted {
            return true;
        }

        if self.current_index >= array.len() {
            self.is_sorted = true; // Mark sorting as complete.
            return true;
        }

        let mut i = self.current_index;
        let value = array[i];

        // Compare and insert the element into the correct position.
        while i > 0 && array[i - 1] > value {
            self.comparisons += 1; // Increment comparisons

            array[i] = array[i - 1]; // Shift element to the right
            i -= 1;
            self.reason = Reasons::Comparing;
        }

        // If an actual swap occurred, update the array and play a beep
        if i != self.current_index {
            array[i] = value;
            self.swaps += 1; // Increment swaps
            self.reason = Reasons::Switching;
            play_beep();
        }

        self.current_index += 1; // Move to the next element

        false // Sorting is not complete yet.
    }

    /// Resets the state of the InsertionSort algorithm.
    fn reset_state(&mut self) {
        *self = Self::new(); // Reset all fields to their initial state.
    }

    /// Checks whether the sorting process is complete.
    fn is_finished(&self) -> bool {
        self.is_sorted
    }

    fn comparisons(&self) -> usize {
        self.comparisons
    }

    fn swaps(&self) -> usize {
        self.swaps
    }
}
