enum Side {
    LW,
    WH,
    HL,
}

#[derive(Debug)]
pub struct Present {
    length: i32,
    width: i32,
    height: i32
}

impl Present {

    pub fn new(length: i32, width :i32, height: i32) -> Present
    {
        Present { length: length, width: width, height: height }
    }

    fn area(&self, side: Side) -> i32 {
        let area = match side {
            Side::LW => self.length*self.width,
            Side::WH => self.width*self.height,
            Side::HL => self.height*self.length,
        };
        area
    }

    fn perimeter(&self, side: Side) ->i32 {
        let perimeter = match side {
            Side::LW => 2 * self.length + 2* self.width,
            Side::WH => 2 * self.width + 2* self.height,
            Side::HL => 2 * self.height + 2* self.length,
        };
        perimeter
    }

    fn volume(&self) -> i32 {
        self.length * self.width * self.height
    }

    fn smallest_perimeter(&self) -> i32 {
        let v = vec![self.perimeter(Side::LW),self.perimeter(Side::WH),self.perimeter(Side::HL)];
        let min_preimeter = *v.iter().min().unwrap();
        min_preimeter
    }

    fn smallest_area(&self) -> i32 {
        let v = vec![self.area(Side::LW),self.area(Side::WH),self.area(Side::HL)];
        let min_area = *v.iter().min().unwrap();
        min_area
    }

    fn surface_area(&self) -> i32 {
          2 * self.area(Side::LW)
        + 2 * self.area(Side::WH) 
        + 2 * self.area(Side::HL)
    }

    pub fn wrapping_paper_area(&self) -> i32 {
        self.surface_area() + self.smallest_area()
    }

    pub fn ribbon_length(&self) -> i32 {
        self.smallest_perimeter() + self.volume()
    }

}