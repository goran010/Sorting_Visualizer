use super::{Reasons, Sorter};
use crate::sound::play_beep;

/// Represents the SelectionSort algorithm and its state.
pub struct SelectionSort {
    current_index: usize, // Tracks the current index in the array that we're processing.
    min_index: usize,     // Tracks the index of the minimum value found during the current pass.
    reason: Reasons, // Tracks the reason for the current action, either comparing or switching.
    is_sorted: bool, // Tracks whether the sorting is complete.
    comparisons: usize, // Indicates if the sorting is finished.
    swaps: usize,
}

impl Sorter for SelectionSort {
    /// Creates a new instance of SelectionSort with initial values.
    fn new() -> Self {
        SelectionSort {
            current_index: 0,           // Start from the first element in the array.
            min_index: 0, // The first element is initially assumed to be the smallest.
            reason: Reasons::Comparing, // Initially, the action is comparing.
            is_sorted: false, // Initially, the sorting is not complete.
            comparisons: 0,
            swaps: 0,
        }
    }

    /// Returns the indices that are currently being compared or swapped.
    /// # Returns
    /// A tuple `(current_index, min_index)` representing the indices involved in the operation.
    fn special(&self) -> (usize, usize) {
        (self.current_index, self.min_index) // These are the indices of the current element and the minimum element.
    }

    /// Returns the reason for the current operation (Comparing or Switching).
    /// # Returns
    /// The `Reasons` enum indicating the current operation.
    fn reason(&self) -> Reasons {
        self.reason // Will return either "Comparing" or "Switching" depending on the current action.
    }

    /// Executes a single step of the SelectionSort algorithm.
    /// # Arguments * `array` - A mutable reference to the array being sorted.
    /// # Returns
    /// * `true` if sorting is complete.
    /// * `false` if sorting is still in progress.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // If the current index is beyond the last element, the sorting is complete.
        if self.current_index >= array.len() {
            self.is_sorted = true; // Mark the sorting as complete.
            return true; // Indicate that sorting is complete.
        }

        // Assume the current element is the smallest in the remaining unsorted portion.
        self.min_index = self.current_index;

        // Iterate through the unsorted portion of the array to find the smallest element.
        for j in (self.current_index + 1)..array.len() {
            self.reason = Reasons::Comparing; // Update reason to "Comparing" during iteration.
            self.comparisons += 1;
            if array[j] < array[self.min_index] {
                // If a smaller element is found...
                self.min_index = j; // Update the index of the smallest element.
            }
        }

        // Swap the current element with the smallest element found in the remaining portion.
        array.swap(self.current_index, self.min_index);

        // Move to the next element in the array for the next iteration.
        self.current_index += 1;

        // After a swap, the reason is updated to "Switching" to indicate that a swap happened.
        self.reason = Reasons::Switching;
        play_beep();
        self.swaps += 1;

        false // Sorting isn't complete yet, so return false.
    }

    /// Resets the state of the SelectionSort algorithm, setting the current index to 0.
    fn reset_state(&mut self) {
        *self = Self::new(); // Reset all fields to their initial state.
    }

    /// Returns whether the sorting process is complete.
    /// # Returns `true` if sorting is finished, otherwise `false`.
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
