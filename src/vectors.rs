pub fn reversed_string(string: &str) -> String {
    let reversed = string.chars().rev().collect::<String>();
    println!("reversed_string: {}", reversed);
    reversed
}
pub fn split_string_and_collect_vector(sentence: &str) -> Vec<&str> {
    // let words = sentence.split_whitespace().collect::<Vec<&str>>();
    let words = sentence.split(" ").collect::<Vec<&str>>();
    println!("split_string_and_collect_vector: {:?}", words);
    words
}

pub fn reversed_sentence(sentence: &str) -> String {
    let words = split_string_and_collect_vector(sentence);
    let mut reversed_sentence = String::new();
    for word in words.iter().rev() {
        reversed_sentence.push_str(word);
        reversed_sentence.push_str(" ");
    }
    println!("reversed_sentence: {}", reversed_sentence);
    reversed_sentence
}

pub fn reversed_sentence_using_fold(sentence: &str) -> String {
    let reversed = sentence.chars().fold(String::new(), |mut acc, c| {
        acc.insert(0, c);
        acc
    });
    println!("reversed_sentence_using_fold: {}", reversed);
    reversed
}

pub fn get_item(index: usize) -> Result<i32, &'static str> {
    let vector = vec![2, 8, 16, 32, 64];

    if index >= vector.len() {
        Err("Index out of bounds")
    } else {
        let item = vector[index];

        println!("get_item: {:?}", item);

        Ok(item)
    }
}
