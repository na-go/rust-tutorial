mod reference_and_borrowing;

fn variables_scope() {
    let s = "hello";
    {
        let t = "world";
        println!("{}, {}", s, t); // s,tが生きてる
    }

    println!("{}", s); // tはscopeから出たので死んだ
}

fn string_type() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn variables_and_data_interaction() {
    let x = 5;
    let y = x;

    println!("x: {}", x);
    println!("y: {}", y);

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("s1: {}", s1); // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    println!("s2: {}", s2);

    let s1_clone = String::from("hello");
    let s2_clone = s1_clone.clone();

    println!("s1: {}, s2: {}", s1_clone, s2_clone);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownerships() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_lentgh(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn main() {
    variables_scope();
    string_type();
    variables_and_data_interaction();

    let s = String::from("hello");

    takes_ownership(s.clone()); //これ以降もsを使いたかったらcloneする？
    takes_ownership(s);
    // println!("{}", s); すでにsはtakes_ownershipにmoveしてしまったため有効でない
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownerships();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s3: {}", s1, s3);
    // println!("s2: {}", s2); ムーブ済みなので有効でない
    let s11 = String::from("I'm Nago.");
    let (s12, s12_length) = calculate_lentgh(s11);

    println!("s12: {}, s12_length: {}", s12, s12_length);
    
    reference_and_borrowing::reference_and_borrowing();
}
