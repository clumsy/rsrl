use crate::core::*;
use crate::domains::Transition;
use crate::policies::{Policy, ParameterisedPolicy};
use std::marker::PhantomData;

/// Continuous Actor-Critic Learning Automaton
pub struct CACLAVar<C, PT, PB> {
    pub critic: Shared<C>,
    variance: f64,

    pub target_policy: Shared<PT>,
    pub behaviour_policy: Shared<PB>,

    pub alpha: Parameter,
    pub beta: Parameter,
    pub gamma: Parameter,
}

impl<C, PT, PB> CACLAVar<C, PT, PB> {
    pub fn new<T1, T2, T3>(
        critic: Shared<C>,
        target_policy: Shared<PT>,
        behaviour_policy: Shared<PB>,
        alpha: T1, beta: T2, gamma: T3,
    ) -> Self
    where
        T1: Into<Parameter>,
        T2: Into<Parameter>,
        T3: Into<Parameter>,
    {
        CACLAVar {
            critic,
            variance: 1.0,

            target_policy,
            behaviour_policy,

            alpha: alpha.into(),
            beta: beta.into(),
            gamma: gamma.into(),
        }
    }
}

impl<C, PT, PB> Algorithm for CACLAVar<C, PT, PB>
where
    C: Algorithm,
    PT: Algorithm,
    PB: Algorithm,
{
    fn handle_terminal(&mut self) {
        self.alpha = self.alpha.step();
        self.gamma = self.gamma.step();

        self.critic.borrow_mut().handle_terminal();
        self.variance = 1.0;

        self.target_policy.borrow_mut().handle_terminal();
        self.behaviour_policy.borrow_mut().handle_terminal();
    }
}

impl<S, C, PT, PB> OnlineLearner<S, PT::Action> for CACLAVar<C, PT, PB>
where
    C: OnlineLearner<S, PT::Action> + ValuePredictor<S>,
    PT: ParameterisedPolicy<S, Action = f64>,
    PB: Algorithm,
{
    fn handle_transition(&mut self, t: &Transition<S, PT::Action>) {
        let mut critic = self.critic.borrow_mut();
        let mut policy = self.target_policy.borrow_mut();

        let s = t.from.state();
        let v = critic.predict_v(s);
        let td_error = if t.terminated() {
            t.reward
        } else {
            t.reward + self.gamma * critic.predict_v(t.to.state())
        };

        critic.handle_transition(t);
        self.variance += self.beta.value() * (td_error * td_error - self.variance);

        if td_error > v {
            let mpa = policy.mpa(s);
            let scaler = (td_error / self.variance.sqrt()).ceil();

            policy.update(s, t.action, self.alpha * scaler * (t.action - mpa));
        }
    }
}

impl<S, C, PT, PB> ValuePredictor<S> for CACLAVar<C, PT, PB>
where
    C: ValuePredictor<S>,
{
    fn predict_v(&mut self, s: &S) -> f64 {
        self.critic.borrow_mut().predict_v(s)
    }
}

impl<S, C, PT, PB> ActionValuePredictor<S, PT::Action> for CACLAVar<C, PT, PB>
where
    C: ActionValuePredictor<S, PT::Action>,
    PT: Policy<S>,
{
    fn predict_qs(&mut self, s: &S) -> Vector<f64> {
        self.critic.borrow_mut().predict_qs(s)
    }

    fn predict_qsa(&mut self, s: &S, a: PT::Action) -> f64 {
        self.critic.borrow_mut().predict_qsa(s, a)
    }
}

impl<S, C, PT, PB> Controller<S, PT::Action> for CACLAVar<C, PT, PB>
where
    PT: ParameterisedPolicy<S>,
    PB: Policy<S, Action = PT::Action>,
{
    fn sample_target(&mut self, s: &S) -> PT::Action {
        self.target_policy.borrow_mut().sample(s)
    }

    fn sample_behaviour(&mut self, s: &S) -> PB::Action {
        self.behaviour_policy.borrow_mut().sample(s)
    }
}
