use super::{Sorter, Reasons};
use crate::sound::play_beep;

pub struct ShellSort {
    gap: usize,
    i: usize,
    finished: bool,
}

impl ShellSort {
    pub fn new() -> Self {
        Self {
            gap: 0, 
            i: 0,
            finished: false,
        }
    }
}

impl Sorter for ShellSort {
    fn new() -> Self {
        Self::new()
    }

    fn special(&self) -> (usize, usize) {
        if self.finished {
            (usize::MAX, usize::MAX)
        } else {
            (self.i, self.i + self.gap.min(1)) // Highlights elements being compared
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
        if self.finished {
            return true;
        }

        // Initialize gap on first run
        if self.gap == 0 {
            self.gap = array.len() / 2;
            self.i = self.gap;
        }

        if self.gap > 0 {
            if self.i < array.len() {
                play_beep(); // Play sound for visualization

                let temp = array[self.i];
                let mut j = self.i;

                // Perform insertion sort within the gap
                while j >= self.gap && array[j - self.gap] > temp {
                    array[j] = array[j - self.gap];
                    j -= self.gap;
                }

                array[j] = temp;
                self.i += 1; // Move to next element

            } else {
                // Reduce gap after finishing a full pass
                self.gap /= 2;
                self.i = self.gap;

                // If gap reaches 0, sorting is finished
                if self.gap == 0 {
                    self.finished = true;
                }
            }
        } else {
            self.finished = true;
        }

        false
    }

    fn reset_state(&mut self) {
        self.gap = 0;
        self.i = 0;
        self.finished = false;
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}
