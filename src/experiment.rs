use agents::Agent;
use domain::{Domain, Observation};
use geometry::{Space, ActionSpace};


#[derive(Debug)]
pub struct Episode {
    pub n_steps: u64,
    pub total_reward: f64
}


pub struct SerialExperiment<'a, A: 'a, D> {
    agent: &'a mut A,
    domain_factory: Box<Fn() -> D>,

    step_limit: u64
}

impl<'a, S: Space, A, D> SerialExperiment<'a, A, D>
    where A: Agent<S>,
          D: Domain<StateSpace=S, ActionSpace=ActionSpace>
{
    pub fn new(agent: &'a mut A,
               domain_factory: Box<Fn() -> D>,
               step_limit: u64) -> SerialExperiment<'a, A, D>
    {
        SerialExperiment {
            agent: agent,
            domain_factory: domain_factory,
            step_limit: step_limit,
        }
    }
}

impl<'a, S: Space, A, D> Iterator for SerialExperiment<'a, A, D>
    where A: Agent<S>,
          D: Domain<StateSpace=S, ActionSpace=ActionSpace>
{
    type Item = Episode;

    fn next(&mut self) -> Option<Episode> {
        let mut domain = (self.domain_factory)();
        let mut a = self.agent.pi(domain.emit().state());

        let mut e = Episode {
            n_steps: 1,
            total_reward: 0.0,
        };

        for j in 1..(self.step_limit+1) {
            let t = domain.step(a);

            e.n_steps = j;
            e.total_reward += t.reward;

            self.agent.train(&t);

            a = match t.to {
                Observation::Terminal(_) => break,
                _ => self.agent.pi(&t.to.state())
            };
        }

        Some(e)
    }
}


pub struct Evaluation<'a, A: 'a, D> {
    agent: &'a mut A,
    domain_factory: Box<Fn() -> D>,
}

impl<'a, S: Space, A, D> Evaluation<'a, A, D>
    where A: Agent<S>,
          D: Domain<StateSpace=S, ActionSpace=ActionSpace>
{
    pub fn new(agent: &'a mut A,
               domain_factory: Box<Fn() -> D>) -> Evaluation<'a, A, D>
    {
        Evaluation {
            agent: agent,
            domain_factory: domain_factory,
        }
    }
}

impl<'a, S: Space, A, D> Iterator for Evaluation<'a, A, D>
    where A: Agent<S>,
          D: Domain<StateSpace=S, ActionSpace=ActionSpace>
{
    type Item = Episode;

    fn next(&mut self) -> Option<Episode> {
        let mut domain = (self.domain_factory)();
        let mut a = self.agent.pi(domain.emit().state());

        let mut e = Episode {
            n_steps: 1,
            total_reward: 0.0,
        };

        loop {
            let t = domain.step(a);

            e.n_steps += 1;
            e.total_reward += t.reward;

            a = match t.to {
                Observation::Terminal(_) => break,
                _ => self.agent.pi(&t.to.state())
            };
        }

        Some(e)
    }
}
