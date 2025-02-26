use super::{Reasons, Sorter};
use crate::sound::play_beep;

/// Represents the InsertionSort algorithm and its state.
pub struct InsertionSort {
    current_index: usize, // Tracks the current index in the array being processed.
    sorted_index: usize,  // Tracks the index of the last sorted element.
    reason: Reasons,      // The reason for the current action (e.g., Comparing or Inserting).
    is_sorted: bool,      // Indicates whether the sorting is complete.
}

impl Sorter for InsertionSort {
    /// Creates a new instance of the InsertionSort algorithm with initial values.
    fn new() -> Self {
        InsertionSort {
            current_index: 1,           // Sorting starts from the second element in the array.
            sorted_index: 0,            // The first element is initially considered sorted.
            reason: Reasons::Comparing, // Initial action is set to "Comparing".
            is_sorted: false,           // Initially, the sorting process is not complete.
        }
    }

    /// Returns the indices currently being compared or moved.
    ///
    /// # Returns
    /// A tuple of `(current_index, sorted_index)` representing the indices involved.
    fn special(&self) -> (usize, usize) {
        (self.current_index, self.sorted_index) // Return the indices of the current and sorted elements.
    }

    /// Returns the reason for the current operation.
    ///
    /// # Returns
    /// The `Reasons` enum indicating whether the current action is Comparing or Inserting.
    fn reason(&self) -> Reasons {
        self.reason // Return the current reason for the operation.
    }

    /// Executes a single step of the InsertionSort algorithm.
    ///
    /// # Arguments
    /// * `array` - A mutable reference to the array being sorted.
    ///
    /// # Returns
    /// * `true` if sorting is complete.
    /// * `false` if sorting is still in progress.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // Check if the current index has reached the end of the array.
        if self.current_index >= array.len() {
            self.is_sorted = true; // Mark the sorting as complete.
            return true; // Indicate that the sorting process is finished.
        }

        let mut i = self.current_index; // Initialize with the current index.

        // Compare and insert the element into the correct position in the sorted portion.
        while i > 0 && array[i] < array[i - 1] {
            array.swap(i, i - 1); // Swap elements to maintain sorted order.
            i -= 1; // Move to the previous element in the sorted portion.

            // Update the reason to "Comparing" after each swap.
            self.reason = Reasons::Comparing;
        }

        // Move to the next element for the next iteration.
        self.current_index += 1;

        // Update the reason to "Switching" after insertion.
        self.reason = Reasons::Switching;
        play_beep();

        false // Sorting is not complete yet, so return false.
    }

    /// Resets the state of the InsertionSort algorithm.
    ///
    /// Resets `current_index` to 1 and marks the sorting process as not complete.
    fn reset_state(&mut self) {
        self.current_index = 1; // Reset to the second element.
        self.is_sorted = false; // Mark the sorting as incomplete.
    }

    /// Checks whether the sorting process is complete.
    ///
    /// # Returns
    /// `true` if sorting is finished, otherwise `false`.
    fn is_finished(&self) -> bool {
        self.is_sorted
    }
}
