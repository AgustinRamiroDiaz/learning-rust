fn moving_ownership() {
    let owner = "data".to_string();
    let stealer = owner;
    let second_stealer = owner;
}

fn borrowing() {
    let mut owner = "data".to_string();
    let watcher = &owner;
    let second_watcher = &owner;

    watcher.push_str("!");

    let mutable_reference = &mut owner;
    mutable_reference.push_str("!");
}

fn dangling_reference(_input: &String) -> &String {
    &"hang in there".to_string()
}

fn dangling_reference_with_lifetimes<'a>(_input: &'a String) -> &'a String {
    &"hang in there".to_string()
}

fn main() {}
