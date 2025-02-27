use super::{Reasons, Sorter};
use crate::sound::play_beep;

pub struct PancakeSort {
    curr_size: usize,
    finished: bool,
}

impl PancakeSort {
    pub fn new() -> Self {
        Self {
            curr_size: 0, // Will be initialized later
            finished: false,
        }
    }

    fn flip(arr: &mut Vec<usize>, k: usize) {
        arr[..=k].reverse();
    }

    fn find_max(arr: &[usize], n: usize) -> usize {
        (0..n).max_by_key(|&i| arr[i]).unwrap_or(0)
    }
}

impl Sorter for PancakeSort {
    fn new() -> Self {
        Self::new()
    }

    fn special(&self) -> (usize, usize) {
        if self.finished {
            (usize::MAX, usize::MAX)
        } else {
            (self.curr_size, 0) // Showing the current subarray being processed
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

        if self.curr_size == 0 {
            self.curr_size = array.len();
        }

        if self.curr_size <= 1 {
            self.finished = true;
            return true;
        }

        play_beep(); // Beep sound for visualization

        let max_index = Self::find_max(array, self.curr_size);

        if max_index != self.curr_size - 1 {
            if max_index > 0 {
                Self::flip(array, max_index);
            }
            Self::flip(array, self.curr_size - 1);
        }

        self.curr_size -= 1;
        false
    }

    fn reset_state(&mut self) {
        self.curr_size = 0;
        self.finished = false;
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}
