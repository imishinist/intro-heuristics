pub mod solver;
pub mod visualizer;

mod data;
mod temperature;

pub use data::Id;
pub use data::Line;
pub use data::Point;
pub use temperature::*;

use std::collections::HashMap;
use std::time::{Duration, Instant};

fn index_map(points: &Vec<Point>) -> HashMap<Id, usize> {
    let start = Instant::now();
    let mut res = HashMap::new();
    for (idx, point) in points.iter().enumerate() {
        res.insert(point.id, idx);
    }
    log::debug!("index_map: {:?}", start.elapsed());
    res
}

fn compute_score(out: &Vec<Point>) -> f64 {
    let mut score = 0.0;
    let n = out.len();
    for i in 0..n {
        let ni = (i + 1) % n;

        score += out[i].distance(&out[ni]);
    }
    score
}

#[derive(Debug)]
pub struct State {
    pub n: usize,
    pub points: Vec<Point>,
    timer: Instant,
}

impl State {
    pub fn new(n: usize, points: Vec<(usize, usize)>) -> Self {
        let points = points
            .into_iter()
            .enumerate()
            .map(|(id, (x, y))| Point::new(id, x, y))
            .collect::<Vec<_>>();

        Self {
            n,
            points,
            timer: Instant::now(),
        }
    }

    pub fn get_timer(&self) -> Instant {
        self.timer
    }

    pub fn reset_timer(&mut self) {
        self.timer = Instant::now();
    }
    pub fn elapsed(&self) -> Duration {
        self.timer.elapsed()
    }

    pub fn reverse_points(&mut self, lower: usize, upper: usize) {
        (&mut self.points[lower..=upper]).reverse();
    }

    pub fn compute_score(&self) -> f64 {
        compute_score(&self.points)
    }

    pub fn compute_map(&self) -> HashMap<Id, usize> {
        index_map(&self.points)
    }
}
