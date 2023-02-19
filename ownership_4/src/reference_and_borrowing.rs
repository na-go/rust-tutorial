fn calculate_lentgh(s: &String) -> usize {
    s.len()
    // ちなみにsは参照だけが渡され所有権は持っていないのでここでsに変更を加えようとするとエラーを吐く
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
    println!("some_string: {}", some_string);
}


// fn dangle() -> &String { // dangleはStringへの参照を返す
//     let s = String::from("hello"); // sは新しいString
//     &s // String sへの参照を返す
// } // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

pub fn reference_and_borrowing() {
    let s1 = String::from("hello");
    let s1_length = calculate_lentgh(&s1);
    println!("s1: {}, s1_length: {}", s1, s1_length); //s1を参照しただけなのでここでもs1は有効

    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &mut s; // これはchangeを終えた時点で借用を終えてるから1回目になる
    // let r2: &mut s; // 1度に2回以上「可変な」借用はできない
    change(r1);

    {
        let _r1 = &mut s;
    } // ちなみにここでr1のスコープは抜けてるのでr1はもう有効でない
    let r2 = &mut s; // これはr1が一行上でスコープを抜けるから問題ない.

    change(r2);

    let r1 = &s; // 不変な参照
    let _r2 = &s; // 不変な参照は複数しても問題ない
    // let r3 = &mut s; // 既に不変で借用をしているので可変では参照できない. 但し, 不変な参照をその後扱わない場合はエラーを吐かない
    println!("r1: {}", r1); // r3をエラーにするために無理やり使いました

    let _no_dangle = no_dangle();
}