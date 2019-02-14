use crate::core::*;
use crate::geometry::Space;
use crate::policies::{FinitePolicy, Policy};
use rand::{
    distributions::Distribution,
    rngs::ThreadRng,
    Rng,
    thread_rng,
};
use std::ops::Add;

pub struct PerturbedPolicy<P, D, R = ThreadRng> {
    pub base_policy: P,
    pub noise_dist: D,

    rng: R,
}

impl<P, D> PerturbedPolicy<P, D> {
    pub fn new(base_policy: P, noise_dist: D) -> Self {
        PerturbedPolicy::with_rng(base_policy, noise_dist, thread_rng())
    }
}

impl<P, D, R> PerturbedPolicy<P, D, R> {
    pub fn with_rng(base_policy: P, noise_dist: D, rng: R) -> Self {
        PerturbedPolicy {
            base_policy,
            noise_dist,
            rng,
        }
    }
}

impl<P, D, R> Algorithm for PerturbedPolicy<P, D, R> {}

impl<S, P, D, R> Policy<S> for PerturbedPolicy<P, D, R>
where
    P: Policy<S>,
    D: Distribution<P::Action>,
    R: Rng,
    P::Action: Add<P::Action, Output = P::Action>,
{
    type Action = P::Action;

    fn sample(&mut self, s: &S) -> P::Action {
        let base_action = self.base_policy.sample(s);
        let perturbation = self.noise_dist.sample(&mut self.rng);

        base_action + perturbation
    }

    fn probability(&mut self, _: &S, _: P::Action) -> f64 {
        unimplemented!()
    }
}

impl<S, P, D, R> FinitePolicy<S> for PerturbedPolicy<P, D, R>
where
    P: FinitePolicy<S>,
    D: Distribution<P::Action>,
    R: Rng,
{
    fn n_actions(&self) -> P::Action {
        self.base_policy.n_actions()
    }

    fn probabilities(&mut self, _: &S) -> Vector<f64> {
        unimplemented!()
    }
}
