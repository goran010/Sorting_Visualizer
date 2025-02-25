pub mod bogo_sort;
pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;
pub mod counting_sort;

/// Enum representing the reasons for sorting actions. 
/// # Variants
/// * `Comparing` - Indicates that elements are being compared.
/// * `Switching` - Indicates that elements are being swapped.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reasons {
    Comparing,
    Switching,
}

/// A trait representing the behavior of a sorting algorithm.
pub trait Sorter {
    /// Creates a new instance of the sorting algorithm.
    /// # Returns a new instance of the sorter.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the indices of elements involved in the current operation.
    /// 
    /// # Returns
    /// A tuple `(usize, usize)` representing the indices being compared or swapped.
    fn special(&self) -> (usize, usize);

    /// Returns the reason for the current sorting action.
    /// 
    /// # Returns
    /// The `Reasons` enum indicating the current operation.
    fn reason(&self) -> Reasons;

    /// Executes a single step of the sorting algorithm.
    /// 
    /// # Arguments
    /// * `array` - A mutable reference to the array being sorted.
    /// 
    /// # Returns
    /// * `true` if sorting is complete.
    /// * `false` if sorting is still in progress.
    fn step(&mut self, array: &mut Vec<usize>) -> bool;

    /// Resets the state of the sorter, allowing the sorting process to start fresh.
    fn reset_state(&mut self);

    /// Checks if the sorting process is finished.
    /// 
    /// # Returns
    /// `true` if sorting is complete, otherwise `false`.
    fn is_finished(&self) -> bool;
}
