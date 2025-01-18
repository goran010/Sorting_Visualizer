use super::{Reasons, Sorter};

pub struct InsertionSort {
    current_index: usize, // Tracks the current index in the array that we're processing.
    sorted_index: usize,  // Tracks the index of the last sorted element.
    reason: Reasons,      // Tracks the reason for the current action, either comparing or inserting.
    is_sorted: bool,      // Tracks whether the sorting is complete.
}

impl Sorter for InsertionSort {
    // Creates a new instance of InsertionSort with initial values.
    fn new() -> Self {
        InsertionSort {
            current_index: 1,  // Start from the second element in the array.
            sorted_index: 0,   // The first element is initially considered sorted.
            reason: Reasons::Comparing, // Initially, the action is comparing.
            is_sorted: false,  // Initially, the sorting is not complete.
        }
    }

    // Returns the indices that are currently being compared or moved.
    fn special(&self) -> (usize, usize) {
        (self.current_index, self.sorted_index)  // These are the indices of the current and sorted elements.
    }

    // Returns the reason for the current operation (Comparing or Inserting).
    fn reason(&self) -> Reasons {
        self.reason  // Will return either "Comparing" or "Inserting" depending on the current action.
    }

    // Executes a single step of the InsertionSort algorithm.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // If the current index reaches the end of the array, the sorting is complete.
        if self.current_index >= array.len() {
            self.is_sorted = true;  // Mark the sorting as complete.
            return true;  // Indicate that sorting is complete.
        }

        let mut i = self.current_index; // Start with the current index.

        // Compare and insert the element into the sorted portion of the array.
        while i > 0 && array[i] < array[i - 1] {
            array.swap(i, i - 1);  // Swap the elements to maintain sorted order.
            i -= 1;  // Move to the previous element in the sorted portion.

            // After a swap, the reason is updated to "Comparing".
            self.reason = Reasons::Comparing;
        }

        // Move to the next element in the array for the next iteration.
        self.current_index += 1;

        // After inserting the element in its correct position, update the reason.
        self.reason = Reasons::Switching;

        false  // Sorting isn't complete yet, so return false.
    }

    // Resets the state of the InsertionSort algorithm, setting the current index to 1.
    fn reset_state(&mut self) {
        self.current_index = 1;  // Start again from the second element of the array.
        self.is_sorted = false;  // Reset the sorted state.
    }

    // Returns whether the sorting process is complete.
    fn is_finished(&self) -> bool {
        self.is_sorted
    }
}
