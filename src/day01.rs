use std::fs;

pub fn read_input_file_and_save_it_in_vec() -> Vec<String> {
    let path = "./assets/day01_input.txt";

    let file = fs::read_to_string(path).expect("should read the input file");

    let mut current_line = String::from("");
    let mut line_vector: Vec<String> = vec![];

    for c in file.chars() {
        if c == '\r' || c == '\n' {
            if current_line.ne("") {
                line_vector.push(current_line.to_owned());
            }
            current_line = String::from("");
        } else {
            current_line.push_str(&c.to_string());
        }
    }
    line_vector
}

//first star solution
pub fn take_numbers_of_vec_and_calc_sum() -> u32 {
    let line_vector = read_input_file_and_save_it_in_vec();
    let mut sum_of_calibration_values = 0;

    for v in line_vector {
        let mut first_number: char = char::default();
        let mut last_number: char = char::default();

        let mut first_number_set: bool = false;
        let mut last_number_set: bool = false;

        for character in v.chars() {
            if character.is_numeric() && !first_number_set {
                first_number = character;
                first_number_set = true;
            } else if character.is_numeric() {
                last_number = character;
                last_number_set = true;
            }
        }
        let mut number_string = first_number.to_string();
        if !last_number_set {
            last_number = first_number;
        }

        number_string.push(last_number);

        sum_of_calibration_values += number_string.parse::<u32>().unwrap();
    }
    sum_of_calibration_values
}

pub fn match_digits(digit: &str) -> char {
    if digit.contains("zero") {
        return '0';
    }
    if digit.contains("one") {
        return '1';
    }
    if digit.contains("two") {
        return '2';
    }
    if digit.contains("three") {
        return '3';
    }
    if digit.contains("four") {
        return '4';
    }
    if digit.contains("five") {
        return '5';
    }
    if digit.contains("six") {
        return '6';
    }
    if digit.contains("seven") {
        return '7';
    }
    if digit.contains("eight") {
        return '8';
    }
    if digit.contains("nine") {
        return '9';
    }
    'e'
}

//second star solution
pub fn take_numbers_of_vec_and_calc_real_sum() -> u32 {
    let line_vector = read_input_file_and_save_it_in_vec();
    let mut sum_of_calibration_values = 0;

    for v in line_vector {
        let mut first_number: char = char::default();
        let mut last_number: char = char::default();
        let mut first_number_set: bool = false;
        let mut last_number_set: bool = false;
        let mut current_word: String = String::default();

        for character in v.chars() {
            current_word.push(character);

            if !first_number_set {
                let current_word_char = match_digits(&current_word);

                if character.is_numeric() {
                    first_number = character;
                    first_number_set = true;
                    continue;
                }
                if current_word_char != 'e' {
                    first_number = current_word_char;
                    first_number_set = true;
                    current_word = String::default();
                    continue;
                }
            } else {
                let current_word_char = match_digits(&current_word);
                if character.is_numeric() {
                    last_number = character;
                    last_number_set = true
                }
                if current_word_char != 'e' {
                    last_number = current_word_char;
                    last_number_set = true;
                    current_word = String::default();
                }
            }
        }

        let mut number_string = first_number.to_string();
        if !last_number_set {
            last_number = first_number;
        }

        number_string.push(last_number);

        sum_of_calibration_values += number_string.parse::<u32>().unwrap();
    }
    sum_of_calibration_values
}
