// I guess a trait is an abstract class
trait Animal {
    // Static function since it doesn't take &self as a parameter.
    fn create(name: &'static str) -> Self;
    // Must be implemented.
    fn name(&self) -> &'static str;
    // Doesn't need to be implemented as it as a default implementation.
    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human {
            name: name,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat {
            name: name,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        // let mut result: i32 = 0;

        // for x in self {
        //     result += *x;
        // }

        // result

        return self
            .iter()
            .fold(1, |sum, x| sum * x);
    }
}

fn main() {
    // let h = Human {
    //     name: "Harvey",
    // };
    // The compiler actually chooses to use the Human::create() method since we specified the type.
    let h: Human = Animal::create("Harvey");

    h.talk();

    let c = Cat {
        name: "Cattoo",
    };

    c.talk();

    let a = vec![1, 2, 3];

    println!("sum = {}", a.sum());
}
