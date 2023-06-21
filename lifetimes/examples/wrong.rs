fn longer(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let outer = "outer";
    let longer_word: &str;

    {
        let inner = "inner";
        longer_word = longer(outer, inner);
    }

    println!("{}", longer_word);
}
