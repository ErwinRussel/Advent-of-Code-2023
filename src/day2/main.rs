use std::fs;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    // Read file
    let file_path = "./input.txt";
    // let file_path = "./example.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    // Read line by line
    let mut sum = 0;

    // Read line by line
    for line in contents.lines() {
        let split: Vec<&str> = line.split(":").collect();

        let game_str = split[0];
        let game_str_list: Vec<&str> = game_str.split_whitespace().collect();
        let game_number = &game_str_list[1].parse::<i32>().unwrap();

        let rounds = split[1];
        let rounds_list: Vec<&str> = rounds.split(";").collect();

        let mut correct_game = true;

        'round_loop: for round in rounds_list {
            let cube_list: Vec<&str> = round.split(",").collect();

            for cube in cube_list {
                let cube_str_list: Vec<&str> = cube.split_whitespace().collect();
                let amount = cube_str_list[0].parse::<i32>().unwrap();
                let color = cube_str_list[1];

                let max_amount = match color{ 
                    "red"=>RED, 
                    "green"=>GREEN, 
                    "blue"=>BLUE,
                    &_ => todo!()
                };

                if amount > max_amount {
                    correct_game = false;
                    break 'round_loop;
                }
            }
        }

        if correct_game {
            sum += game_number;
        }
    }

    println!("{}", sum);
}
