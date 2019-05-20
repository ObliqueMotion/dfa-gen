use dfagen::{DFABuilder};

fn main() {
    let q0 = String::from("q0");
    let q1 = String::from("q1");
    let q2 = String::from("q2");
    let q3 = String::from("q3");

    let mut dfa = DFABuilder::default()
        .add_state(&q0)
        .add_state(&q1)
        .add_state(&q2)
        .add_state(&q3)
        .mark_dead_state(&q3)
        .mark_start_state(&q0)
        .mark_accept_state(&q0)
        .mark_accept_state(&q1)
        .add_transition(&q0, &'a', &q1)
        .add_transition(&q0, &'b', &q3)
        .add_transition(&q1, &'a', &q1)
        .add_transition(&q1, &'b', &q2)
        .add_transition(&q2, &'a', &q1)
        .add_transition(&q2, &'b', &q2)
        .build();

    dbg!(&dfa);

    dbg!(dfa.recognize("".chars()));
    dbg!(dfa.recognize("a".chars()));
    dbg!(dfa.recognize("b".chars()));
    dbg!(dfa.recognize("aa".chars()));
    dbg!(dfa.recognize("ab".chars()));
    dbg!(dfa.recognize("abb".chars()));
    dbg!(dfa.recognize("aba".chars()));
    dbg!(dfa.recognize("abba".chars()));
    dbg!(dfa.recognize("babba".chars()));
}