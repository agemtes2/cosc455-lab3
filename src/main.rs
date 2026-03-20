fn main() {
    let test1 = "ABC123DEF".to_string();
    let test2 = "123ABC".to_string();

    assert_eq!(q1_parser(test1), true);
    assert_eq!(q1_parser(test2), false);

    println!("All tests passed!");
}

fn is_uppercase_or_digit(c: char) -> bool {
    is_uppercase_letter(c) || (c >= '0' && c <= '9')
}

fn is_uppercase_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn q1_parser(text: String) -> bool {
    let characters_array: Vec<char> = text.chars().collect();

    if characters_array.len() < 2 {
        return false;
    }

    for (i, character) in characters_array.iter().enumerate() {
        if i < 2 {
            if !is_uppercase_letter(*character) {
                return false;
            }
        } else {
            if !is_uppercase_or_digit(*character) {
                return false;
            }
        }
    }

    true
}