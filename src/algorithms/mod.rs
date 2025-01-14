pub mod bogo_sort;
pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;
use std::{thread, time};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reasons {
    Comparing,
    Switching,
}

pub trait Sorter {
    fn new() -> Self
    where
        Self: Sized;

    fn special(&self) -> (usize, usize);
    fn reason(&self) -> Reasons;

   
    fn step(&mut self, array: &mut Vec<usize>) -> bool;

    fn reset_state(&mut self);

    // Function to add delay to slow down the sorting process
    fn add_delay(&self) {
        // Add a small delay of 200 milliseconds to make the sorting process visible
        let delay = time::Duration::from_millis(50);
        thread::sleep(delay);
    }
}

