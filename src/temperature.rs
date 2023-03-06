use std::time::Instant;

pub struct Temperature {
    t0: f64,
    t1: f64,
    tl: f64,
    timer: Instant,

    t: f64,

    count: usize,
}

#[inline]
fn get_time(inst: &Instant) -> f64 {
    inst.elapsed().as_millis() as f64 / 1000.0
}

impl Temperature {
    pub fn new(t0: f64, t1: f64, tl: f64, timer: Instant) -> Self {
        log::debug!("T0 = {}, T1 = {}, TL = {}", t0, t1, tl);
        Self {
            t0,
            t1,
            tl,
            timer,
            t: t0,
            count: 0,
        }
    }

    pub fn get_time(&mut self) -> Option<f64> {
        self.count += 1;
        // 計算をサボる
        if self.count % 100 != 0 {
            return Some(self.t);
        }

        let t = get_time(&self.timer) / self.tl;
        if t >= 1.0 {
            return None;
        }
        self.t = self.t0.powf(1.0 - t) * self.t1.powf(t);
        Some(self.t)
    }
}
