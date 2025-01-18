use super::{Sorter, Reasons};

/// Represents the BubbleSort algorithm and its state.
pub struct BubbleSort {
    pass: usize,             // Tracks the current pass through the array.
    index: Option<usize>,    // Tracks the current index being compared, wrapped in an Option.
    needs_switch: bool,      // Indicates if a swap is needed between two elements.
    action_reason: Reasons,  // Tracks the reason for the current action (Comparing or Switching).
    finished: bool,          // Indicates if the sorting is finished.
}

impl Sorter for BubbleSort {
    /// Initializes a new instance of BubbleSort with initial values.
    fn new() -> Self {
        BubbleSort {
            pass: 0,               // Start with the first pass.
            index: None,           // No index is set initially.
            needs_switch: false,   // No swap needed initially.
            action_reason: Reasons::Comparing, // The action is "Comparing" initially.
            finished: false,       // Sorting is not finished initially.
        }
    }

    /// Executes a single step of the BubbleSort algorithm.
    /// 
    /// # Arguments
    /// * `array` - A mutable reference to the array being sorted.
    /// 
    /// # Returns
    /// * `true` if sorting is complete.
    /// * `false` if sorting is still in progress.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        let len = array.len();

        // Check if the sorting is complete.
        if self.pass == len - 1 {
            self.finished = true; // Mark the sorting as finished.
            return true;          // Sorting is complete.
        }

        // Determine the current index or initialize it if it's the first time.
        if let Some(idx) = self.index {
            if idx < len - self.pass - 1 {
                self.index = Some(idx + 1); // Move to the next pair of elements.
            } else {
                self.pass += 1; // Move to the next pass (next iteration).
                self.index = Some(0); // Reset index for the next pass.
            }
        } else {
            self.index = Some(0); // Initialize index to 0 for the first pass.
        }

        // Perform the comparison and determine if a swap is needed.
        if let Some(idx) = self.index {
            if idx + 1 < len {
                self.needs_switch = array[idx] > array[idx + 1]; // Check if the elements need to be swapped.
                self.action_reason = if self.needs_switch {
                    Reasons::Switching  // If a swap is needed, set action to Switching.
                } else {
                    Reasons::Comparing  // If no swap is needed, set action to Comparing.
                };

                // Perform the swap if necessary.
                if self.needs_switch {
                    array.swap(idx, idx + 1); // Swap the elements at `idx` and `idx + 1`.
                    self.needs_switch = false; // Reset the flag after the swap.
                }
            }
        }

        false // Continue sorting by returning false (not complete yet).
    }

    /// Resets the state of BubbleSort, allowing the sorting to start fresh.
    fn reset_state(&mut self) {
        *self = Self::new(); // Reset all fields to their initial state.
    }

    /// Returns the current indices of the elements being compared or swapped.
    /// 
    /// # Returns
    /// A tuple `(current_index, next_index)` representing the indices of the elements being compared.
    fn special(&self) -> (usize, usize) {
        if let Some(idx) = self.index {
            (idx, idx + 1) // Return the current index and the next index being compared.
        } else {
            (usize::MAX, usize::MAX) // If no index is set, return MAX values.
        }
    }

    /// Returns the current reason for the sorting action.
    /// 
    /// # Returns
    /// The `Reasons` enum indicating the current operation (Comparing or Switching).
    fn reason(&self) -> Reasons {
        self.action_reason // Return the current action reason.
    }

    /// Returns whether the sorting process is complete.
    /// 
    /// # Returns
    /// `true` if sorting is finished, otherwise `false`.
    fn is_finished(&self) -> bool {
        self.finished
    }
}
