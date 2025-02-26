use strum_macros::EnumIter;
use std::time::Duration;

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
    Counting
}

/// Enum representing the state of the visualizer.
#[derive(PartialEq, Debug)]
pub enum State {
    Start,
    Running,
    Finished,
}

/// Constants for configuring the visualizer.
pub const BAR_HEIGHT_MULTIPLIER: usize = 28;
pub const BAR_WIDTH: f32 = 6.9;
pub const STEP_DELAY: Duration = Duration::from_millis(10);
pub const BASELINE: f32 = 720.0;
