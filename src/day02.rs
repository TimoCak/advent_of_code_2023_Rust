use std::{error::Error, fs};

#[derive(Default, Debug, Clone)]
pub struct Cubeset {
    pub amount: usize,
    pub color: String,
    pub game: usize,
}

pub fn valid_colors(s: &str) -> bool {
    match s {
        "red" => true,
        "green" => true,
        "blue" => true,
        _ => false,
    }
}

pub fn read_input_file() -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string("./assets/day02_input.txt")?;

    Ok(file)
}

pub fn cut_prefix_string_and_delete_spaces(s: &str) -> String {
    let mut found_game_prefix = false;
    let mut string_without_prefix = String::from("");
    let iterator = s.chars().enumerate();
    for c in iterator.clone() {
        if c.1 == ':' {
            found_game_prefix = true;
            continue;
        }
        if found_game_prefix && c.1 != ' ' && c.1 != char::default() {
            string_without_prefix.push(c.1);
        }
    }
    string_without_prefix.push(';');
    string_without_prefix
}

pub fn store_vec_of_cubeset(i: usize, s: String) -> Vec<Cubeset> {
    let mut current_color = String::default();
    let mut current_amount = String::default();
    let mut current_cubeset = Cubeset::default();

    let line_iterator = s.chars().enumerate();

    let mut cubeset_vec = vec![];
    for c in line_iterator {
        if c.1.is_numeric() {
            current_amount.push(c.1);
            current_cubeset.amount = current_amount.parse::<usize>().unwrap();
        }
        if !c.1.is_numeric() {
            if valid_colors(&current_color) {
                current_cubeset.color = current_color.clone();
            } else {
                current_color.push(c.1);
            }
        }

        if c.1 == ',' || c.1 == ';' {
            current_color = String::default();
            current_amount = String::default();
            current_cubeset.game = i + 1;
            cubeset_vec.push(current_cubeset);
            current_cubeset = Cubeset::default();
        }
    }

    cubeset_vec
}

pub fn valid_game(cubesets: Vec<Cubeset>) -> bool {
    for cubeset in cubesets {
        if cubeset.amount > 12 && cubeset.color.eq("red") {
            return false;
        }
        if cubeset.amount > 13 && cubeset.color.eq("green") {
            return false;
        }
        if cubeset.amount > 14 && cubeset.color.eq("blue") {
            return false;
        }
    }

    true
}

pub fn sum_of_valid_games() -> usize {
    let lines = read_input_file().unwrap();
    let lines_iterator = lines.lines().enumerate();
    let mut sum = 0;

    for i in lines_iterator.clone() {
        let cutted_line =
            cut_prefix_string_and_delete_spaces(lines_iterator.clone().nth(i.0).unwrap().1);
        let cubeset_vec = store_vec_of_cubeset(i.0, cutted_line);
        if valid_game(cubeset_vec) {
            sum += i.0 + 1;
        }
    }

    return sum;
}

/* part2 */
pub fn get_vec_of_valid_games() -> Vec<Vec<Cubeset>> {
    let lines = read_input_file().unwrap();
    let lines_iterator = lines.lines().enumerate();
    let mut game_map = vec![];
    for i in lines_iterator.clone() {
        let cutted_line =
            cut_prefix_string_and_delete_spaces(lines_iterator.clone().nth(i.0).unwrap().1);
        let cubeset_vec = store_vec_of_cubeset(i.0, cutted_line);

        game_map.push(cubeset_vec);
    }
    game_map
}

pub fn sum_over_games_vec() -> usize {
    let game_map = get_vec_of_valid_games();

    let mut sum = 0;

    for games in game_map {
        let mut current_min_red = 0;
        let mut current_min_green = 0;
        let mut current_min_blue = 0;

        for v in games {
            if v.color.eq("red") && current_min_red < v.amount {
                current_min_red = v.amount;
            } else if v.color.eq("green") && current_min_green < v.amount {
                current_min_green = v.amount;
            } else if v.color.eq("blue") && current_min_blue < v.amount {
                current_min_blue = v.amount;
            }
        }

        sum += current_min_red * current_min_green * current_min_blue;
    }
    sum
}
