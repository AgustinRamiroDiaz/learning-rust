fn longer<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let outer = "outer".to_string();
    let longer_word: &String;

    {
        let inner = "inner".to_string();
        longer_word = longer(&outer, &inner);
    }

    println!("{}", longer_word);
}
