fn main() {
    let mut u = new_user ("me@example.com".to_string(), "username".to_string());

    println! ("{}", u.username);
    println! ("{:#?}", u);

    u.username = String::from ("new_username");

    println! ("{}", u.username);
    println! ("{:#?}", u);

    let u2 = User {
        email: String::from ("new_email@example.com"),
        username: String::from ("new_user"),
        ..u
    };
    println! ("{:#?}", u2);

    a_vs_b ();

    let mut rec = Rectangle { width: 5, height: 5 };
    let rec1 = Rectangle { width: 3, height: 3 };
    let rec2 = Rectangle { width: 6, height: 6 };

    println! ("The area of the rectangle is : {}", rec.area());

    rec.update_height (10);

    println! ("The area of the rectangle is : {}", rec.area());

    println! ("Can rec hold rec1?: {}", rec.can_hold(&rec1));
    println! ("Can rec hold rec1?: {}", rec.can_hold(&rec2));
}

// we can derive a number of traits to add useful functionality to a struct
#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}

fn new_user (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

// tuple structs
struct A (i32, i32, i32);
struct B (i32, i32, i32);

fn a_vs_b () {
    let a : A = A (32, 34, 1);
    println! ("a {} {} {}", a.0, a.1, a.2);
    let b : B = B (34, 13, 23);
    println! ("b {} {} {}", b.0, b.1, b.2);
    // let c : B = a; compile-time error : cannot cast identical tuple structs
    // let d : A = b; compile-time error : cannot cast identical tuple structs
}

// A != B

// methods

#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn area (&self) -> i32 {
        self.height * self.width
    }

    fn update_height (&mut self, height: i32) {
        self.height = height
    }

    fn can_hold (&self, r : &Rectangle) -> bool {
        self.width > r.width &&  self.height > r.height
    }
}