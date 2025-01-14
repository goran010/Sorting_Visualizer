use super::{Reasons, Sorter};

pub struct HeapSort {
    index: usize,    // Tracks the current position being sorted
    root: usize,     // Tracks the root of the current subtree
    reason: Reasons, // Reason for the current sorting action
}

impl HeapSort {
    /// Performs the sift-down operation to maintain the heap property.
    fn sift_down(&mut self, array: &mut [usize], end_index: usize) -> bool {
        let mut child = self.root * 2 + 1; // Left child

        if child > end_index {
            return true; // No children to sift down
        }

        // Find the larger child
        if child + 1 <= end_index && array[child] < array[child + 1] {
            child += 1; // Right child is larger
        }

        // Swap if the child is larger than the root
        if array[self.root] < array[child] {
            array.swap(self.root, child);
            self.root = child; // Update root to the swapped child
            self.reason = Reasons::Switching;
            self.add_delay();
            return false; // Continue sifting down
        }

        true // Heap property is maintained
    }
}

impl Sorter for HeapSort {
    /// Initializes a new `HeapSort` instance.
    fn new() -> Self {
        HeapSort {
            index: usize::MAX,
            root: usize::MAX,
            reason: Reasons::Comparing,
        }
    }

    /// Returns the special indices involved in the current operation.
    fn special(&self) -> (usize, usize) {
        (self.root, usize::MAX)
    }

    /// Returns the reason for the current sorting action.
    fn reason(&self) -> Reasons {
        self.reason
    }

    /// Performs one step of the HeapSort algorithm.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.index == usize::MAX {
            // Build the heap
            self.index = array.len() - 1;
            self.root = (self.index + 1) / 2 - 1; // Start at the last non-leaf node
        }

        if self.root != usize::MAX {
            // Sift down during heap construction
            if self.sift_down(array, self.index) {
                if self.root == 0 {
                    self.root = usize::MAX;
                } else {
                    self.root -= 1;
                }
            }
        } else {
            // Perform sorting by extracting the root
            if self.index == 0 {
                return true; // Sorting is complete
            }

            array.swap(0, self.index); // Move the max element to the end
            self.index -= 1;           // Reduce the heap size
            self.root = 0;             // Sift down the new root
        }

        false
    }

    /// Resets the state of the HeapSort instance.
    fn reset_state(&mut self) {
        self.index = usize::MAX;
        self.root = usize::MAX;
        self.reason = Reasons::Comparing;
    }
}
