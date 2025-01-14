use super::{Reasons, Sorter};

pub struct SelectionSort {
    current_index: usize, // Tracks the current index in the array that we're processing.
    min_index: usize,     // Tracks the index of the minimum value found during the current pass.
    reason: Reasons,      // Tracks the reason for the current action, either comparing or switching.
}

impl Sorter for SelectionSort {
    // Creates a new instance of SelectionSort with initial values.
    fn new() -> Self {
        SelectionSort {
            current_index: 0,  // Start from the first element in the array.
            min_index: 0,      // The first element is initially assumed to be the smallest.
            reason: Reasons::Comparing, // Initially, the action is comparing.
        }
    }

    // Returns the indices that are currently being compared or swapped.
    fn special(&self) -> (usize, usize) {
        (self.current_index, self.min_index)  // These are the indices of the current element and the minimum element.
    }

    // Returns the reason for the current operation (Comparing or Switching).
    fn reason(&self) -> Reasons {
        self.reason  // Will return either "Comparing" or "Switching" depending on the current action.
    }

    // Executes a single step of the SelectionSort algorithm.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // If the current index is beyond the last element, the sorting is complete.
        if self.current_index >= array.len() {
            return true;  // Sorting is complete.
        }

        // Assume the current element is the smallest in the remaining unsorted portion.
        self.min_index = self.current_index;

        // Iterate through the unsorted portion of the array to find the smallest element.
        for j in (self.current_index + 1)..array.len() {
            if array[j] < array[self.min_index] {  // If a smaller element is found...
                self.min_index = j;  // Update the index of the smallest element.
            }
        }

        // Swap the current element with the smallest element found in the remaining portion.
        array.swap(self.current_index, self.min_index);

        // Move to the next element in the array for the next iteration.
        self.current_index += 1;

        // After a swap, the reason is updated to "Switching" to indicate that a swap happened.
        self.reason = Reasons::Switching;

        // Add a delay to slow down the sorting process (useful for visualization).
        self.add_delay();

        false  // Sorting isn't complete yet, so return false.
    }

    // Resets the state of the SelectionSort algorithm, setting the current index to 0.
    fn reset_state(&mut self) {
        self.current_index = 0;  // Start again from the beginning of the array.
    }
}
