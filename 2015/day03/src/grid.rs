use std::{collections::HashMap, ops::Add};


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

#[derive(Clone)]
pub struct Grid {
    locations : HashMap<Position,i32>,
    santa_location : Position,
    robo_location : Position,
}


impl Grid {
    pub fn build(start: i32) -> Grid{
        let mut res = Grid {
            locations: HashMap::new(),
            santa_location: Position { x: 0, y: 0 },
            robo_location: Position { x: 0, y: 0 },
        };
        res.locations.insert(Position { x: 0, y: 0 }, start);

        return res
    }

    pub fn get_presents(&self, x: i32, y: i32) -> i32 {
        match self.locations.get(&Position { x: x, y: y }) {
            None => 0,
            Some(presents) => *presents,
        }
    }

    pub fn houses_visited(&self) -> i32 {
        self.locations.keys().len() as i32
    }


    pub fn translate(&mut self, direction: char, santa: bool){
        match direction {
            '^' => { self.update_grid(Position { x: 0, y: 1 },santa)}
            'v' => { self.update_grid(Position { x: 0, y: -1 },santa)}
            '>' => { self.update_grid(Position { x: 1, y: 0 },santa)}
            '<' => { self.update_grid(Position { x: -1, y: 0 },santa)}
            _ => panic!("Unknown command")
        }
    }

    fn update_grid(&mut self, offset: Position, santa: bool){
        if santa {
            self.santa_location = self.santa_location + offset;
            *self.locations.entry(self.santa_location).or_insert(0) += 1;
        }
        else {
            self.robo_location = self.robo_location + offset;
            *self.locations.entry(self.robo_location).or_insert(0) += 1;
        }
    }

    pub fn follow_instructions(&mut self, instructions: &String)
    {
        for instruction in instructions.chars() {
            self.translate(instruction, true);
        }
    }

    pub fn follow_instructions_with_robot(&mut self, instructions: &String){
        for (i, instruction) in instructions.chars().enumerate() {
            self.translate(instruction, i%2 == 0);
        }
    }

}