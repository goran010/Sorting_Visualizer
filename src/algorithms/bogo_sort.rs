use super::{Reasons, Sorter};
use crate::sound::play_beep;
use rand::prelude::SliceRandom; // Import the SliceRandom trait to shuffle the array

/// Represents the BogoSort algorithm and its state.
pub struct BogoSort {
    is_sorted: bool, // Tracks whether the array is sorted or not.
    reason: Reasons, // Tracks the reason for the current action (either comparing or switching).
}

impl BogoSort {
    /// Checks if the array is sorted by comparing each pair of adjacent elements.
    ///
    /// # Arguments
    /// * `array` - A reference to the array to check.
    ///
    /// # Returns
    /// `true` if the array is sorted, otherwise `false`.
    fn is_sorted_check(&self, array: &[usize]) -> bool {
        array.windows(2).all(|w| w[0] <= w[1]) // Check if each pair of adjacent elements is sorted.
    }
}

impl Sorter for BogoSort {
    /// Initializes a new instance of BogoSort.
    fn new() -> Self {
        BogoSort {
            is_sorted: false,           // Starts by assuming the array is not sorted.
            reason: Reasons::Comparing, // Initial state is "Comparing" because BogoSort will check if it's sorted.
        }
    }

    /// Returns a pair of indices that are currently being compared or swapped.
    ///
    /// # Returns
    /// A tuple `(usize, usize)` with MAX values since BogoSort doesn't work by comparing specific pairs at each step.
    fn special(&self) -> (usize, usize) {
        (usize::MAX, usize::MAX) // No specific elements are actively being compared.
    }

    /// Returns the current reason for the action ("Comparing" or "Switching").
    ///
    /// # Returns
    /// The `Reasons` enum indicating the current operation.
    fn reason(&self) -> Reasons {
        self.reason // Returns whether we're comparing or switching elements.
    }

    /// Executes a single step of the BogoSort algorithm.
    ///
    /// # Arguments
    /// * `array` - A mutable reference to the array being sorted.
    ///
    /// # Returns
    /// * `true` if sorting is complete.
    /// * `false` if sorting is still in progress.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.is_sorted {
            return true; // If the array is sorted, return true to indicate that sorting is complete.
        }

        // Check if the array is sorted.
        self.is_sorted = self.is_sorted_check(array);

        // If the array is not sorted, shuffle it to attempt to sort it randomly.
        if !self.is_sorted {
            array.shuffle(&mut rand::thread_rng()); // Shuffle the array randomly.
            self.reason = Reasons::Switching; // Indicate that elements have been shuffled (switched).
            play_beep();
        } else {
            self.reason = Reasons::Comparing; // If sorted, set the reason to "Comparing" (though no comparisons are needed).
        }

        false // Sorting is not complete yet, so return false.
    }

    /// Resets the state of BogoSort for a fresh sort, making the array unsorted again.
    fn reset_state(&mut self) {
        self.is_sorted = false; // Reset the sorting state to unsorted.
        self.reason = Reasons::Comparing; // Reset the reason to "Comparing".
    }

    /// Returns whether the sorting process is complete.
    ///
    /// # Returns
    /// `true` if sorting is finished, otherwise `false`.
    fn is_finished(&self) -> bool {
        self.is_sorted
    }
}
