use super::{Reasons, Sorter};

pub struct MergeSort {
    partition_stack: Vec<(usize, usize, usize)>, // Stack for managing partitions (left, right, phase)
    temp: Vec<usize>,                           // Temporary array used for merging
    reason: Reasons,                            // Tracks the current action (Comparing or Switching)
}

impl MergeSort {
    /// Merges two sorted halves of the array.
    fn merge(&mut self, array: &mut [usize], left: usize, mid: usize, right: usize) {
        let mut left_idx = left;  // Index for the left half of the array
        let mut right_idx = mid + 1; // Index for the right half of the array
        let mut temp_idx = left;  // Index for the temporary array

        // Copy the current range of the array into the temporary array
        self.temp[left..=right].copy_from_slice(&array[left..=right]);

        // Merge the two halves into the original array
        while left_idx <= mid && right_idx <= right {
            self.reason = Reasons::Comparing; // Indicate comparing during merge
            if self.temp[left_idx] <= self.temp[right_idx] {
                array[temp_idx] = self.temp[left_idx];
                left_idx += 1;
            } else {
                array[temp_idx] = self.temp[right_idx];
                right_idx += 1;
            }
            temp_idx += 1;
        }

        // Copy any remaining elements from the left half into the original array
        while left_idx <= mid {
            array[temp_idx] = self.temp[left_idx];
            left_idx += 1;
            temp_idx += 1;
        }

        // Remaining elements in the right half are already in place, so no action needed.
    }
}

impl Sorter for MergeSort {
    fn new() -> Self {
        MergeSort {
            partition_stack: vec![], // Initialize with an empty stack for partitions
            temp: Vec::new(),        // Initialize the temporary array as empty
            reason: Reasons::Comparing, // Set the initial reason to "Comparing"
        }
    }

    fn special(&self) -> (usize, usize) {
        // In MergeSort, there aren't specific elements being compared in each step,
        // so we return MAX values for the indices.
        (usize::MAX, usize::MAX)
    }

    fn reason(&self) -> Reasons {
        self.reason // Return the current reason for the action (Comparing or Switching)
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.temp.is_empty() {
            // If the temporary array is empty, initialize it with the current array
            self.temp = array.clone();
            // Push the initial partition (whole array) to the stack
            self.partition_stack.push((0, array.len() - 1, 0));
        }

        if let Some((left, right, phase)) = self.partition_stack.pop() {
            if phase == 0 {
                // Phase 0: Partition the array further by dividing it into two halves
                if left < right {
                    let mid = (left + right) / 2;
                    self.partition_stack.push((left, right, 1)); // Mark the partition as complete (phase 1)
                    self.partition_stack.push((mid + 1, right, 0)); // Sort the right half
                    self.partition_stack.push((left, mid, 0)); // Sort the left half
                }
            } else if phase == 1 {
                // Phase 1: Merge the two halves
                let mid = (left + right) / 2;
                self.merge(array, left, mid, right);
                self.reason = Reasons::Switching; // Indicate that switching occurred during merge
            }
        } else {
            return true; // Sorting is complete when the stack is empty
        }

        false // Continue sorting
    }

    fn reset_state(&mut self) {
        self.partition_stack.clear(); // Clear the stack
        self.temp.clear();             // Clear the temporary array
        self.reason = Reasons::Comparing; // Reset the reason to "Comparing"
    }
}
