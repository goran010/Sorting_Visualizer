use super::{Sorter, Reasons};

pub struct BubbleSort {
    pass: usize,             // Tracks the current pass through the array.
    index: Option<usize>,    // Tracks the current index being compared, wrapped in an Option.
    needs_switch: bool,      // Indicates if a swap is needed.
    action_reason: Reasons,  // Tracks the reason for the current action.
}

impl Sorter for BubbleSort {
    fn new() -> Self {
        BubbleSort {
            pass: 0,
            index: None,
            needs_switch: false,
            action_reason: Reasons::Comparing,
        }
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        let len = array.len();

        // Check if the sorting is complete.
        if self.pass == len - 1 {
            return true; // Sorting is complete.
        }

        // Determine the current index or initialize it.
        if let Some(idx) = self.index {
            if idx < len - self.pass - 1 {
                self.index = Some(idx + 1); // Move to the next pair.
            } else {
                self.pass += 1; // Move to the next pass.
                self.index = Some(0); // Reset index for the next pass.
            }
        } else {
            self.index = Some(0); // Initialize index for the first step.
        }

        // Perform the comparison and determine if a swap is needed.
        if let Some(idx) = self.index {
            if idx + 1 < len {
                self.needs_switch = array[idx] > array[idx + 1];
                self.action_reason = if self.needs_switch {
                    Reasons::Switching
                } else {
                    Reasons::Comparing
                };

                // Perform the swap if necessary.
                if self.needs_switch {
                    array.swap(idx, idx + 1);
                    self.needs_switch = false; // Reset the flag after the swap.
                }
            }
        }
        self.add_delay();
        false // Continue sorting.
    }

    fn reset_state(&mut self) {
        *self = Self::new();
    }

    fn special(&self) -> (usize, usize) {
        if let Some(idx) = self.index {
            (idx, idx + 1)
        } else {
            (usize::MAX, usize::MAX)
        }
    }

    fn reason(&self) -> Reasons {
        self.action_reason
    }

   

     
}
