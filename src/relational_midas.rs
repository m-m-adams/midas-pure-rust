use cms::*;
use core::{f64, hash::Hash};

pub struct RelationalCore<T: Hash + Copy> {
    tick: u64,
    current: NodeCMS<T>,
    all: NodeCMS<T>,
}
struct NodeCMS<T: Hash + Copy> {
    source: CMS<T>,
    dest: CMS<T>,
    combined: CMS<(T, T)>,
}
struct NodeScore {
    source: f64,
    dest: f64,
    combined: f64,
}

impl<T: Hash + Copy> NodeCMS<T> {
    fn new_with_probs(tol: f64, p_err: f64, capacity: usize) -> NodeCMS<T> {
        let source = CMS::new_with_probs(tol, p_err, capacity);
        let dest = CMS::new_with_probs(tol, p_err, capacity);
        let combined = CMS::new_with_probs(tol, p_err, capacity);
        Self {
            source,
            dest,
            combined,
        }
    }
    fn new_from_node(t: &NodeCMS<T>) -> NodeCMS<T> {
        let source = CMS::new_from_cms(&t.source);
        let dest = CMS::new_from_cms(&t.dest);
        let combined = CMS::new_from_cms(&t.combined);
        Self {
            source,
            dest,
            combined,
        }
    }
    fn insert(&mut self, node: (T, T)) -> NodeScore {
        let source = self.source.insert(node.0) as f64;
        let dest = self.dest.insert(node.1) as f64;
        let combined = self.combined.insert(node) as f64;
        NodeScore {
            source,
            dest,
            combined,
        }
    }
    fn clear(&mut self) {
        self.combined.clear();
        self.source.clear();
        self.dest.clear();
    }
}

impl<T: Hash + Copy> RelationalCore<T> {
    pub fn new(tol: f64, p_err: f64, capacity: usize) -> RelationalCore<T> {
        let current = NodeCMS::new_with_probs(tol, p_err, capacity);
        let all = NodeCMS::new_from_node(&current);
        Self {
            tick: 0,
            current,
            all,
        }
    }

    fn score(a: NodeScore, s: NodeScore, t: f64) -> f64 {
        if t - 1. < 0.5 {
            return 0.;
        }

        let source_score =
            (a.source - (s.source / t)).max(0.).powi(2) * (t / (s.source * (t - 1.)));
        let dest_score = (a.dest - (s.dest / t)).max(0.).powi(2) * (t / (s.dest * (t - 1.)));
        let comb_score =
            (a.combined - (s.combined / t)).max(0.).powi(2) * (t / (s.combined * (t - 1.)));
        //println!("user {}, ws {}, comb {}", s.source, s.dest, s.combined);

        (source_score + dest_score + comb_score) / 3.
    }

    pub fn update(&mut self, item: (T, T), time: u64) -> f64 {
        if time > self.tick {
            self.tick = time;
            self.current.clear();
        }
        let a: NodeScore = self.current.insert(item);
        let s: NodeScore = self.all.insert(item);

        RelationalCore::<T>::score(a, s, time as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_event_anomaly() {
        let mut midas: RelationalCore<u64> = RelationalCore::new(0.1, 0.001, 1_000);
        let mut max = 0.;
        for i in 0..100 {
            for s in 1..100 {
                for d in 11..200 {
                    max = midas.update((s, d), i).max(max);
                }
            }
        }

        let anom = midas.update((250, 300), 3);
        println!("anom is {} and norm is {}", anom, max);
        assert!(anom > max);
    }
}
