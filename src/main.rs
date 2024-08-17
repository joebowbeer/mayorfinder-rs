fn main() {
    let peeps = ["A", "B", "C"];

    fn knows(a: &str, b: &str) -> bool {
        matches!((a, b), ("A", "B") | ("C", "B"))
    }

    println!(
        "{} is mayor.",
        find_mayor(knows, &peeps).or("No one".into()).unwrap()
    );
}

/// Given a list of people and a "knows" relation, find the mayor,
/// that is, the person who everyone knows but who doesn't know anyone.
fn find_mayor<P: Copy + PartialEq>(knows: fn(P, P) -> bool, peeps: &[P]) -> Option<P> {
    // Find the "most known" person
    let candidate = peeps
        .iter()
        .fold(None, |acc: Option<P>, b: &P| -> Option<P> {
            match acc {
                Some(a) if knows(*b, a) => Some(a),
                Some(a) if knows(a, *b) => Some(*b),
                None => Some(*b),
                _ => None,
            }
        });
    // This person is mayor if they don't know anyone but everyone knows them
    match candidate {
        Some(c)
            if peeps
                .iter()
                .all(|p| c == *p || (!knows(c, *p) && knows(*p, c))) =>
        {
            candidate
        }
        _ => None,
    }
}

#[test]
fn find_mayor_test() {
    fn knows(a: &str, b: &str) -> bool {
        matches!((a, b), ("A", "B") | ("C", "B"))
    }
    assert_eq!(Some("B"), find_mayor(knows, &["A", "B", "C"]), "ABC");
    assert_eq!(None, find_mayor(knows, &["A", "B", "C", "X"]), "ABCX");
    assert_eq!(None, find_mayor(knows, &["A", "X", "B", "C"]), "AXBC");
    assert_eq!(None, find_mayor(|_, _| false, &[] as &[&str]), "empty");
    assert_eq!(Some("A"), find_mayor(|_, _| false, &["A"]), "one");
}
