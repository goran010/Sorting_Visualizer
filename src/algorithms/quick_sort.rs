use super::{Reasons, Sorter};

pub struct QuickSort {
    partition_stack: Vec<(usize, usize)>, // Stack to track the partitions (low, high)
    reason: Reasons,                      // Reason for the current operation (Comparing or Switching)
}

impl QuickSort {
    /// Partitions the array and returns the pivot index.
    /// This function selects a pivot and places it at the correct sorted position in the array.
    fn partition(&mut self, array: &mut [usize], low: usize, high: usize) -> usize {
        let pivot = array[high];  // Choose the pivot element (usually the last element in the range)
        let mut i = low;

        // Loop over the array, comparing each element with the pivot.
        for j in low..high {
            self.reason = Reasons::Comparing; // Indicate comparing step
            if array[j] <= pivot {
                array.swap(i, j);  // Swap elements that are less than or equal to the pivot
                i += 1;
            }
        }

        // After processing, swap the pivot to its correct position (i-th index)
        array.swap(i, high);
        self.reason = Reasons::Switching; // Indicate switching after pivot placement
        i // Return the pivot index
    }
}

impl Sorter for QuickSort {
    /// Creates a new instance of QuickSort.
    fn new() -> Self {
        QuickSort {
            partition_stack: Vec::new(), // Initialize an empty stack to manage partitions
            reason: Reasons::Comparing,  // The initial reason is "Comparing"
        }
    }

    /// Returns the special indices currently being compared (low, high).
    fn special(&self) -> (usize, usize) {
        if let Some(&(low, high)) = self.partition_stack.last() {
            (low, high) // Return the last partition's low and high indices
        } else {
            (usize::MAX, usize::MAX) // Return MAX values if no partitions are present
        }
    }

    /// Returns the reason for the current sorting action (either "Comparing" or "Switching").
    fn reason(&self) -> Reasons {
        self.reason
    }

    /// Executes a single step of the QuickSort algorithm.
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

                // Add delay to slow down the sorting process (useful for visualization)
                   

                // Continue sorting the left and right parts
                return false; // Sorting is not complete yet, so return false
            }
        }

        // If no more partitions to process, sorting is complete
        true
    }

    /// Resets the state of the QuickSort instance for a fresh sort.
    fn reset_state(&mut self) {
        self.partition_stack.clear(); // Clear the stack
        self.reason = Reasons::Comparing; // Reset the reason to "Comparing"
    }
}
