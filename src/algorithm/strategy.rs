use crate::algorithm::problem::*;
use macroquad::prelude::*;

pub trait Strategy {
    fn path_finding(problem: &Problem) -> Vec<Vec2>;
}
