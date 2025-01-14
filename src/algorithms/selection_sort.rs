use super::{Reasons, Sorter};

pub struct SelectionSort {
    current_index: usize,
    min_index: usize,
    reason: Reasons,
}

impl Sorter for SelectionSort {
    fn new() -> Self {
        SelectionSort {
            current_index: 0,
            min_index: 0,
            reason: Reasons::Comparing,
        }
    }

    fn special(&self) -> (usize, usize) {
        (self.current_index, self.min_index)
    }

    fn reason(&self) -> Reasons {
        self.reason
    }

    fn step(&mut self, array: &mut Vec<usize>) -> bool {
        if self.current_index >= array.len() {
            return true;
        }

        self.min_index = self.current_index;
        for j in (self.current_index + 1)..array.len() {
            if array[j] < array[self.min_index] {
                self.min_index = j;
            }
        }
        array.swap(self.current_index, self.min_index);
        self.current_index += 1;
        self.reason = Reasons::Switching;
        self.add_delay();
        false
    }

    fn reset_state(&mut self) {
        self.current_index = 0;
    }

    fn switch(&mut self, array: &mut Vec<usize>) {
        array.swap(self.current_index, self.min_index);
    }

    fn modify_state(&mut self, _: &[usize]) -> bool {
        false
    }
}
