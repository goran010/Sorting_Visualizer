use std::time::Duration;
use strum_macros::EnumIter;

/// Enum representing the available sorting algorithms.
#[derive(PartialEq, Debug, EnumIter, Clone, Copy)]
pub enum Algorithms {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Bogo,
    Quick,
    Heap,
    Counting,
    Cocktail,
    Gnome,
    Pancake,
    Shell,
    Comb,
    OddEven,
}

/// Enum representing the state of the visualizer.
#[derive(PartialEq, Debug)]
pub enum State {
    Start,
    Running,
    Finished,
}

/// Constants for configuring the visualizer.
pub const STEP_DELAY: Duration = Duration::from_millis(10);
