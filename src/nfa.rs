pub fn initialise_nfa() -> nfa<String>{
    let mut nfa: nfa<String> = Nfa::new();
    let a = nfa.add_state("hello");
    nfa.add_epsilon_transition(a, a);
    nfa.add_transition(a, "hello" ,a);
    // examples to start with probs should read the docs before continuing
    // https://doc.rust-lang.org/beta/nightly-rustc/rustc_transmute/layout/nfa/struct.Nfa.html#

    return nfa;
}