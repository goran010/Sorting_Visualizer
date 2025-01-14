use super::{Reasons, Sorter};

pub struct InsertionSort {
    current_index: usize,  // Tracks the current index (i.e., the element to be inserted)
    comparison_index: usize, // Tracks the index of the element being compared with
    reason: Reasons,        // Tracks the reason for the current action (comparing or switching)
}

impl Sorter for InsertionSort {
    fn new() -> Self {
        InsertionSort {
            current_index: 1,
            comparison_index: 1,
            reason: Reasons::Comparing,
        }
    }

    fn special(&self) -> (usize, usize) {
        (self.current_index, self.comparison_index)
    }

    fn reason(&self) -> Reasons {
        self.reason
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // The sorting process starts with comparing and possibly shifting elements
        if self.comparison_index > 0 && array[self.comparison_index - 1] > array[self.comparison_index] {
            // If the current element is smaller than the previous one, swap them
            array.swap(self.comparison_index - 1, self.comparison_index);
            self.comparison_index -= 1;
            self.reason = Reasons::Switching;
            return false; // Continue sorting as more shifts might be needed
        }

        // If no more swaps are needed, move to the next element in the array
        self.current_index += 1;
        if self.current_index >= array.len() {
            return true; // Sorting is complete
        }

        // Set up for the next iteration
        self.comparison_index = self.current_index;
        self.reason = Reasons::Comparing;
        self.add_delay();
        false // Continue sorting
    }

    fn reset_state(&mut self) {
        self.current_index = 1;
        self.comparison_index = 1;
        self.reason = Reasons::Comparing;
    }
   
}
