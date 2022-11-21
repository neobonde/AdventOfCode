use std::fs;

#[cfg(test)]
mod tests;
mod grid;


fn main() {
    let instructions = fs::read_to_string("input.txt").unwrap();

    let mut house_grid = grid::Grid::build(2);
    house_grid.follow_instructions_with_robot(&instructions);
    let houses = house_grid.houses_visited();

    println!("{houses}");

}
