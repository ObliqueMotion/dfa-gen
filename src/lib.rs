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
        match self.transitions.get(transition) {
            Some(state_pairs) => match state_pairs.get(&self.current) {
                Some(destination) => self.current = destination.clone(),
                None => {
                    if self.dead_states.get(&self.current).is_some() {
                        return;
                    }
                    if self.goal_states.get(&self.current).is_some() {
                        return;
                    }
                    panic!("DFA::next(): There is no path defined from state ({:?}) on transition ({:?})", self.current, transition);
                }
            },
            None => panic!(
                "DFA::next(): Attempted to move with unknown transition ({:?})",
                transition
            ),
        }
    }

    pub fn state(&self) -> S {
        (*self.current).clone()
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
        self.restart();
        for transition in inputs {
            self.next(&transition);
        }
        self.eval()
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
            panic!("DFABuilder::add_state(): All states must be added before any transitions are added. Try moving this higher in the builder.");
        }
        self.dfa.states.insert(Rc::new(state.clone()));
        self
    }

    pub fn mark_accept_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.dead_states.get(state) {
                    panic!("DFABuilder::mark_accept_state(): Attempted to mark a dead state ({:?}) as an accept state.", state);
                }
                self.dfa.accept_states.insert(state.clone());
            }
            None => panic!("DFABuilder::mark_accept_state(): Attempted to mark a non-existent state ({:?}) as an accept state.", state),
        }
        self
    }

    pub fn mark_goal_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.dead_states.get(state) {
                    panic!(
                        "DFABuilder::mark_goal_state(): Attempted to mark a dead state ({:?}) as a goal state.",
                        state
                    );
                }
                self.dfa.accept_states.insert(state.clone());
                self.dfa.goal_states.insert(state.clone());
            }
            None => panic!(
                "DFABuilder::mark_goal_state(): Attempted to mark a non-existent state ({:?}) as a goal state.",
                state
            ),
        }
        self
    }

    pub fn mark_dead_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.accept_states.get(state) {
                    panic!("DFABuilder::mark_dead_state(): Attempted to mark an accept state ({:?}) as a dead state.", state);
                }
                self.dfa.dead_states.insert(state.clone());
            }
            None => panic!(
                "DFABuilder::mark_dead_state(): Attempted to mark a non-existent state ({:?}) as a dead state.",
                state
            ),
        }
        self
    }

    pub fn mark_start_state(mut self, state: &S) -> Self {
        if let Some(start) = self.dfa.start {
            panic!("DFABUilder::mark_start_state(): Attempted to mark state ({:?}) as the start state when the start state ({:?}) has already been defined.", state, start);
        }
        match self.dfa.states.get(state) {
            Some(state) => {
                self.dfa.start = Some(state.clone());
                self.dfa.current = state.clone();
            }
            None => panic!(
                "DFABuilder::mark_start_state(): Attempted to mark a non-existent state ({:?}) as a start state.",
                state
            ),
        }
        self
    }

    pub fn add_transition(mut self, from: &S, transition: &T, to: &S) -> Self {
        if let Some(state) = self.dfa.dead_states.get(from) {
            panic!("DFABuilder::add_transition(): Attempted to create a transition from state ({:?}), which is a dead state and all transitions implicitly lead to itself.", state);
        }
        if let Some(state) = self.dfa.goal_states.get(from) {
            panic!("DFABuilder::add_transition(): Attempted to create a transition from state ({:?}), which is a goal state and all transitions implicitly lead to itself.", state);
        }
        match (self.dfa.states.get(from), self.dfa.states.get(to)) {
            (Some(from), Some(to)) => {
                if let Some(state_pairs) = self.dfa.transitions.get_mut(transition) {
                    state_pairs.insert(from.clone(), to.clone());
                } else {
                    let transition = transition.clone();
                    let mut state_pairs = HashMap::new();
                    state_pairs.insert(from.clone(), to.clone());
                    self.dfa.transitions.insert(transition, state_pairs);
                }
            }
            (None, _) => panic!(
                "DFABuilder::add_transition(): Attempted to create a transition from state ({:?}), which does not exist in this DFA.",
                from
            ),
            (_, None) => panic!(
                "DFABuilder::add_transition(): Attempt to create a transition to state ({:?}), which does not exist in this DFA.",
                to
            ),
        }
        self
    }

    pub fn build(self) -> DFA<S, T> {
        if self.dfa.states.is_empty() {
            panic!("DFABuilder::build(): The DFA must have at least one state, but it is empty.");
        }
        if self.dfa.start.is_none() {
            panic!(
                "DFABuilder::build(): DFA must have a valid start state. No start state was defined."
            );
        }
        if self.dfa.accept_states.is_empty() {
            panic!("DFABuilder::build(): DFA must have at least one accept state. No accept states were defined.");
        }
        let transition_states =
            self.dfa.states.len() - self.dfa.dead_states.len() - self.dfa.goal_states.len();
        for state_pairs in self.dfa.transitions.values() {
            if state_pairs.keys().count() < transition_states {
                panic!("DFABuilder::build(): All transition states have a complete set of output edges for every transition.");
            }
        }
        self.dfa
    }
}

#[macro_export]
macro_rules! dfa {
    (
        states{$(
            $state:expr
        ),*};
        start { $start:expr };
        marks {
            accept { $(
                $accept:expr
            ),*},
            dead{$(
                $dead:expr
            ),*},
            goal{$(
                $goal:expr
            ),*},
        };
        transitions {$(
            $edge:expr => ($from:expr, $to:expr),
        )*};
    ) => {{
            DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
}