struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl From<Dog> for Cat {
    fn from(dog: Dog) -> Self {
        Cat {
            name: dog.name.clone(),
        }
    }
}

fn main() {
    let scruffles_the_dog = Dog {
        name: "Scruffles".to_string(),
    };

    println!("{}", scruffles_the_dog.name);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dog() {
        let scruffles_the_dog = Dog {
            name: "Scruffles".to_string(),
        };
        assert_eq!(scruffles_the_dog.name, "Scruffles");
        let scruffles_the_cat: Cat = scruffles_the_dog.into();
        assert_eq!(scruffles_the_cat.name, "Scruffles");
    }
}
