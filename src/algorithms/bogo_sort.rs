use rand::prelude::SliceRandom; // Add this line
use super::{Sorter, Reasons};

pub struct BogoSort {
    is_sorted: bool,
    reason: Reasons,
}

impl BogoSort {
    fn is_sorted_check(&self, array: &[usize]) -> bool {
        array.windows(2).all(|w| w[0] <= w[1])
    }
}

impl Sorter for BogoSort {
    fn new() -> Self {
        BogoSort {
            is_sorted: false,
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
        if self.is_sorted {
            return true;
        }
        self.is_sorted = self.is_sorted_check(array);
        if !self.is_sorted {
            array.shuffle(&mut rand::thread_rng()); // Now works because of the import
            self.reason = Reasons::Switching;
        } else {
            self.reason = Reasons::Comparing;
        }
        self.add_delay();
        false
    }

    fn reset_state(&mut self) {
        self.is_sorted = false;
        self.reason = Reasons::Comparing;
    }

      

     
}
