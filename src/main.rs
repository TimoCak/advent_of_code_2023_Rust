mod day01;
mod day02;

fn main() {
    println!("day01: ⭐: {:?}", day01::take_numbers_of_vec_and_calc_sum());
    println!("day01: ⭐⭐: {:?}", day01::take_numbers_of_vec_and_calc_real_sum());
    println!("day02: ⭐⭐⭐: {}", day02::sum_of_valid_games());
    println!("day02: ⭐⭐⭐⭐: {}", day02::sum_over_games_vec());
}
