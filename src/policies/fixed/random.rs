use domains::Transition;
use geometry::Vector;
use policies::{Policy, FinitePolicy};
use rand::{
    thread_rng, ThreadRng,
    distributions::{IndependentSample, Range},
};
use Handler;

pub struct Random(usize, ThreadRng);

impl Random {
    pub fn new(n_actions: usize) -> Self {
        Random(n_actions, thread_rng())
    }
}

impl<S> Handler<Transition<S, usize>> for Random {}

impl<S> Policy<S, usize> for Random {
    fn sample(&mut self, _: &S) -> usize {
        Range::new(0, self.0).ind_sample(&mut self.1)
    }

    fn probability(&mut self, _: &S, _: usize) -> f64 {
        1.0 / self.0 as f64
    }
}

impl<S> FinitePolicy<S> for Random {
    fn probabilities(&mut self, s: &S) -> Vector<f64> {
        vec![1.0 / self.0 as f64; self.0].into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Policy, Random};

    #[test]
    fn test_sampling() {
        let mut p = Random::new();
        let qs = vec![1.0, 0.0];

        let mut n0: f64 = 0.0;
        let mut n1: f64 = 0.0;
        for _ in 0..10000 {
            match p.sample(&qs) {
                0 => n0 += 1.0,
                _ => n1 += 1.0,
            }
        }

        assert!((0.50 - n0 / 10000.0).abs() < 0.05);
        assert!((0.50 - n1 / 10000.0).abs() < 0.05);
    }

    #[test]
    fn test_probabilites() {
        let mut p = Random::new();

        assert_eq!(p.probabilities(&[1.0, 0.0, 0.0, 1.0]), vec![0.25; 4]);
        assert_eq!(p.probabilities(&[1.0, 0.0, 0.0, 0.0, 0.0]), vec![0.2; 5]);
        assert_eq!(p.probabilities(&[0.0, 0.0, 0.0, 0.0, 1.0]), vec![0.2; 5]);
    }
}
