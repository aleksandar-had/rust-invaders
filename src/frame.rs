use crate::{NUM_COLS, NUM_ROWS};

// create a type alias due to complex type Vec<Vec<...
// the lifetime is 'static!!! We want the data to live for the entire
// lifetime of the running program as the main frame
pub type Frame = [[char; NUM_ROWS]; NUM_COLS];

pub fn new_frame() -> Frame {
    [[' '; NUM_ROWS]; NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
