use std::fmt::Display;

use crate::{
    color::{Color, PrimaryColor},
    cube_set::PrimaryColorSet,
};

pub struct Bank {
    colors: PrimaryColorSet,
    capacity_per_color: u32,
}

impl Bank {
    pub fn new(initial_count: u32) -> Self {
        Self {
            colors: PrimaryColorSet::with_initial_count(initial_count),
            capacity_per_color: initial_count,
        }
    }

    pub fn capacity_per_color(&self) -> u32 {
        self.capacity_per_color
    }

    pub fn remaining(&self, color: PrimaryColor) -> u32 {
        self.colors.capacity(color)
    }

    /// Returns true if the color was successfully withdrawn from the bank, false otherwise.
    pub fn maybe_withdraw(&mut self, color: Color) -> bool {
        if color
            .decompose_iter()
            .all(|(color, count)| self.colors.capacity(color) >= count)
        {
            for &color in color.decompose() {
                self.colors.remove(color).unwrap();
            }
            true
        } else {
            false
        }
    }

    pub fn deposit(&mut self, color: Color) {
        for &color in color.decompose() {
            self.colors.insert(color);
        }
    }
}

impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.colors)
    }
}
