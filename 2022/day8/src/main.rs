use std::fs::read_to_string;

fn main() {
    let data = read_file("./src/info.txt");

    println!("Part 1 answer is {}.", part1(data.clone()));
    println!("Part 2 answer is {}.", part2(data.clone()));
}

fn read_file(path: &str) -> String {
    let file = read_to_string(path);
    return match file {
        Ok(t) => t,
        Err(error) => panic!("Could not read the file: {:?}", error),
    };
}

fn process_data(data: String) -> Vec<Vec<u8>> {
    let mut ans: Vec<Vec<u8>> = Vec::new();

    for row in data.split("\n") {
        let mut temp: Vec<u8> = Vec::new();
        for tree in row.chars() {
            temp.push(tree.to_string().parse::<u8>().unwrap())
        }
        ans.push(temp)
    }

    return ans
}

fn part1(raw_data: String) -> u32 {
    let mut ans: u32 = 0;
    let data = process_data(raw_data);

    for i in 1..data.len()-1 {
        for j in 1..data[i].len()-1 {
            let tree = data[i][j];
            let mut count = 0;

            // left
            for index in 0..j {
                if data[i][index] >= tree {
                    count += 1;
                    break
                }
            }
            // right
            for index in j+1..data[i].len() {
                if data[i][index] >= tree {
                    count += 1;
                    break
                }
            }

            // up
            for index in 0..i {
                if data[index][j] >= tree {
                    count += 1;
                    break
                }
            }
            // down
            for index in i+1..data.len() {
                if data[index][j] >= tree {
                    count += 1;
                    break
                }
            }

            if count < 4 { 
                ans += 1 
            }
        }
    }

    ans += (data.len()*2 + data[0].len()*2 -4) as u32;

    return ans
}

fn part2(raw_data: String) -> u32 {
    let mut ans: u32 = 0;
    let data = process_data(raw_data);

    for i in 1..data.len()-1 {
        for j in 1..data[i].len()-1 {
            let tree = data[i][j];
            let mut count = 0;
            let mut value = 1;

            // left
            for index in (0..j).rev() {
                count += 1;
                if data[i][index] >= tree {
                    break
                }
            }
            value *= count;
            count = 0;
            // right
            for index in j+1..data[i].len() {
                count += 1;
                if data[i][index] >= tree {
                    break
                }
            }
            value *= count;
            count = 0;
            // up
            for index in (0..i).rev() {
                count += 1;
                if data[index][j] >= tree {
                    break
                }
            }
            value *= count;
            count = 0;
            // down
            for index in i+1..data.len() {
                count += 1;
                if data[index][j] >= tree {
                    break
                }
            }
            value *= count;

            if value > ans { 
                ans = value 
            }
        }
    }

    return ans
}
