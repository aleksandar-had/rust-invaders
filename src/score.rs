use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
};

pub struct Score {
    chars: Vec<char>,
}

impl Score {
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }

    pub fn max_ammo(invaders: &Invaders) -> usize {
        let score = invaders.total_count - invaders.army.len();
        if score < 10 {
            1
        } else if score < 25 {
            2
        } else {
            3
        }
    }

    pub fn update(&mut self, invaders: &Invaders) {
        let score = invaders.total_count - invaders.army.len();
        // format our score string
        let formatted = format!(
            "SCORE: {:0>3}\nMAX AMMO: {:0>3}",
            score,
            Self::max_ammo(invaders)
        );

        // clear the old score vector
        self.chars.clear();

        // copy chars from formatted string into the char vector
        for c in formatted.chars() {
            self.chars.push(c);
        }
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        // iterate over all characters
        for (i, c) in self.chars.iter().enumerate() {
            // put them in the first row
            frame[i][0] = *c;
        }
    }
}
