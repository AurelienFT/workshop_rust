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

impl Human {

    fn is_adult(&self) -> bool {
        self.age > 18
    }

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
    let human = Human {
        age: 32,
        name: "Test".to_string(),
        situation: Situation::BOSS
    };
    human.is_adult();
    human.get_spending_cost();
}