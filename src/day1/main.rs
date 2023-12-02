use std::fs;

fn main() {
    // Read file
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    // Read line by line
    // let mut result = Vec::new();
    let mut sum = 0;

    for line in contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left <= right {
            let left_char = chars[left];
            let right_char = chars[right];

            if left_char.is_numeric() {
                sum += 10 * left_char.to_digit(10).unwrap()
            } else {
                left += 1;
            }

            if right_char.is_numeric() {
               sum += right_char.to_digit(10).unwrap()
            } else {
                right -= 1;
            }
        }

        break;

        // result.push(line.to_string())
    }

    // println!("{:?}", result)
    println!("{}", sum);
}
