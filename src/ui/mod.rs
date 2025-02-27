mod buttons;
pub mod constants;
use rfd::FileDialog;
use std::fs;
use strum::IntoEnumIterator;

use self::constants::{CEIL, FLOOR, Theme, VECTOR_SIZE};
use crate::algorithms::{
    Reasons, Sorter, bogo_sort::BogoSort, bubble_sort::BubbleSort, counting_sort::CountingSort,
    heap_sort::HeapSort, insertion_sort::InsertionSort, merge_sort::MergeSort,
    quick_sort::QuickSort, selection_sort::SelectionSort, cocktail_sort::CocktailSort,
    gnome_sort::GnomeSort, pancake_sort::PancakeSort, shell_sort::ShellSort,
    comb_sort::CombSort, odd_even_sort::OddEvenSort,
};
use crate::types::{Algorithms, STEP_DELAY, State};
use crate::util;
use buttons::ButtonHandler;
use eframe::{
    egui::{self, Button, ComboBox, Ui},
    epaint::{Color32, vec2},
};
use std::{thread, time::Instant};

/// Main structure managing the visualizer's state, data, and behavior.
pub(crate) struct Visualizer<'a> {
    selected_algorithm: Algorithms, // The currently selected sorting algorithm.
    numbers: Vec<usize>,            // The array of numbers being sorted.
    original_numbers: Vec<usize>,   // A copy of the original unsorted array.
    state: State, // The current state of the visualizer (Start, Running, Finished).
    sorter: Box<dyn Sorter + 'a>, // The sorting algorithm instance.
    start_time: Option<Instant>, // Timer tracking the start of sorting.
    total_elapsed_time: f64, // Total elapsed time of the sorting process.
    selected_theme: Theme, // The currently selected theme.
    user_input: String,
}

impl<'a> Default for Visualizer<'a> {
    /// Creates a default instance of the visualizer with the Bubble Sort algorithm and dark theme.
    fn default() -> Self {
        let numbers = util::gen_random_vector(FLOOR, CEIL, VECTOR_SIZE);
        let numbers_string = numbers
            .iter()
            .map(|n| n.to_string()) // Convert numbers to strings
            .collect::<Vec<_>>() // Collect into a vector
            .join(","); // Join into a single comma-separated string
        Self {
            selected_algorithm: Algorithms::Bubble,
            numbers: numbers.clone(),
            original_numbers: numbers,
            state: State::Start,
            sorter: Box::new(BubbleSort::new()),
            start_time: None,
            total_elapsed_time: 0.0,
            selected_theme: Theme::Dark, // Default theme is dark
            user_input: numbers_string,
        }
    }
}

impl Visualizer<'_> {
    /// Creates a new instance of the visualizer.
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Draws the bars representing the current state of the array.
    fn draw_bars(&self, ui: &mut Ui) {
        let window_width = ui.available_width();
        let window_height = ui.available_height();
        let num_bars = self.numbers.len().max(1); // Prevent division by zero
        let spacing = 5.0; // Reduce space between bars for better fit
        let total_spacing = spacing * (num_bars - 1) as f32;
        let bar_width = ((window_width - total_spacing) / num_bars as f32).max(2.0); // Ensure minimum width

        let max_value = *self.numbers.iter().max().unwrap_or(&1); // Get max value to scale height

        let top_ui_height = 150.0; // Reduced estimation for UI height
        let graph_height = (window_height - top_ui_height).max(200.0); // Maximize graph usage

        let painter = ui.painter();

        for (index, &value) in self.numbers.iter().enumerate() {
            let x = index as f32 * (bar_width + spacing) + 5.0;
            let height = ((value as f32 / max_value as f32) * graph_height).max(10.0); // Ensure bars have a minimum height
            let y = window_height - height + 105.0; // ✅ Bars align properly at bottom

            let color = self.get_bar_color(index);
            let rect = egui::Rect::from_min_size(egui::pos2(x, y), vec2(bar_width, height));

            painter.rect_filled(rect, 4.0, color); // **Rounded top corners**
        }
    }

    /// Determines the color of a bar based on the sorting state and indices.
    fn get_bar_color(&self, index: usize) -> Color32 {
        if self.state != State::Finished
            && (index == self.sorter.special().0 || index == self.sorter.special().1)
        {
            match self.sorter.reason() {
                Reasons::Comparing => Color32::LIGHT_YELLOW,
                Reasons::Switching => Color32::LIGHT_GREEN,
            }
        } else {
            self.selected_theme.bar_color() // Bar color based on the selected theme
        }
    }

    /// Handles the selection of a sorting algorithm from the dropdown menu.
    fn handle_algorithm_selection(&mut self, ui: &mut Ui) -> bool {
        let previous = self.selected_algorithm;
        ui.label("Algorithm:");
        ComboBox::from_id_source("algorithm_selector")
            .selected_text(format!("{:?} Sort", self.selected_algorithm))
            .show_ui(ui, |ui| {
                Algorithms::iter().for_each(|alg| {
                    ui.selectable_value(
                        &mut self.selected_algorithm,
                        alg,
                        format!("{:?} Sort", alg),
                    );
                });
            });
        if previous != self.selected_algorithm {
            self.switch_algorithm();
            true
        } else {
            false
        }
    }

    /// Handles the selection of a theme from the dropdown menu.
    fn handle_theme_selection(&mut self, ui: &mut Ui) {
        ui.label("Theme:");
        ComboBox::from_id_source("theme_selector")
            .selected_text(format!("{:?}", self.selected_theme))
            .show_ui(ui, |ui| {
                for theme in [
                    Theme::Dark,
                    Theme::Light,
                    Theme::Summer,
                    Theme::Autumn,
                    Theme::Winter,
                    Theme::Spring,
                ] {
                    ui.selectable_value(&mut self.selected_theme, theme, format!("{:?}", theme));
                }
            });
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
            Algorithms::Counting => Box::new(CountingSort::new()),
            Algorithms::Cocktail => Box::new(CocktailSort::new()),
            Algorithms::Gnome => Box::new(GnomeSort::new()),
            Algorithms::Pancake => Box::new(PancakeSort::new()),
            Algorithms::Shell => Box::new(ShellSort::new()),
            Algorithms::Comb => Box::new(CombSort::new()),
            Algorithms::OddEven => Box::new(OddEvenSort::new()),
        };
        self.reset();
    }

    /// Creates the control buttons for the visualizer (Start, Step, Reset, Shuffle).
    fn create_control_buttons(&mut self, ui: &mut Ui) {
        if self.state == State::Running {
            if ui.add(Button::new("Stop")).clicked() {
                self.state = State::Start;
            }
            ui.add_enabled(false, Button::new("Step"));
        } else {
            if ui.add(Button::new("Start")).clicked() {
                self.state = State::Running;
                self.start_time = Some(Instant::now());
            }
            if ui.add(Button::new("Step")).clicked() {
                ButtonHandler::handle_step(self);
            }
        }
        if ui.add(Button::new("Reset")).clicked() {
            ButtonHandler::handle_reset(self);
        }
        if ui.add(Button::new("Shuffle")).clicked() {
            ButtonHandler::handle_shuffle(self);
        }
    }

    /// Handles continuous sorting steps while in the "Running" state.
    fn handle_running(&mut self) {
        if self.state == State::Running {
            thread::sleep(STEP_DELAY);
            ButtonHandler::handle_step(self);

            // Update the elapsed time and check if the sorting is finished
            if let Some(start) = self.start_time {
                self.total_elapsed_time = start.elapsed().as_secs_f64(); // Time in seconds
            }

            if self.sorter.is_finished() {
                self.state = State::Finished;
            }
        }
    }

    fn process_user_input(&mut self) {
        // 🔹 Parse user input: Split by commas, trim spaces, convert to numbers
        let new_numbers: Vec<usize> = self
            .user_input
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();

        if !new_numbers.is_empty() {
            self.numbers = new_numbers.clone();
            self.original_numbers = new_numbers;
            self.state = State::Start; // Reset state
        }
    }
    /// Opens a file dialog, reads numbers from a file, and updates the numbers field.
    fn load_numbers_from_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Text Files", &["txt"]) // Add filter for text files
            .pick_file()
        {
            if let Ok(contents) = fs::read_to_string(&path) {
                let new_numbers: Vec<usize> = contents
                    .split(',')
                    .filter_map(|s| s.trim().parse::<usize>().ok())
                    .collect();
    
                if !new_numbers.is_empty() {
                    self.numbers = new_numbers.clone();
                    self.original_numbers = new_numbers;
                    self.user_input = self
                        .numbers
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                }
            }
        }
    }

    fn load_numbers_from_csv(&mut self) {
        if let Some(path) = FileDialog::new().add_filter("CSV Files", &["csv"]).pick_file() {
            if let Ok(contents) = fs::read_to_string(path) {
                let new_numbers: Vec<usize> = contents
                    .lines() // Split by lines
                    .flat_map(|line| line.split(','))
                    .filter_map(|s| s.trim().parse::<usize>().ok())
                    .collect();
    
                if !new_numbers.is_empty() {
                    self.numbers = new_numbers.clone();
                    self.original_numbers = new_numbers;
                    self.user_input = self
                        .numbers
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                }
            }
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

impl eframe::App for Visualizer<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint(); // UI refresh request

        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = self.selected_theme.background_color();
        ctx.set_style(style);

        // 🔹 Numbers input field moved to the top
        egui::TopBottomPanel::top("numbers_input").show(ctx, |ui| {
            ui.horizontal(|ui| { // Horizontal layout for input field and button
                ui.label("Numbers:");
                ui.add_sized(
                    [ui.available_width() - 60.0, 30.0], // Width and height of the input field
                    egui::TextEdit::singleline(&mut self.user_input),
                );
        
                // Enter button for user input
                if ui.button("Select").clicked() {
                    self.process_user_input(); // Process user input
                }
            });
        
            // User can press Enter to submit input
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.process_user_input();
            }
        });

        // 🔹 Sorting control panel at the top (below numbers input)
        egui::TopBottomPanel::top("timer_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    // 🔹 "Load from File" button next to elapsed time
                    if ui.button("📂 Load from txt").clicked() {
                        self.load_numbers_from_file();
                    }

                    if ui.button("📊 Load from CSV").clicked() {
                        self.load_numbers_from_csv();
                    }

                    ui.label(
                        egui::RichText::new(format!(
                            "Elapsed Time: {:.2}s",
                            self.total_elapsed_time
                        ))
                        .color(self.selected_theme.text_color()),
                    );
                });
            });
        });
        // 🔹 Main sorting UI and visualization
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.handle_algorithm_selection(ui) {
                    self.switch_algorithm();
                }
                self.handle_theme_selection(ui);
                self.create_control_buttons(ui);
            });

            self.handle_running();
            self.draw_bars(ui);
        });
    }
}
