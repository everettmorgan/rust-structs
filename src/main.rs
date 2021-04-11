use std::fmt;

impl fmt::Display for User {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{{ u: {u}, e: {e}, a: {a}, sc: {sc} }}",
            u=&self.username,
            e=&self.email,
            a=&self.active,
            sc=&self.sign_in_count,
        )
    }
}

fn main() {
    let mut u = new_user ("me@example.com".to_string(), "username".to_string());

    println! ("{}", u.username);
    println! ("{}", u);

    u.username = String::from ("new_username");

    println! ("{}", u.username);
    println! ("{}", u);

    let u2 = User {
        email: String::from ("new_email@example.com"),
        username: String::from ("new_user"),
        ..u
    };
    println! ("{}", u2);

    a_vs_b ();
}

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