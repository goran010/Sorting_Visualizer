use super::{Reasons, Sorter};

pub struct MergeSort {
    partition_stack: Vec<(usize, usize, usize)>, // Stack for managing partitions
    temp: Vec<usize>,                           // Temporary array for merging
    reason: Reasons,                            // Reason for the current action
}

impl MergeSort {
    /// Merges two sorted halves of the array.
    fn merge(&mut self, array: &mut [usize], left: usize, mid: usize, right: usize) {
        let mut left_idx = left;
        let mut right_idx = mid + 1;
        let mut temp_idx = left;

        // Copy the current range to the temporary array
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

        // Copy any remaining elements from the left half
        while left_idx <= mid {
            array[temp_idx] = self.temp[left_idx];
            left_idx += 1;
            temp_idx += 1;
        }

        // Remaining elements in the right half are already in place
    }
}

impl Sorter for MergeSort {
    fn new() -> Self {
        MergeSort {
            partition_stack: vec![],
            temp: Vec::new(),
            reason: Reasons::Comparing,
        }
    }

    fn special(&self) -> (usize, usize) {
        (usize::MAX, usize::MAX)
    }

    fn reason(&self) -> Reasons {
        self.reason
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.temp.is_empty() {
            self.temp = array.clone();
            self.partition_stack.push((0, array.len() - 1, 0));
        }

        if let Some((left, right, phase)) = self.partition_stack.pop() {
            if phase == 0 {
                if left < right {
                    let mid = (left + right) / 2;
                    self.partition_stack.push((left, right, 1)); // Done with partitioning
                    self.partition_stack.push((mid + 1, right, 0)); // Sort the right half
                    self.partition_stack.push((left, mid, 0)); // Sort the left half
                }
            } else if phase == 1 {
                let mid = (left + right) / 2;
                self.merge(array, left, mid, right);
                self.reason = Reasons::Switching; // Indicate switching after merge
            }
        } else {
            return true; // Sorting is complete
        }

        false
    }

    fn reset_state(&mut self) {
        self.partition_stack.clear();
        self.temp.clear();
        self.reason = Reasons::Comparing;
    }

    fn switch(&mut self, _: &mut Vec<usize>) {}

    fn modify_state(&mut self, _: &[usize]) -> bool {
        false
    }
}

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

    fn modify_state(&mut self, _: &[usize]) -> bool {
        false // Not used in BubbleSort.
    }

    fn switch(&mut self, _: &mut Vec<usize>) {
        // Swaps are handled directly in `step`.
    }
}
