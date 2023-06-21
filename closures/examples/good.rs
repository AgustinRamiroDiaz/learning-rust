struct Person {
    name: String,
    age: u8,
}

fn apply_twice<F, T>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    f(f(x))
}

fn main() {
    let cindy = Person {
        name: "Cindy".to_string(),
        age: 25,
    };

    let old = "old".to_string();
    // let grow = |Person { name, age }| Person { name, age: age + 1 };
    let grow = |Person { name, age }| Person {
        name: old.clone() + &name,
        age: age + 1,
    };

    let old_cindy = apply_twice(grow, cindy);

    println!("{} is {} years old", old_cindy.name, old_cindy.age);
}
