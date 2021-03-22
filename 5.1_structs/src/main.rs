struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main_1() {
    let mut user1 = User {
        email: String::from("knabinn97@gmail.com"),
        username: String::from("Nabin Kim"),
        active: true,
        sign_in_count: 1,
    };

    // .을 통해 접근할 수 있다.
    user1.active = false;

    let user2 = User {
        email: String::from("knabinn97@gmail.com"),
        username: String::from("Nabin Kim"),
        //active: user1.active,
        //sign_in_count: user1.sign_in_count,
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        //email: email,
        //username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main_2() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/*
struct User2 {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}*/

fn main() {
    /*
    let user1 = User2 {
        email: "knabinn97@gmail.com",
        username: "Nabin Kim",
        active: true,
        sign_in_count: 1,
    };
    */
}