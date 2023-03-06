use std::io;
use std::io::BufReader;
use std::time::Duration;

use proconio::source::line::LineSource;

use intro_heuristics::solver::{crossline, greedy};
use intro_heuristics::visualizer;
use intro_heuristics::State;

#[allow(non_snake_case)]

fn main() {
    env_logger::init();
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));

    proconio::input! {
        from &mut stdin,
        n: usize,
        pos: [(usize, usize); n],
    }
    log::info!("N: {}", n);

    let mut state = State::new(n, pos);
    visualizer::draw(&state, 0, Duration::from_secs(2));

    greedy::solve(&mut state);
    visualizer::draw(&state, 0, Duration::from_secs(2));

    crossline::solve(&mut state);
    visualizer::draw(&state, 0, Duration::from_secs(1));

    println!(
        "{}",
        state
            .points
            .iter()
            .map(|f| f.id.get().to_string())
            .collect::<Vec<_>>()
            .join("\n"),
    );
}
