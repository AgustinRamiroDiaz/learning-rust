fn main() {
    let id = IDBuilder::new()
        .name("Agustin".to_string())
        .age(24)
        .country("Argentina".to_string())
        .build();

    println!("{id}");

    // Won't compile
    // IDBuilder::new().name("Agustin".to_string()).build();
    // IDBuilder::new().age(24).age(25);
}

struct Country(String);
struct NoCountry;

struct Age(u8);
struct NoAge;

struct One<T>(T);
struct No;

struct IDBuilder<N, A, C> {
    name: N,
    age: A,
    country: C,
}

impl IDBuilder<No, NoAge, NoCountry> {
    fn new() -> Self {
        IDBuilder {
            name: No,
            age: NoAge,
            country: NoCountry,
        }
    }
}

impl<A, C> IDBuilder<No, A, C> {
    fn name(self, name: String) -> IDBuilder<One<String>, A, C> {
        IDBuilder {
            name: One(name),
            age: self.age,
            country: self.country,
        }
    }
}

impl<N, C> IDBuilder<N, NoAge, C> {
    fn age(self, age: u8) -> IDBuilder<N, Age, C> {
        IDBuilder {
            name: self.name,
            age: Age(age),
            country: self.country,
        }
    }
}

impl<N, A> IDBuilder<N, A, NoCountry> {
    fn country(self, country: String) -> IDBuilder<N, A, Country> {
        IDBuilder {
            name: self.name,
            age: self.age,
            country: Country(country),
        }
    }
}

impl IDBuilder<One<String>, Age, Country> {
    fn build(self) -> ID {
        ID {
            name: self.name.0,
            age: self.age.0,
            country: self.country.0,
        }
    }
}

struct ID {
    name: String,
    age: u8,
    country: String,
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} from {} is {} years old",
            self.name, self.country, self.age
        )
    }
}
