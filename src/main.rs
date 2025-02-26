use eframe::{epaint::Vec2, run_native, NativeOptions};

mod algorithms;
mod ui;
mod util;
mod types; 
mod sound;

/// The entry point for the sorting algorithm visualizer application.
fn main() {
    // Define the native options for the application window.
    let native_options = NativeOptions {
        initial_window_size: Some(Vec2::new(1530., 750.)), // Set the initial window size.
        ..Default::default() // Use default options for other settings.
    };

    // Run the application using the `Visualizer` defined in the `ui` module.
    run_native(
        "Sorting Visualizer", // The title of the application window.
        native_options, // The native options defined above.
        Box::new(|cc| Box::new(ui::Visualizer::new(cc))), // Create a new Visualizer instance.
    )
    .unwrap(); // Unwrap any errors that occur during the application run.
}
