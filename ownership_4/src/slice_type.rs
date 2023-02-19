fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_str = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_str > 0 {
            return &s[first_str..i]
        } else if item == b' '{
            first_str += i+1;
        }
    }
    &s[..]
}

fn re_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

fn re_second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_str = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_str > 0 {
            return &s[first_str..i]
        } else if item == b' '{
            first_str += i+1;
        }
    }
    &s[..]
}

pub fn slice_type() {
    let s = String::from("I am Nago.");

    let word = first_word(&s);
    let word_2 = second_word(&s);

    println!("word: {}, word_2; {}", word, word_2);

    // 改善版
    let my_string = String::from("I am Nago.");

    let word = re_first_word(&my_string[..]);
    let word_2 = re_second_word(&my_string[..]);

    println!("word: {}, word_2:{}", word, word_2);

    let my_string_literal = "I am Nago.";

    let word = re_first_word(my_string_literal);
    let word_2 = re_second_word(my_string_literal);

    println!("word: {}, word_2:{}", word, word_2);

    let a = [1,2,3,4,5];
    let _slice = &a[1..3];
}