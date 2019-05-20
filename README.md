# dfa-gen
A Generic Deterministic Finite-State Automaton Generator Written in Rust

## Example

```rust
use dfagen::{DFABuilder, DFA};

fn main() {
    let no_one = String::from("1 has not been seen.");
    let has_one = String::from("1 has been seen.");

    let mut dfa = DFABuilder::default()
        .add_state(&no_one)
        .add_state(&has_one)
        .mark_start_state(&no_one)
        .mark_goal_state(&has_one)
        .add_transition(&no_one, &'0', &no_one)
        .add_transition(&no_one, &'1', &has_one)
        .build();

    dbg!(&dfa);
    dbg!(dfa.recognize_new("000".chars()));
    dbg!(dfa.recognize_new("100".chars()));
    dbg!(dfa.recognize_new("010".chars()));
    dbg!(dfa.recognize_new("001".chars()));
    dbg!(dfa.recognize_new("111".chars()));
}
```

**Output**
```
[src/main.rs:16] &dfa = DFA {
    states: {
        "1 has been seen.",
        "1 has not been seen."
    },
    accept_states: {
        "1 has been seen."
    },
    dead_states: {},
    goal_states: {
        "1 has been seen."
    },
    transitions: {
        '0': {
            "1 has not been seen.": "1 has not been seen."
        },
        '1': {
            "1 has not been seen.": "1 has been seen."
        }
    },
    start: Some(
        "1 has not been seen."
    ),
    current: "1 has not been seen."
}
[src/main.rs:17] dfa.recognize_new("000".chars()) = Reject
[src/main.rs:18] dfa.recognize_new("100".chars()) = Accept
[src/main.rs:19] dfa.recognize_new("010".chars()) = Accept
[src/main.rs:20] dfa.recognize_new("001".chars()) = Accept
[src/main.rs:21] dfa.recognize_new("111".chars()) = Accept

```
