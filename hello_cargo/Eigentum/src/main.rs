fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world");
    let word = first_word2(&s);
    let world = first_word1(&s);
    s.clear();


}
fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word2(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

