use super::grid;

#[test]
fn move_east(){
    assert_eq!(test_santa(">"),2);
}

#[test]
fn move_square() {
    assert_eq!(test_santa("^>v<"),4);
}

#[test]
fn move_back_forth() {
    assert_eq!(test_santa("^v^v^v^v^v"),2);
}

#[test]
fn robo_move_up_down(){
    assert_eq!(test_santa_robo("^v"),3);
}

#[test]
fn robo_move_square() {
    assert_eq!(test_santa_robo("^>v<"),3);
}

#[test]
fn robo_move_back_forth() {
    assert_eq!(test_santa_robo("^v^v^v^v^v"),11);
}


fn test_santa(input: &str) -> i32{
    let instructions = input;

    let mut house_grid = grid::Grid::build(1);
    house_grid.follow_instructions(&instructions.to_string());

    house_grid.houses_visited()
}


fn test_santa_robo(input: &str) -> i32{
    let instructions = input;

    let mut house_grid = grid::Grid::build(2);
    house_grid.follow_instructions_with_robot(&instructions.to_string());

    house_grid.houses_visited()
}