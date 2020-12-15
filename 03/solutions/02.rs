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

fn main() {
    let human = Human {
        age: 32,
        name: "Test".to_string(),
        situation: Situation::BOSS
    };
}