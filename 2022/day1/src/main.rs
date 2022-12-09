use std::fs::read_to_string;

fn main() {
    println!("{}", part1("src/info.txt"));
    println!("{}", part2("src/info.txt"));
}

fn read_file(path: &str) -> String {
    let file = read_to_string(path);
    return match file {
        Ok(t) => t,
        Err(error) => panic!("Could not read the file: {:?}", error),
    };
}

fn part1(path: &str) -> u32 {
    let mut ans: u32 = 0;
    let data = read_file(path);

    // Finding the highest calorie backpack.
    for ration in data.split("\n\n") {
        let mut backpack: u32 = 0;

        for calories in ration.split("\n") {
            backpack += match calories.trim().parse::<u32>() {
                Ok(num) => num,
                Err(error) => panic!("Parse error: {}", error)
            };
        };

        if backpack > ans {
            ans = backpack;
        };
    };

    return ans
}

fn part2(path: &str) -> u32 {
    let mut top3: Vec<u32> = vec![0, 0, 0];
    let data = read_file(path);

    // Finding the top 3 highest calorie backpack.
    for ration in data.split("\n\n") {
        let mut backpack: u32 = 0;
        
        for calories in ration.split("\n") {
            backpack += match calories.trim().parse::<u32>() {
                Ok(num) => num,
                Err(error) => panic!("Parse error: {}", error)
            };
        };

        for i in 0..3 {
            if backpack > top3[i] {
                top3[i] = backpack;
                top3.sort();
                break
            };
        };
    };

    return top3.into_iter().reduce(|a, b| a+b).unwrap();
}