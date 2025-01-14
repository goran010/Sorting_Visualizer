use super::{Reasons, Sorter};

pub struct QuickSort {
    partition_stack: Vec<(usize, usize)>, // Stack to track partitions
    reason: Reasons,                      // Reason for the current operation
}

impl QuickSort {
    /// Partitions the array and returns the pivot index.
    fn partition(&mut self, array: &mut [usize], low: usize, high: usize) -> usize {
        let pivot = array[high];  // Choose pivot element
        let mut i = low;

        for j in low..high {
            // Compare each element to the pivot
            self.reason = Reasons::Comparing; // Indicate comparing step
            if array[j] <= pivot {
                array.swap(i, j);  // Swap elements that are less than pivot
                i += 1;
            }
        }

        // Place the pivot at the correct position
        array.swap(i, high);
        self.reason = Reasons::Switching; // Indicate switching after pivot placement
        i
    }
}

impl Sorter for QuickSort {
    /// Creates a new QuickSort instance.
    fn new() -> Self {
        QuickSort {
            partition_stack: Vec::new(),
            reason: Reasons::Comparing,
        }
    }

    /// Returns the special indexes currently being compared.
    fn special(&self) -> (usize, usize) {
        if let Some(&(low, high)) = self.partition_stack.last() {
            (low, high)
        } else {
            (usize::MAX, usize::MAX)
        }
    }

    /// Returns the reason for the current operation.
    fn reason(&self) -> Reasons {
        self.reason
    }

    /// Executes a single step of the QuickSort algorithm.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.partition_stack.is_empty() {
            if array.is_empty() {
                return true; // Handle edge case: empty array
            }
            self.partition_stack.push((0, array.len() - 1)); // Initial partition
        }

        // Processing each partition
        while let Some((low, high)) = self.partition_stack.pop() {
            if low < high {
                let pivot = self.partition(array, low, high); // Partition the array
                
                // After partitioning, push the left and right partitions for further sorting
                if pivot > low {
                    self.partition_stack.push((low, pivot - 1)); // Left partition
                }
                if pivot + 1 < high {
                    self.partition_stack.push((pivot + 1, high)); // Right partition
                }
                self.add_delay();
                // Continue sorting the left and right parts
                return false; // Sorting is not complete yet
            }
        }
        
        return true; // Sorting is complete when the stack is empty
    }

    /// Resets the state of the QuickSort instance.
    fn reset_state(&mut self) {
        self.partition_stack.clear();
        self.reason = Reasons::Comparing;
    }
}
