enum Situation {
    EMPLOYEE,
    STUDENT,
    KID,
    BOSS,
}

struct Human {
    age: i32,
    name: String,
    situation: Situation
}


struct Animal {
    age: i32,
    name: String,
}

trait LivingBeings {
    fn is_adult(&self) -> bool {
        false
    }
}

impl LivingBeings for Human {
    fn is_adult(&self) -> bool {
        self.age > 18
    }
}

impl LivingBeings for Animal {
    fn is_adult(&self) -> bool {
        self.age > 7
    }
}


impl Human {

    fn get_spending_cost(&self) -> i32 {
        match self.situation {
            Situation::BOSS => 3000,
            Situation::EMPLOYEE => 1500,
            Situation::STUDENT => 700,
            Situation::KID => 0
        }
    }
}

fn main() {
    let animal = Animal {
        age: 8,
        name: "Test".to_string(),
    };
    println!("{}", animal.is_adult());
}