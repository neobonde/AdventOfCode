use naughty_or_nice::check_list;


#[cfg(test)]
mod tests;
mod naughty_or_nice;

fn main() {
    let nice_count = check_list("input.txt");

    println!("Nice: {nice_count}");
}
