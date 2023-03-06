use std::cmp;
use std::ops::RangeInclusive;

pub mod greedy {
    use crate::{visualizer, State};
    use std::time::Duration;

    pub fn solve(state: &mut State) {
        let n = state.n;
        for i in 1..n {
            let pi = i - 1;

            let mut min_i = i;
            let mut min = f64::MAX;
            for j in (i + 1).min(n)..n {
                let dist = state.points[pi].distance(&state.points[j]);
                if min > dist {
                    min_i = j;
                    min = dist;
                }
            }
            if i != min_i {
                state.points.swap(i, min_i);
                visualizer::draw(state, i, Duration::from_millis(20));
            }
        }
        log::info!(
            "greedy score: {}, elapsed: {:?}",
            state.compute_score(),
            state.elapsed()
        );
    }
}

pub mod crossline {
    use crate::{visualizer, Line, Point, State, Temperature};
    use rand::Rng;
    use std::mem;
    use std::time::Duration;

    fn find_cross_line(points: &[Point]) -> Option<(Line, Line)> {
        let n = points.len();
        for i in 0..n {
            let pi = (i + 1) % n;
            let line1 = Line::new(points[i].clone(), points[pi].clone());

            for j in (i + 2)..n {
                let pj = (j + 1) % n;
                let line2 = Line::new(points[j].clone(), points[pj].clone());

                if line1.cross(&line2) {
                    return Some((line1.clone(), line2));
                }
            }
        }
        None
    }

    pub fn solve(state: &mut State) {
        let mut map = state.compute_map();

        let mut temp = Temperature::new(2e3, 5.0, 2.0, state.timer.clone());
        let mut rng = rand::thread_rng();
        let mut score = state.compute_score();

        loop {
            let t = match temp.get_time() {
                None => break,
                Some(v) => v,
            };

            if let Some((line1, line2)) = find_cross_line(&state.points) {
                let mut t10 = *map.get(&line1.0.id).unwrap();
                let mut t11 = *map.get(&line1.1.id).unwrap();
                if t10 > t11 {
                    mem::swap(&mut t10, &mut t11);
                }

                let mut t20 = *map.get(&line2.0.id).unwrap();
                let mut t21 = *map.get(&line2.1.id).unwrap();
                if t20 > t21 {
                    mem::swap(&mut t20, &mut t21);
                }

                let mut t1 = t11;
                let mut t2 = t20;

                if t10 == 0 && t11 == state.n - 1 {
                    t1 = 0;
                    t2 = t20;
                    log::info!("t1 both side: {} {} {} {}", t10, t11, t20, t21);
                }
                if t20 == 0 && t21 == state.n - 1 {
                    t1 = 0;
                    t2 = t10;
                    log::info!("t2 both side: {} {} {} {}", t10, t11, t20, t21,);
                }
                if t1 > t2 {
                    mem::swap(&mut t1, &mut t2);
                }

                // 交差をひっくり返す
                state.reverse_points(t1, t2);
                let new_score = state.compute_score();
                if new_score < score {
                    log::debug!(
                        "score updated: {} -> {}, diff: {}",
                        score,
                        new_score,
                        score - new_score
                    );
                    score = new_score;
                    map = state.compute_map();
                    visualizer::draw(&state, t1, Duration::from_millis(300));
                } else {
                    let p = f64::exp((score - new_score) as f64 / t);
                    log::debug!(
                        "戻す確率: 1 - p = exp(({} - {}) / {}) = {}",
                        score,
                        new_score,
                        t,
                        1.0 - p
                    );

                    if !rng.gen_bool(p) {
                        log::debug!("戻す");
                        state.reverse_points(t1, t2);
                    }
                }
            } else {
                break;
            }
        }
        log::info!("solve: {:?}", state.elapsed());
        log::info!("solved score: {}", state.compute_score());
    }
}

#[allow(dead_code)]
fn is_overlap<T: Ord + Copy>(x: RangeInclusive<T>, y: RangeInclusive<T>) -> bool {
    let x = if x.start() <= x.end() {
        x
    } else {
        *x.end()..=*x.start()
    };
    let y = if y.start() <= y.end() {
        y
    } else {
        *y.end()..=*y.start()
    };
    cmp::max(x.start(), y.start()) <= cmp::min(x.end(), y.end())
}

#[cfg(test)]
mod tests {
    use crate::solver::is_overlap;
    use crate::{Line, Point};
    use rand::Rng;

    #[test]
    fn overlap_test() {
        assert_eq!(is_overlap(1..=10, 11..=20), false);
        assert_eq!(is_overlap(1..=10, 20..=11), false);
        assert_eq!(is_overlap(10..=1, 11..=20), false);
        assert_eq!(is_overlap(10..=1, 20..=11), false);

        assert_eq!(is_overlap(1..=10, 10..=20), true);
        assert_eq!(is_overlap(1..=10, 20..=10), true);
        assert_eq!(is_overlap(10..=1, 10..=20), true);
        assert_eq!(is_overlap(10..=1, 20..=10), true);
    }

    #[test]
    fn cross_test() {
        let mut rng = rand::thread_rng();

        macro_rules! point {
            ($x:expr, $y:expr) => {
                Point::new(rng.gen_range(0, 100), $x, $y)
            };
        }
        macro_rules! line {
            ($a:expr, $b:expr) => {
                Line($a.clone(), $b.clone())
            };
        }

        let p00 = point!(0, 0);
        let p01 = point!(0, 1);
        let p10 = point!(1, 0);
        let p11 = point!(1, 1);

        let cross_func = |line1: Line, line2: Line| -> bool { line1.cross(&line2) };

        println!("{}", cross_func(line!(p00, p10), line!(p01, p11)));

        // x軸に平行
        assert_eq!(
            cross_func(line!(p00, p10), line!(p01, p11)),
            false,
            "parallel for x-axis"
        );
        // y軸に平行
        assert_eq!(
            cross_func(line!(p00, p01), line!(p10, p11)),
            false,
            "parallel for y-axis"
        );

        assert_eq!(cross_func(line!(p00, p11), line!(p01, p10)), true, "cross");

        assert_eq!(
            cross_func(line!(p00, p01), line!(p00, p10)),
            false,
            "same point"
        );
    }
}
