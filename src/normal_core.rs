use cms::*;
use core::{f64, hash::Hash};

pub struct NormalCore<T: Hash + Copy> {
    tick: u64,
    current: CMS<T>,
    all: CMS<T>,
}

impl<T: Hash + Copy> NormalCore<T> {
    pub fn new(tol: f64, p_err: f64, capacity: usize) -> NormalCore<T> {
        let c = CMS::new_with_probs(tol, p_err, capacity);
        let a = CMS::new_from_cms(&c);
        Self {
            tick: 0,
            current: c,
            all: a,
        }
    }

    fn score(a: f64, s: f64, t: f64) -> f64 {
        if t - 1. < 0.5 {
            return 0.;
        }
        (a - (s / t)).max(0.).powi(2) * (t.powi(2) / (s * (t - 1.)))
    }

    pub fn update(&mut self, item: T, time: u64) -> f64 {
        if time > self.tick {
            self.tick = time;
            self.current.clear();
        }

        let a: u64 = self.current.insert(item);
        let s: u64 = self.all.insert(item);

        NormalCore::<T>::score(a as f64, s as f64, time as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_event_anomaly() {
        let mut midas: NormalCore<(u64, u64)> = NormalCore::new(0.1, 0.001, 100);
        let mut max = 0.;
        for i in 0..1_000 {
            for _ in 1..10 {
                max = midas.update((10, 10), i).max(max);
            }
        }
        let anom = midas.update((11, 10), 3);
        println!("anom is {} and norm is {}", anom, max);
        assert!(anom > max);
    }
    #[test]
    fn one_extra_anomaly() {
        let mut midas: NormalCore<(u64, u64)> = NormalCore::new(0.1, 0.001, 100);
        let mut max = 0.;
        for i in 0..1_000 {
            for _ in 0..10 {
                max = midas.update((10, 10), i).max(max);
            }
        }
        let mut max_more = 0.;
        for _ in 0..11 {
            max_more = midas.update((10, 10), 1_000).max(max_more);
        }
        println!("cluster scored a {} and normal scored {}", max_more, max);
        assert!(max_more > max);
    }
    #[test]
    fn extra_vs_new_anomaly() {
        let mut midas: NormalCore<(u64, u64)> = NormalCore::new(0.1, 0.001, 100);
        for i in 0..1_000 {
            for _ in 0..10 {
                midas.update((10, 10), i);
            }
        }
        let mut max_more = 0.;
        for _ in 0..11 {
            max_more = midas.update((10, 10), 1_000).max(max_more);
        }
        let anom = midas.update((11, 10), 3);

        println!("cluster scored a {} and unique scored {}", max_more, anom);
        assert!(max_more < anom);
    }
}
