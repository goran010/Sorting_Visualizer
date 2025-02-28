use super::{Reasons, Sorter};
use crate::sound::play_beep;

/// Represents the state of the Cocktail Shaker Sort algorithm.
pub struct CocktailSort {
    start: usize,
    end: usize,
    swapped: bool,
    forward: bool,
    finished: bool,
    current: usize,
    comparisons: usize,
    swaps: usize,
}

impl CocktailSort {
    pub fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            swapped: false,
            forward: true,
            finished: false,
            current: 0,
            comparisons: 0,
            swaps: 0,
        }
    }

    fn initialize(&mut self, array_len: usize) {
        if array_len <= 1 {
            self.finished = true;
        } else {
            self.start = 0;
            self.end = array_len - 1;
            self.swapped = false;
            self.forward = true;
            self.current = 0;
            self.finished = false;
        }
    }
}

impl Sorter for CocktailSort {
    fn new() -> Self {
        Self::new()
    }

    fn special(&self) -> (usize, usize) {
        if self.finished || self.start >= self.end {
            (usize::MAX, usize::MAX)
        } else if self.forward {
            (self.current, self.current + 1)
        } else {
            (self.current - 1, self.current)
        }
    }

    fn reason(&self) -> Reasons {
        if self.finished {
            Reasons::Comparing
        } else {
            Reasons::Switching
        }
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        // Early return for empty or single-element arrays
        if array.is_empty() || array.len() == 1 {
            self.finished = true;
            return true;
        }

        // Initialize if not yet done
        if self.end == 0 {
            self.initialize(array.len());
            return false;
        }

        // Check if we're done
        if self.finished || self.start >= self.end {
            self.finished = true;
            return true;
        }

        // Perform a single step of the sorting algorithm
        self.comparisons += 1;

        if self.forward {
            // Forward pass
            if self.current < self.end {
                if array[self.current] > array[self.current + 1] {
                    array.swap(self.current, self.current + 1);
                    play_beep();
                    self.swaps += 1;
                    self.swapped = true;
                }

                self.current += 1;

                // If we reached the end of the current forward pass
                if self.current == self.end {
                    self.end -= 1;
                    self.forward = false;
                    self.current = self.end;
                }
            }
        } else {
            // Backward pass
            if self.current > self.start {
                if array[self.current - 1] > array[self.current] {
                    array.swap(self.current - 1, self.current);
                    play_beep();
                    self.swaps += 1;
                    self.swapped = true;
                }

                self.current -= 1;

                // If we reached the start of the current backward pass
                if self.current == self.start {
                    self.start += 1;

                    // Check if we didn't swap anything in the complete pass
                    if !self.swapped {
                        self.finished = true;
                        return true;
                    }

                    // Start a new forward pass and reset swapped flag
                    self.forward = true;
                    self.swapped = false;
                    self.current = self.start;
                }
            }
        }

        // Check if we're done
        if self.start >= self.end {
            self.finished = true;
        }

        self.finished
    }

    fn reset_state(&mut self) {
        *self = Self::new();
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn comparisons(&self) -> usize {
        self.comparisons
    }

    fn swaps(&self) -> usize {
        self.swaps
    }
}
