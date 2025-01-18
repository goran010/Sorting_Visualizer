use super::{Reasons, Sorter};

/// Represents the HeapSort algorithm and its state.
pub struct HeapSort {
    index: usize,    // Tracks the current position being sorted.
    root: usize,     // Tracks the root of the current subtree.
    reason: Reasons, // Reason for the current sorting action (Comparing or Switching).
}

impl HeapSort {
    /// Performs the sift-down operation to maintain the heap property.
    /// This operation ensures that the subtree rooted at `root` is a valid max-heap.
    /// 
    /// # Arguments
    /// * `array` - A mutable slice representing the array.
    /// * `end_index` - The index of the last element in the current heap.
    /// 
    /// # Returns
    /// `true` if the sift-down operation is complete, `false` if further sifting is required.
    fn sift_down(&mut self, array: &mut [usize], end_index: usize) -> bool {
        let mut child = self.root * 2 + 1; // Left child of the current root.

        // If there are no children, the subtree is already a valid heap.
        if child > end_index {
            return true;
        }

        // Check if the right child exists and is larger than the left child.
        if child + 1 <= end_index && array[child] < array[child + 1] {
            child += 1; // Right child is larger, so we choose it.
        }

        // If the root is smaller than the larger of its children, swap them.
        if array[self.root] < array[child] {
            array.swap(self.root, child); // Swap the root with the larger child.
            self.root = child; // Update the root to the new child.
            self.reason = Reasons::Switching; // Indicate that a swap occurred.
            return false; // Continue sifting down.
        }

        // If the heap property is maintained, we stop sifting down.
        true
    }
}

impl Sorter for HeapSort {
    /// Initializes a new `HeapSort` instance.
    fn new() -> Self {
        HeapSort {
            index: usize::MAX, // Initially, the index is not set.
            root: usize::MAX,  // Initially, there is no root node.
            reason: Reasons::Comparing, // Initially, we're comparing elements.
        }
    }

    /// Returns the special indices involved in the current operation.
    /// 
    /// # Returns
    /// A tuple `(root, usize::MAX)` where `root` is the current root being sifted.
    fn special(&self) -> (usize, usize) {
        (self.root, usize::MAX)
    }

    /// Returns the reason for the current sorting action (either `Comparing` or `Switching`).
    /// 
    /// # Returns
    /// The `Reasons` enum indicating the current operation.
    fn reason(&self) -> Reasons {
        self.reason
    }

    /// Performs one step of the HeapSort algorithm.
    /// 
    /// # Arguments
    /// * `array` - A mutable reference to the array being sorted.
    /// 
    /// # Returns
    /// * `true` if sorting is complete.
    /// * `false` if sorting is still in progress.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.index == usize::MAX {
            // First step: Build the heap.
            self.index = array.len() - 1;  // Start from the last element.
            self.root = (self.index + 1) / 2 - 1; // The last non-leaf node.
        }

        if self.root != usize::MAX {
            // Perform sift-down during heap construction.
            if self.sift_down(array, self.index) {
                // If the root has been sifted down correctly, move to the next root.
                if self.root == 0 {
                    self.root = usize::MAX; // Finished building the heap.
                } else {
                    self.root -= 1; // Move to the parent node.
                }
            }
        } else {
            // After building the heap, start the sorting process by extracting the root.
            if self.index == 0 {
                return true; // Sorting is complete.
            }

            // Swap the root with the last unsorted element (this moves the largest element to the end).
            array.swap(0, self.index);
            self.index -= 1; // Decrease the heap size.
            self.root = 0;   // Start sifting down the new root.
        }

        false // Continue sorting until the heap is fully sorted.
    }

    /// Resets the state of the HeapSort instance.
    fn reset_state(&mut self) {
        self.index = usize::MAX; // Reset index to an invalid value.
        self.root = usize::MAX;  // Reset root to an invalid value.
        self.reason = Reasons::Comparing; // Reset reason to comparing.
    }

    /// Checks if the sorting process is finished.
    /// 
    /// # Returns
    /// `true` if sorting is finished, otherwise `false`.
    fn is_finished(&self) -> bool {
        self.index == 0 // Sorting is finished when the index reaches 0.
    }
}
