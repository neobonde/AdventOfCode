
#[cfg(test)]
mod tests;
mod advent_coins;

fn main() {
    let input = "yzbqklnj";
    let answer = advent_coins::mine(input,"000000");
    println!("answer: {answer}");
}
