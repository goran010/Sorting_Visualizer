mod buttons;
pub mod constants;

use self::constants::{CEIL, FLOOR, VECTOR_SIZE};
use crate::algorithms::{
    bogo_sort::BogoSort, bubble_sort::BubbleSort, heap_sort::HeapSort,
    insertion_sort::InsertionSort, merge_sort::MergeSort, quick_sort::QuickSort,
    selection_sort::SelectionSort, Reasons, Sorter,
};
use crate::util;
use buttons::ButtonHandler;
use eframe::{
    egui::{self, Button, CentralPanel, ComboBox, Grid, Sense, Ui},
    epaint::{vec2, Color32, Stroke, Vec2},
};
use std::{thread, time::{Duration, Instant}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Enum for selecting sorting algorithms.
#[derive(PartialEq, Debug, EnumIter, Clone, Copy)]
enum Algorithms {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Bogo,
    Quick,
    Heap,
}

/// Constants for UI configuration.
const BAR_HEIGHT_MULTIPLIER: usize = 32;
const BAR_WIDTH: f32 = 6.9;
const CORNER_ROUNDING: f32 = 2.0;
const GRID_ID: &str = "numbers";
const STEP_DELAY: Duration = Duration::from_millis(20);
const BASELINE: f32 = 700.0;

/// State of the visualizer.
#[derive(PartialEq, Debug)]
enum State {
    Start,
    Running,
    Finished,
}

/// The main struct managing the visualizer's state and behavior.
pub(crate) struct Visualizer<'a> {
    selected_algorithm: Algorithms,
    numbers: Vec<usize>,
    original_numbers: Vec<usize>,
    state: State,
    sorter: Box<dyn Sorter + 'a>,
    start_time: Option<Instant>,   // Timer field
    total_elapsed_time: f64,       // Total elapsed time
}


impl<'a> Default for Visualizer<'a> {
    fn default() -> Self {
        let numbers = util::gen_random_vector(FLOOR, CEIL, VECTOR_SIZE);
        Self {
            selected_algorithm: Algorithms::Bubble,
            numbers: numbers.clone(),
            original_numbers: numbers,
            state: State::Start,
            sorter: Box::new(BubbleSort::new()),
            start_time: None,              // Initialize as None
            total_elapsed_time: 0.0,       // Initialize total elapsed time
        }
    }
}

impl Visualizer<'_> {
    /// Creates a new Visualizer instance.
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Draws the bars representing the array values.
    fn draw_bars(&self, ui: &mut Ui) {
        let highlighted_indices = self.sorter.special();
        let operation_reason = self.sorter.reason();

        ui.horizontal_top(|ui| {
            for (index, &value) in self.numbers.iter().enumerate() {
                let height = (value * BAR_HEIGHT_MULTIPLIER) as f32;
                let size = vec2(BAR_WIDTH, BASELINE - height);
                let color = if self.state != State::Finished
                    && (index == highlighted_indices.0 || index == highlighted_indices.1)
                {
                    match operation_reason {
                        Reasons::Comparing => Color32::LIGHT_YELLOW,
                        Reasons::Switching => Color32::LIGHT_GREEN,
                    }
                } else {
                    Color32::GRAY
                };
                Self::draw_bar_helper(size, color, ui);
            }
        });
    }

    /// Helper function to draw a single bar with its label.
    fn draw_bar_helper(size: Vec2, color: Color32, ui: &mut Ui) {
        Grid::new(GRID_ID)
            .spacing(vec2(1.0, 1.0)) // Minimal spacing between grid cells
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Allocate space for the bar
                    let mut rect = ui.allocate_exact_size(size, Sense::hover()).0;
                    rect.set_top(size.y);
                    rect.set_bottom(BASELINE);

                    // Draw the bar
                    ui.painter().rect(rect, CORNER_ROUNDING, color, Stroke::NONE);
                });
            });
    }

    /// Handles the algorithm selection dropdown.
    fn handle_algorithm_selection(&mut self, ui: &mut Ui) -> bool {
    let previous_selection = self.selected_algorithm;
    ui.label("Algorithm:");
    ComboBox::from_id_source(0)
        .selected_text(format!("{:?} Sort", self.selected_algorithm))
        .show_ui(ui, |ui| {
            for algorithm in Algorithms::iter() {
                ui.selectable_value(
                    &mut self.selected_algorithm,
                    algorithm,
                    format!("{algorithm:?} Sort"),
                );
            }
        });
    if previous_selection != self.selected_algorithm {
        self.switch_algorithm(); // Switch and reset when the algorithm changes
        true
    } else {
        false
    }
}


    /// Switches the algorithm and resets the visualizer.
    fn switch_algorithm(&mut self) {
    self.sorter = match self.selected_algorithm {
        Algorithms::Bubble => Box::new(BubbleSort::new()),
        Algorithms::Selection => Box::new(SelectionSort::new()),
        Algorithms::Insertion => Box::new(InsertionSort::new()),
        Algorithms::Merge => Box::new(MergeSort::new()),
        Algorithms::Bogo => Box::new(BogoSort::new()),
        Algorithms::Heap => Box::new(HeapSort::new()),
        Algorithms::Quick => Box::new(QuickSort::new()),
    };
    self.reset();                 // Reset the visualizer
    self.total_elapsed_time = 0.0; // Reset the elapsed time
    self.start_time = None;        // Clear the timer
}


    /// Creates buttons for controlling the visualization.
    fn create_control_buttons(&mut self, ui: &mut Ui) {
        if self.state == State::Running {
            if ui.add(Button::new("Stop")).clicked() {
                self.state = State::Start;
            }
            ui.add_enabled(false, Button::new("Step"));
        } else {
            if ui.add(Button::new("Start")).clicked() {
                self.state = State::Running;
                self.start_time = Some(Instant::now()); // Reset the timer when sorting starts
            }
            if ui.add(Button::new("Step")).clicked() {
                ButtonHandler::handle_step(self);
            }
        }
        if ui.add(Button::new("Reset")).clicked() {
            ButtonHandler::handle_reset(self);
            self.start_time = None; // Clear the timer
        }
        if ui.add(Button::new("Shuffle")).clicked() {
            ButtonHandler::handle_shuffle(self);
        }
    }

    /// Handles continuous steps when in the running state.
   fn handle_running(&mut self) {
    if self.state == State::Running {
        if let Some(start_time) = self.start_time {
            let now = Instant::now();
            let elapsed_since_start = now.duration_since(start_time).as_secs_f64();
            self.total_elapsed_time += elapsed_since_start;
            self.start_time = Some(now); // Update the start_time
        }

        thread::sleep(STEP_DELAY);
        ButtonHandler::handle_step(self);

        if self.sorter.is_finished() {
            self.state = State::Finished; // Transition to Finished state
            self.start_time = None;       // Stop the timer
        }
    }
}




    /// Resets the visualizer to its initial state.
    fn reset(&mut self) {
    self.state = State::Start;
    self.sorter.reset_state();
    self.start_time = None;         // Clear the timer
    self.total_elapsed_time = 0.0;  // Reset elapsed time
}

}

impl eframe::App for Visualizer<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Accumulate elapsed time only if sorting is running
        if self.state == State::Running {
            if let Some(start_time) = self.start_time {
                let now = Instant::now();
                let elapsed_since_start = now.duration_since(start_time).as_secs_f64();
                self.total_elapsed_time += elapsed_since_start;
                self.start_time = Some(now);
            }
        }

        // Display total elapsed time
        egui::TopBottomPanel::top("timer_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(format!(
                        "Elapsed Time: {:.2}s",
                        self.total_elapsed_time
                    )); // Display elapsed time
                });
            });
        });

        // Main central panel for buttons and visualization
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.handle_algorithm_selection(ui) {
                    self.switch_algorithm();
                }
                self.create_control_buttons(ui);
            });

            self.handle_running();
            self.draw_bars(ui);
        });
    }
}


