mod buttons;
pub mod constants;
use strum::IntoEnumIterator;

use self::constants::{CEIL, FLOOR, VECTOR_SIZE};
use crate::algorithms::{
    bogo_sort::BogoSort, bubble_sort::BubbleSort, heap_sort::HeapSort, insertion_sort::InsertionSort,
    merge_sort::MergeSort, quick_sort::QuickSort, selection_sort::SelectionSort, Reasons, Sorter,
};
use crate::util;
use buttons::ButtonHandler;
use eframe::{
    egui::{self, Button, CentralPanel, ComboBox, Grid, Sense, Ui},
    epaint::{vec2, Color32, Stroke, Vec2},
};
use std::{thread, time::Instant};
use crate::types::{Algorithms, State, BAR_HEIGHT_MULTIPLIER, BAR_WIDTH, STEP_DELAY, BASELINE};


/// Main structure managing the visualizer's state, data, and behavior.
pub(crate) struct Visualizer<'a> {
    selected_algorithm: Algorithms, // The currently selected sorting algorithm.
    numbers: Vec<usize>, // The array of numbers being sorted.
    original_numbers: Vec<usize>, // A copy of the original unsorted array.
    state: State, // The current state of the visualizer (Start, Running, Finished).
    sorter: Box<dyn Sorter + 'a>, // The sorting algorithm instance.
    start_time: Option<Instant>, // Timer tracking the start of sorting.
    total_elapsed_time: f64, // Total elapsed time of the sorting process.
}

impl<'a> Default for Visualizer<'a> {
    /// Creates a default instance of the visualizer with the Bubble Sort algorithm.
    fn default() -> Self {
        let numbers = util::gen_random_vector(FLOOR, CEIL, VECTOR_SIZE);
        Self {
            selected_algorithm: Algorithms::Bubble,
            numbers: numbers.clone(),
            original_numbers: numbers,
            state: State::Start,
            sorter: Box::new(BubbleSort::new()),
            start_time: None,
            total_elapsed_time: 0.0,
        }
    }
}

impl Visualizer<'_> {
    /// Creates a new instance of the visualizer.
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self { Self::default() }

    /// Draws the bars representing the current state of the array.
    fn draw_bars(&self, ui: &mut Ui) {
        ui.horizontal_top(|ui| {
            self.numbers.iter().enumerate().for_each(|(index, &value)| {
                let color = self.get_bar_color(index);
                Self::draw_bar_helper(vec2(BAR_WIDTH, BASELINE - (value * BAR_HEIGHT_MULTIPLIER) as f32), color, ui);
            });
        });
    }

    /// Determines the color of a bar based on the sorting state and indices.
    fn get_bar_color(&self, index: usize) -> Color32 {
        if self.state != State::Finished && (index == self.sorter.special().0 || index == self.sorter.special().1) {
            match self.sorter.reason() { Reasons::Comparing => Color32::LIGHT_YELLOW, Reasons::Switching => Color32::LIGHT_GREEN }
        } else { Color32::GRAY }
    }

    /// Helper function to draw a single bar.
    fn draw_bar_helper(size: Vec2, color: Color32, ui: &mut Ui) {
        Grid::new("numbers").spacing(vec2(1.0, 1.0)).show(ui, |ui| {
            ui.vertical(|ui| {
                let mut rect = ui.allocate_exact_size(size, Sense::hover()).0;
                rect.set_top(size.y);
                rect.set_bottom(BASELINE);
                ui.painter().rect(rect, 2.0, color, Stroke::NONE);
            });
        });
    }

    /// Handles the selection of a sorting algorithm from the dropdown menu.
    fn handle_algorithm_selection(&mut self, ui: &mut Ui) -> bool {
        let previous = self.selected_algorithm;
        ui.label("Algorithm:");
        ComboBox::from_id_source(0)
            .selected_text(format!("{:?} Sort", self.selected_algorithm))
            .show_ui(ui, |ui| {
                Algorithms::iter().for_each(|alg| {
                    ui.selectable_value(&mut self.selected_algorithm, alg, format!("{:?} Sort", alg));
                });
            });
        if previous != self.selected_algorithm { self.switch_algorithm(); true } else { false }
    }
     /// Switches the current sorting algorithm and resets the visualizer.
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
        self.reset();
    }

    /// Creates the control buttons for the visualizer (Start, Step, Reset, Shuffle).
    fn create_control_buttons(&mut self, ui: &mut Ui) {
        if self.state == State::Running {
            if ui.add(Button::new("Stop")).clicked() { self.state = State::Start; }
            ui.add_enabled(false, Button::new("Step"));
        } else {
            if ui.add(Button::new("Start")).clicked() {
                self.state = State::Running;
                self.start_time = Some(Instant::now());
            }
            if ui.add(Button::new("Step")).clicked() { ButtonHandler::handle_step(self); }
        }
        if ui.add(Button::new("Reset")).clicked() { ButtonHandler::handle_reset(self); }
        if ui.add(Button::new("Shuffle")).clicked() { ButtonHandler::handle_shuffle(self); }
    }

    /// Handles continuous sorting steps while in the "Running" state.
    fn handle_running(&mut self) {
        if self.state == State::Running {
            thread::sleep(STEP_DELAY);
            ButtonHandler::handle_step(self);
            if self.sorter.is_finished() { self.state = State::Finished; }
        }
    }
    /// Resets the visualizer state and timer.
    fn reset(&mut self) {
        self.state = State::Start;
        self.sorter.reset_state();
        self.start_time = None;
        self.total_elapsed_time = 0.0;
    }
}
/// Updates the UI and handles events in the visualizer.
impl eframe::App for Visualizer<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.state == State::Running {
            if let Some(start_time) = self.start_time {
                let now = Instant::now();
                self.total_elapsed_time += now.duration_since(start_time).as_secs_f64();
                self.start_time = Some(now);
            }
        }

        egui::TopBottomPanel::top("timer_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(format!("Elapsed Time: {:.2}s", self.total_elapsed_time));
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.handle_algorithm_selection(ui) { self.switch_algorithm(); }
                self.create_control_buttons(ui);
            });
            self.handle_running();
            self.draw_bars(ui);
        });
    }
}
