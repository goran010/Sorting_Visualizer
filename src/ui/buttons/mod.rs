use super::constants::{CEIL, FLOOR, VECTOR_SIZE};
use crate::random::gen_random_vector;
use crate::ui;
use ui::*;

pub struct ButtonHandler;

impl ButtonHandler {
    /// If not finished, takes a single step within the selected algorithm.
    /// Else, resets the app state.
    pub(crate) fn handle_step(app: &mut Visualizer) {
        if app.state != State::Finished && app.sorter.step(&mut app.numbers) {
            app.state = State::Finished;
        }
        if app.state == State::Finished {
            app.reset();
        }
    }

    /// Resets `app` state and sets `numbers` to their initial state.
    pub(crate) fn handle_reset(app: &mut Visualizer) {
        app.reset();
        app.numbers = app.original_numbers.clone();
        app.comparisons = 0;
        app.swaps = 0;
    }

    /// Resets `app` state, generates new numbers, and updates the initial state.
    pub(crate) fn handle_shuffle(app: &mut Visualizer) {
        app.reset();
        app.numbers = gen_random_vector(FLOOR, CEIL, VECTOR_SIZE);
        app.comparisons = 0;
        app.swaps = 0;
        app.original_numbers = app.numbers.clone();
        app.user_input = app
            .numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
    }
}
