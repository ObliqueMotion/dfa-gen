use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct DFA<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    states: HashSet<Rc<S>>,
    accept_states: HashSet<Rc<S>>,
    dead_states: HashSet<Rc<S>>,
    goal_states: HashSet<Rc<S>>,
    transitions: HashMap<T, HashMap<Rc<S>, Rc<S>>>,
    start: Option<Rc<S>>,
    current: Rc<S>,
}

#[derive(Debug)]
pub enum Evaluation {
    Accept,
    Reject,
}

impl<S, T> DFA<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    pub fn next(&mut self, transition: &T) {
        if self.dead_states.get(&self.current).is_some() {
            return;
        }
        if self.goal_states.get(&self.current).is_some() {
            return;
        }
        match self.transitions.get(transition) {
            Some(state_pairs) => {
                match state_pairs.get(&self.current) {
                    Some(destination) => self.current = destination.clone(),
                    None => panic!("Invalid Transition: There is no path defined from state ({:?}) on transition ({:?})", self.current, transition),
                }
            }
            None => panic!("Invalid Transition: Attempted to move with unknown transition ({:?})", transition),
        }
    }

    pub fn restart(&mut self) {
        self.current = self.start.as_ref().unwrap().clone();
    }

    pub fn eval(&self) -> Evaluation {
        if self.accept_states.get(&self.current).is_some() {
            Evaluation::Accept
        } else {
            Evaluation::Reject
        }
    }

    pub fn recognize(&mut self, inputs: impl Iterator<Item = T>) -> Evaluation {
        for transition in inputs {
            self.next(&transition);
            if self.dead_states.get(&self.current).is_some() {
                break;
            }
            if self.goal_states.get(&self.current).is_some() {
                break;
            }
        }
        self.eval()
    }

    pub fn recognize_new(&mut self, inputs: impl Iterator<Item = T>) -> Evaluation {
        self.restart();
        self.recognize(inputs)
    }
}

#[derive(Debug, Default)]
pub struct DFABuilder<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    dfa: DFA<S, T>,
}

impl<'a, S, T> DFABuilder<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    pub fn add_state(mut self, state: &S) -> Self {
        if !self.dfa.transitions.is_empty() {
            panic!("Failed to add state. No states may be added after any transition is added. Try moving this higher in the builder.");
        }
        self.dfa.states.insert(Rc::new(state.clone()));
        self
    }

    pub fn mark_accept_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.dead_states.get(state) {
                    panic!("Invalid Accept State: Attempted to mark dead state ({:?}) as goal state.", state);
                }
                self.dfa.accept_states.insert(state.clone());
            }
            None => panic!("Invalid Accept State: Attempted to mark non-existent state ({:?}) as a accept state.", state),
        }
        self
    }

    pub fn mark_goal_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.dead_states.get(state) {
                    panic!(
                        "Invalid Goal State: Attempted to mark dead state ({:?}) as goal state.",
                        state
                    );
                }
                self.dfa.accept_states.insert(state.clone());
                self.dfa.goal_states.insert(state.clone());
            }
            None => panic!(
                "Invalid Goal State: Attempted to mark non-existent state ({:?}) as goal state.",
                state
            ),
        }
        self
    }

    pub fn mark_dead_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.accept_states.get(state) {
                    panic!("Invalid Dead State: Attempted to mark accept state ({:?}) as a dead state.", state);
                }
                self.dfa.dead_states.insert(state.clone());
            }
            None => panic!(
                "Invalid Dead State: Attempted to mark non-existent state ({:?}) as dead state.",
                state
            ),
        }
        self
    }

    pub fn mark_start_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                self.dfa.start = Some(state.clone());
                self.dfa.current = state.clone();
            }
            None => panic!(
                "Invalid Start State: Attempted to mark non-existent state ({:?}) as start state.",
                state
            ),
        }
        self
    }

    pub fn add_transition(mut self, from: &S, transition: &T, to: &S) -> Self {
        if let Some(state) = self.dfa.dead_states.get(from) {
            panic!("Invalid Transition: From state ({:?}) is a dead state and cannot have transitions.", state);
        }
        if let Some(state) = self.dfa.goal_states.get(from) {
            panic!("Invalid Transition: From state ({:?}) is a goal state and cannot have transitions.", state);
        }
        match (self.dfa.states.get(from), self.dfa.states.get(to)) {
            (Some(from), Some(to)) => {
                let transition = transition.clone();
                self.dfa
                    .transitions
                    .entry(transition)
                    .and_modify(|states| {
                        states.insert(from.clone(), to.clone());
                    })
                    .or_insert({
                        let mut states = HashMap::new();
                        states.insert(from.clone(), to.clone());
                        states
                    });
            }
            (None, _) => panic!(
                "Invalid Transition: From state ({:?}) does not exist in this DFA.",
                from
            ),
            (_, None) => panic!(
                "Invalid Transition: To state ({:?}) does not exist in this DFA.",
                to
            ),
        }
        self
    }

    pub fn build(self) -> DFA<S, T> {
        if self.dfa.start.is_none() {
            panic!(
                "Failed to Build DFA: No start state was defined. Try using mark_start_state()."
            );
        }
        if self.dfa.accept_states.is_empty() {
            panic!("Failed to Build DFA: No accept states were defined. try using mark_accept_state().");
        }
        let transition_states =
            self.dfa.states.len() - self.dfa.dead_states.len() - self.dfa.goal_states.len();
        for state_pairs in self.dfa.transitions.values() {
            if state_pairs.keys().count() < transition_states {
                panic!("Failed to Build DFA: Not all transition states are fully connectet.");
            }
        }
        self.dfa
    }
}

pub fn bits_of(x: u128) -> impl Iterator<Item = u8> {
    format!("{:b}", x)
        .into_bytes()
        .into_iter()
        .map(|b| (b as char).to_digit(10).unwrap() as u8)
}
