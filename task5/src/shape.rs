#[derive(Copy, Clone, Debug)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Circle(Circle),
    Line(Line),
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}