use super::{Sorter, Reasons};

/// Represents the CountingSort algorithm and its state.
pub struct CountingSort {
    counts: Vec<usize>, // Array to count occurrences of each number.
    output: Vec<usize>, // Partially sorted array.
    max_value: usize,   // Maximum value in the array.
    current_value: usize, // Current value being placed.
    output_index: usize,  // Position in output array.
    reason: Reasons,    // Current action reason.
    is_sorted: bool,    // Indicates if sorting is complete.
    step_phase: usize,  // Tracks which phase of the algorithm is running.
    array_index: usize, // Tracks the index being modified in real-time.
    processing_index: usize, // Tracks which element is being processed.
}

impl Sorter for CountingSort {
    /// Creates a new CountingSort instance.
    fn new() -> Self {
        CountingSort {
            counts: Vec::new(),
            output: Vec::new(),
            max_value: 0,
            current_value: 0,
            output_index: 0,
            reason: Reasons::Comparing,
            is_sorted: false,
            step_phase: 0,
            array_index: 0,
            processing_index: 0,
        }
    }

    /// Executes a single step of CountingSort, ensuring every change is shown.
    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.is_sorted {
            return true;
        }

        match self.step_phase {
            0 => {
                // Step 1: Initialize count array
                self.max_value = *array.iter().max().unwrap_or(&0);
                self.counts = vec![0; self.max_value + 1];
                self.output = vec![0; array.len()];
                self.processing_index = 0;
                self.step_phase = 1;
            }
            1 => {
                // Step 2: Count occurrences (step-by-step)
                if self.processing_index < array.len() {
                    let num = array[self.processing_index];
                    self.counts[num] += 1;
                    self.reason = Reasons::Comparing;
                    self.processing_index += 1;
                    return false;
                }
                self.processing_index = 0;
                self.current_value = 0;
                self.step_phase = 2;
            }
            2 => {
                // Step 3: Build sorted output (step-by-step)
                if self.current_value <= self.max_value {
                    if self.counts[self.current_value] > 0 {
                        array[self.array_index] = self.current_value;
                        self.counts[self.current_value] -= 1;
                        self.reason = Reasons::Switching;
                        self.array_index += 1;
                        return false;
                    } else {
                        self.current_value += 1;
                    }
                } else {
                    self.step_phase = 3;
                }
            }
            3 => {
                // Step 4: Sorting complete
                self.is_sorted = true;
            }
            _ => {
                return true;
            }
        }
        false
    }

    fn reset_state(&mut self) {
        self.counts.clear();
        self.output.clear();
        self.max_value = 0;
        self.current_value = 0;
        self.output_index = 0;
        self.array_index = 0;
        self.processing_index = 0;
        self.is_sorted = false;
        self.step_phase = 0;
    }

    fn is_finished(&self) -> bool {
        self.is_sorted
    }

    /// Returns the special indices currently being processed.
    fn special(&self) -> (usize, usize) {
        match self.step_phase {
            1 => (self.processing_index, self.processing_index), // Counting phase
            2 => (self.array_index, self.array_index), // Placing phase
            _ => (usize::MAX, usize::MAX),
        }
    }

    /// Returns the reason for the current sorting action.
    fn reason(&self) -> Reasons {
        self.reason
    }
}
