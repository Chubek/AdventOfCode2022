use std::fs::read_to_string;

fn main() {
    let max_ration: u64 = read_to_string("calories.txt")
                        .expect("Could not read calories.txt")
                        .split("\n\n")
                        .map(|group_ration| {
                            group_ration
                                .split("\n")
                                .map(|x| x.parse::<u64>().expect("Could not parse number"))
                                .sum::<u64>()
                        })
                        .max()
                        .expect("Cound not get max");

    println!("{max_ration}");
}