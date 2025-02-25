use super::{Sorter, Reasons};
use crate::sound::play_beep; 

/// A struct representing the MergeSort algorithm.
/// It maintains the state of the sorting process, including subarray size, index, and a temporary array for merging.
pub struct MergeSort {
    size: usize,          // Current size of the subarrays being merged.
    index: usize,         // Index tracking the current position in the array.
    temp: Vec<usize>,     // Temporary array used during merging.
    reason: Reasons,      // Current action reason (Comparing or Switching).
    is_sorted: bool,      // Indicates whether the sorting process is complete.
}

impl MergeSort {
    /// Merges two sorted subarrays into a single sorted subarray.
    ///
    /// # Arguments
    /// * `array` - The array containing the subarrays to be merged.
    /// * `start` - The starting index of the first subarray.
    /// * `mid` - The ending index of the first subarray.
    /// * `end` - The ending index of the second subarray.
    fn merge(&mut self, array: &mut [usize], start: usize, mid: usize, end: usize) {
        let mut left = start;
        let mut right = mid + 1;
        let mut temp_idx = start;

        // Merge elements from both halves into the temporary array.
        while left <= mid && right <= end {
            if array[left] <= array[right] {
                self.temp[temp_idx] = array[left];
                left += 1;
            } else {
                self.temp[temp_idx] = array[right];
                right += 1;
            }
            temp_idx += 1;
        }

        // Copy remaining elements from the left subarray, if any.
        while left <= mid {
            self.temp[temp_idx] = array[left];
            left += 1;
            temp_idx += 1;
        }

        // Copy remaining elements from the right subarray, if any.
        while right <= end {
            self.temp[temp_idx] = array[right];
            right += 1;
            temp_idx += 1;
        }

        // Copy merged elements back into the original array.
        for i in start..=end {
            array[i] = self.temp[i];
        }
    }
}

impl Sorter for MergeSort {
    /// Creates a new instance of MergeSort with initial settings.
    fn new() -> Self {
        MergeSort {
            size: 1,                  // Start merging subarrays of size 1.
            index: 0,                 // Initialize index tracker.
            temp: Vec::new(),         // Temporary array will be initialized during sorting.
            reason: Reasons::Comparing, // Initial action reason is Comparing.
            is_sorted: false,         // Sorting is not complete initially.
        }
    }

    /// Executes a single step of the MergeSort algorithm.
    /// Returns `true` if sorting is complete, otherwise `false`.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.is_sorted {
            return true; // Stop if sorting is already complete.
        }

        let n = array.len();

        // Initialize the temporary array during the first step.
        if self.temp.is_empty() {
            self.temp = vec![0; n];
        }

        // If the current pass is complete, move to the next size of subarrays.
        if self.index >= n {
            self.size *= 2;
            self.index = 0;

            // Mark sorting as complete if the size exceeds the array length.
            if self.size >= n {
                self.is_sorted = true;
                return true;
            }
        }

        // Identify the subarrays to be merged.
        let start = self.index;
        let mid = (start + self.size - 1).min(n - 1);
        let end = (start + 2 * self.size - 1).min(n - 1);

        // Merge the subarrays if they exist.
        if mid < end {
            self.merge(array, start, mid, end);
            self.reason = Reasons::Switching; // Indicate that elements were switched.
            play_beep();
        }

        self.index += 2 * self.size; // Move to the next pair of subarrays.
        false
    }

    /// Resets the state of the MergeSort instance for a new sorting process.
    fn reset_state(&mut self) {
        self.size = 1;                 // Reset the size to its initial value.
        self.index = 0;                // Reset the index tracker.
        self.temp.clear();             // Clear the temporary array.
        self.reason = Reasons::Comparing; // Reset the action reason.
        self.is_sorted = false;        // Sorting is no longer marked as complete.
    }

    /// Returns the range of indices currently being processed.
    fn special(&self) -> (usize, usize) {
        let n = self.temp.len();
        let start = self.index.min(n);
        let end = (start + self.size).min(n);
        (start, end) // Highlight the current range being merged.
    }

    /// Returns the reason for the current sorting action (Comparing or Switching).
    fn reason(&self) -> Reasons {
        self.reason
    }

    /// Checks if the sorting process is complete.
    fn is_finished(&self) -> bool {
        self.is_sorted
    }
}
