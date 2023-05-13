#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        println!("hello, my name is {}", self.name);
        println!("my age is {}", self.age);
    }
}

#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race { // no receiver, a static method
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) { // exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        // shared and read-only borrowed access to self
        println!("recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("lap {idx}: {lap}  sec");
        }
    }

    fn finish(self) {
        // exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let peter = Person {
        name: String::from("peter"),
        age: 27,
    };

    peter.say_hello();

    let mut race = Race::new("monaco grand prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
}
