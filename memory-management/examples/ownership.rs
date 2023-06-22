fn moving_ownership() {
    let owner = "data".to_string();
    let stealer = owner;
    let second_stealer = owner;
}

fn borrowing() {
    let mut owner = "data".to_string();
    // Multiple immutable (read-only) references
    let watcher = &owner;
    let second_watcher = &owner;

    watcher.push_str("!"); // cannot mutate immutable reference

    let mutable_reference = &mut owner; // first mutable reference
    mutable_reference.push_str("!");
}

fn either_one_mutable_or_many_inmutables() {
    let mut owner = "data".to_string();
    // Multiple immutable (read-only) references
    let watcher = &owner;
    let second_watcher = &owner;

    let mutable_reference = &mut owner; // first mutable reference
    mutable_reference.push_str("!");

    println!("{} {} {}", watcher, second_watcher, mutable_reference);
}

fn only_one_mutable() {
    let mut owner = "data".to_string();
    let first_mutable_reference = &mut owner;
    let second_mutable_reference = &mut owner; // second mutable reference is not allowed

    println!("{} {}", first_mutable_reference, second_mutable_reference);
}

fn dangling_reference(_input: &String) -> &String {
    &"hang in there".to_string()
}

fn dangling_reference_with_lifetimes<'a>(_input: &'a String) -> &'a String {
    &"hang in there".to_string()
}

fn main() {}
