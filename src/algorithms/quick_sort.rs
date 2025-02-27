use super::{Reasons, Sorter};
use crate::sound::play_beep;

/// Represents the QuickSort algorithm and its state.
pub struct QuickSort {
    partition_stack: Vec<(usize, usize)>, // Stack to track the partitions (low, high)
    reason: Reasons, // Reason for the current operation (Comparing or Switching)
    swaps: usize,    // Indicates if the sorting is finished.
    comparisons: usize,
}

impl QuickSort {
    /// Partitions the array and returns the pivot index.
    /// This function selects a pivot and places it at the correct sorted position in the array.
    /// # Arguments
    /// * `array` - The array slice to partition.
    /// * `low` - The starting index of the partition.
    /// * `high` - The ending index of the partition.
    /// # Returns
    /// The index of the pivot after partitioning.
    fn partition(&mut self, array: &mut [usize], low: usize, high: usize) -> usize {
        let pivot = array[high]; // Choose the pivot element (usually the last element in the range)
        let mut i = low;

        // Loop over the array, comparing each element with the pivot.
        for j in low..high {
            self.reason = Reasons::Comparing; // Indicate comparing step
            if array[j] <= pivot {
                array.swap(i, j); // Swap elements that are less than or equal to the pivot
                i += 1;
            }
        }

        // After processing, swap the pivot to its correct position (i-th index)
        array.swap(i, high);
        self.reason = Reasons::Switching; // Indicate switching after pivot placement
        play_beep();
        self.swaps += 1;
        i // Return the pivot index
    }
}

impl Sorter for QuickSort {
    /// Creates a new instance of QuickSort.
    fn new() -> Self {
        QuickSort {
            partition_stack: Vec::new(), // Initialize an empty stack to manage partitions
            reason: Reasons::Comparing,  // The initial reason is "Comparing"
            swaps: 0,                    // Indicates if the sorting is finished.
            comparisons: 0,
        }
    }

    /// Returns the special indices currently being compared (low, high).
    /// # Returns
    /// A tuple `(low, high)` representing the current partition's bounds.
    fn special(&self) -> (usize, usize) {
        if let Some(&(low, high)) = self.partition_stack.last() {
            (low, high) // Return the last partition's low and high indices
        } else {
            (usize::MAX, usize::MAX) // Return MAX values if no partitions are present
        }
    }

    /// Returns the reason for the current sorting action.
    ///
    /// # Returns
    /// The `Reasons` enum indicating the current operation (Comparing or Switching).
    fn reason(&self) -> Reasons {
        self.reason
    }

    /// Executes a single step of the QuickSort algorithm.
    /// # Arguments
    /// * `array` - A mutable reference to the array being sorted.
    /// # Returns
    /// * `true` if sorting is complete.
    /// * `false` if sorting is still in progress.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // Initialize the stack with the first partition (the entire array)
        if self.partition_stack.is_empty() {
            if array.is_empty() {
                return true; // Handle edge case: empty array
            }
            self.partition_stack.push((0, array.len() - 1)); // The initial partition
        }

        // Process the partitions (perform partitioning and recursive sorting)
        while let Some((low, high)) = self.partition_stack.pop() {
            if low < high {
                // Partition the array and get the pivot index
                let pivot = self.partition(array, low, high);

                // After partitioning, push the left and right partitions to the stack
                if pivot > low {
                    self.partition_stack.push((low, pivot - 1)); // Left partition (elements before the pivot)
                }
                if pivot + 1 < high {
                    self.partition_stack.push((pivot + 1, high)); // Right partition (elements after the pivot)
                }

                // Continue sorting the left and right parts
                return false; // Sorting is not complete yet, so return false
            }
        }

        // If no more partitions to process, sorting is complete
        true
    }

    /// Resets the state of the QuickSort instance for a fresh sort.
    /// Clears the stack and resets the reason to "Comparing".
    fn reset_state(&mut self) {
        *self = Self::new(); // Reset all fields to their initial state.
    }

    /// Checks if the QuickSort process is finished.
    /// # Returns `true` if sorting is finished, otherwise `false`.
    fn is_finished(&self) -> bool {
        self.partition_stack.is_empty() // Sorting is finished if the stack is empty
    }
    fn comparisons(&self) -> usize {
        self.comparisons
    }

    fn swaps(&self) -> usize {
        self.swaps
    }
}
