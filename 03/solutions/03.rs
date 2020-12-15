enum Situation {
    EMPLOYEE,
    STUDENT,
    KID,
    BOSS
}

struct Human {
    age: i32,
    name: String,
    situation: Situation
}

fn is_adult(human: Human) -> bool {
    human.age > 18
}

fn main() {
    let human = Human {
        age: 32,
        name: "Test".to_string(),
        situation: Situation::BOSS
    };
    is_adult(human);
}