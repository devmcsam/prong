use bevy::prelude::*;
use crate::paddle::PaddleSide;

#[derive(Resource, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score {
    left: u16,
    right: u16,
}

impl Score {
    #[inline(always)]
    pub const fn new(left: u16, right: u16) -> Self {
        Self { left, right }
    }

    #[inline(always)]
    pub const fn get_left_score(&self) -> u16 {
        self.left
    }

    #[inline(always)]
    pub const fn get_right_score(&self) -> u16 {
        self.right
    }

    #[inline(always)]
    pub const fn increment(&mut self, side: PaddleSide) {
        match side {
            PaddleSide::Left => self.left += 1,
            PaddleSide::Right => self.right += 1,
        }
    }
}