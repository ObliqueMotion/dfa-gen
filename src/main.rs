use dfa_gen::{bits_of, DFABuilder, DFA};

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
