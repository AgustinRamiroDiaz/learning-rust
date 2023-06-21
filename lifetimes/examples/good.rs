fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
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
