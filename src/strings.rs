pub fn print_string_slice(s: &str) {
    // let mut new_string = s.to_string();
    // new_string.push_str("! other stuff here");

    // let new_string = s.to_owned() + "! other stuff here";

    let new_string = format!("{}! other stuff here", s);

    println!("{}", new_string);
}

pub fn print_string(s: String) {
    println!("{}", s);
}

pub fn first_three_characters_from_sentence(s: &str) -> &str {
    println!("{}", &s[0..3]);
    &s[0..3]
}

pub fn concatenate_using_format(s: &str) {
    println!("{}", format!("Title: Quick story\n{}", s));
}

pub fn loop_over_chars(s: &str) {
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Found the vowel '{}'!", c),
            _ => println!("Found a consonant - '{}'!", c),
        }
    }
}
