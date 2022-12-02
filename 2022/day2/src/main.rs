use std::fs::read_to_string;

fn main() {
    let data = read_file("./src/info.txt");

    println!("Answer is {}", part2(data))
}

fn read_file(path: &str) -> String {
    let file = read_to_string(path);
    return match file {
        Ok(t) => t,
        Err(error) => panic!("Could not read the file: {:?}", error),
    };
}

fn part1(data: String) -> u32 {
    let mut total: u32 = 0;

    for round in data.split("\n") {
        let arr: Vec<&str> = round.split(" ").collect();
        total += match arr[0] {
            "A" => { 
                match arr[1] {
                    "Y" => 6 +2, 
                    "X" => 3 +1, 
                    "Z" => 0 +3, 
                    _ => panic!("XYZ didn't match in A")
                }
            },
            "B" => { 
                match arr[1] {
                    "Z" => 6 +3,
                    "Y" => 3 +2,
                    "X" => 0 +1,
                    _ => panic!("XYZ didn't match in B")
                }
            },
            "C" => { 
                match arr[1] {
                    "X" => 6 +1,
                    "Z" => 3 +3,
                    "Y" => 0 +2,
                    _ => panic!("XYZ didn't match in A")
                }
            },
            _ => panic!("ABC didn't match")
        }
    }

    return total
}

fn part2(data: String) -> u32 {
    let mut total: u32 = 0;

    for round in data.split("\n") {
        let arr: Vec<&str> = round.split(" ").collect();
        total += match arr[0] {
            "A" => { // rock
                match arr[1] {
                    "Z" => 6 +2, 
                    "Y" => 3 +1, 
                    "X" => 0 +3, 
                    _ => panic!("XYZ didn't match in A")
                }
            },
            "B" => { // paper
                match arr[1] {
                    "Z" => 6 +3,
                    "Y" => 3 +2,
                    "X" => 0 +1,
                    _ => panic!("XYZ didn't match in B")
                }
            },
            "C" => { // scissors
                match arr[1] {
                    "Z" => 6 +1,
                    "Y" => 3 +3,
                    "X" => 0 +2,
                    _ => panic!("XYZ didn't match in A")
                }
            },
            _ => panic!("ABC didn't match")
        }
    }

    return total
}