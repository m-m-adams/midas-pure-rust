use cms::*;
use core::{f64, hash::Hash};

struct NormalCore<T: Hash + Copy> {
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
        if (a < 0.5) || (t - 1. < 0.5) {
            return 0.;
        }

        (a - (s / t)).powi(2) * (t.powi(2) / (s * (t - 1.)))
    }

    pub fn update(&mut self, item: T, time: u64) -> f64 {
        let a: u64 = self.current.insert(item);
        let s: u64 = self.all.insert(item);
        if time > self.tick {
            self.tick = time;
            self.current.clear();
        }
        NormalCore::<T>::score(a as f64, s as f64, time as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut midas: NormalCore<(u64, u64)> = NormalCore::new(0.1, 0.001, 100);
        for i in 1..1_000_000 {
            println!("{}", midas.update((10, 10), i))
        }
        println!("{}", midas.update((11, 10), 3));
        assert_eq!(0, 1);
    }
}
