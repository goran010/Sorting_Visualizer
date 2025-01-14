use super::{Reasons, Sorter};

pub struct InsertionSort {
    current_index: usize,  // Tracks the current index (i.e., the element to be inserted)
    comparison_index: usize, // Tracks the index of the element being compared with
    reason: Reasons,        // Tracks the reason for the current action (comparing or switching)
}

impl Sorter for InsertionSort {
    // Creates a new InsertionSort instance, initializing with the first unsorted element
    fn new() -> Self {
        InsertionSort {
            current_index: 1,  // Start comparing from the second element
            comparison_index: 1, // The first comparison happens between the second and first element
            reason: Reasons::Comparing, // Start with a comparison
        }
    }

    // Returns the indices of the current elements being compared.
    fn special(&self) -> (usize, usize) {
        (self.current_index, self.comparison_index)  // Indices of the current element and the one it's being compared with
    }

    // Returns the current reason for the sorting action (either "Comparing" or "Switching").
    fn reason(&self) -> Reasons {
        self.reason
    }

    // Performs a single step of the InsertionSort algorithm
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // The sorting process starts with comparing and possibly shifting elements
        if self.comparison_index > 0 && array[self.comparison_index - 1] > array[self.comparison_index] {
            // If the current element is smaller than the previous one, swap them
            array.swap(self.comparison_index - 1, self.comparison_index);
            self.comparison_index -= 1; // Move comparison index to the left (continue checking previous elements)
            self.reason = Reasons::Switching; // Set reason to "Switching" as we are swapping
            return false; // Continue sorting as more shifts might be needed
        }

        // If no more swaps are needed, move to the next element in the array
        self.current_index += 1;
        if self.current_index >= array.len() {
            return true; // Sorting is complete
        }

        // Set up for the next iteration: start comparing the next element in the array
        self.comparison_index = self.current_index; // Reset the comparison index to the current index
        self.reason = Reasons::Comparing; // We are now comparing the next pair
        self.add_delay();  // Optional: Add a small delay to slow down the sorting process for visualization
        false // Continue sorting
    }

    // Resets the state of the InsertionSort instance to start a fresh sorting process
    fn reset_state(&mut self) {
        self.current_index = 1;  // Start from the second element
        self.comparison_index = 1; // Compare the second and first element
        self.reason = Reasons::Comparing; // Start with a comparison
    }
}
