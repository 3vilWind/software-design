use crate::drawing::DrawingApi;
use crate::shape::{Circle, Line, Point, Size};

pub trait Graph {
    fn draw(self: Box<Self>);
    // panic on incorrect data
    fn read(drawing: Box<dyn DrawingApi>, data: &str) -> Self where Self: Sized;
}

pub struct EdgesListGraph {
    drawing: Box<dyn DrawingApi>,
    data: Vec<Vec<u32>>,
}

impl Graph for EdgesListGraph {
    fn draw(mut self: Box<Self>) {
        let count = self.data.len() as u32;
        for (from, v) in self.data.iter().enumerate() {
            self.drawing.draw_vertex(count, from as u32);

            for to in v {
                self.drawing.draw_edge(count, from as u32, *to);
            }
        }

        self.drawing.blocking_run();
    }

    fn read(drawing: Box<dyn DrawingApi>, data: &str) -> Self {
        let lines: Vec<&str> = data.lines().collect();
        let (first, others) = lines.split_at(1);
        let n = first[0].parse::<u32>().unwrap();
        let mut data = Vec::with_capacity(n as usize);
        for _ in 0..n {
            data.push(Vec::new());
        }
        for line in others {
            let dat: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
            let (from, to) = (*dat.get(0).unwrap(), *dat.get(1).unwrap());
            data.get_mut(from as usize).unwrap().push(to);
        }
        Self { drawing, data }
    }
}

pub struct AdjacencyMatrixGraph {
    drawing: Box<dyn DrawingApi>,
    data: Vec<Vec<bool>>,
}

impl Graph for AdjacencyMatrixGraph {
    fn draw(mut self: Box<Self>) {
        let count = self.data.len() as u32 + 1;
        for i in 0..count {
            self.drawing.draw_vertex(count, i);
        }
        for i in 1..count {
            for j in 0..i {
                if *self.data.get((i - 1) as usize).unwrap().get(j as usize).unwrap() {
                    self.drawing.draw_edge(count, i, j);
                }
            }
        }

        self.drawing.blocking_run();
    }

    fn read(drawing: Box<dyn DrawingApi>, data: &str) -> Self {
        let mut dat = Vec::with_capacity(data.len() as usize + 1);
        for line in data.lines() {
            dat.push(line.split_whitespace().map(|x| x == "1").collect());
        }
        Self { drawing, data: dat }
    }
}

impl dyn DrawingApi {
    fn draw_vertex(&mut self, count: u32, current: u32) {
        let pos = get_vertex_position(self.size(), count, current);
        self.draw_circle(Circle { x: pos.x, y: pos.y, radius: get_vertex_radius(self.size(), count) })
    }

    fn draw_edge(&mut self, count: u32, from: u32, to: u32) {
        let from_pos = get_vertex_position(self.size(), count, from);
        let to_pos = get_vertex_position(self.size(), count, to);
        self.draw_line(Line { from: from_pos, to: to_pos });
    }
}

fn get_vertex_radius(size: Size, count: u32) -> f32 {
    let full_radius = size.width as f32 / 2.5;
    let full_length = 2.0 * std::f32::consts::PI * full_radius;

    full_length / (count as f32 * (2.0 + 1.0))
}

fn get_vertex_position(size: Size, count: u32, current: u32) -> Point {
    let t = 2.0 * std::f64::consts::PI * (current as f64) / (count as f64);
    Point {
        x: scale(t.sin(), size.width as f64) + (size.width as f32 / 2.0) as i32,
        y: scale(t.cos(), size.height as f64) + (size.height as f32 / 2.0) as i32,
    }
}

fn scale(x: f64, bord: f64) -> i32 {
    (x * (bord / 2.5)) as i32
}

struct MockDrawingApi;

impl DrawingApi for MockDrawingApi {
    fn draw_circle(&mut self, _circle: Circle) {}
    fn draw_line(&mut self, _line: Line) {}
    fn blocking_run(self: Box<Self>) {}

    fn size(&self) -> Size {
        Size { width: 400, height: 400 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges_list_read() {
        let data = "3\n0 1\n0 2";
        let result = EdgesListGraph::read(Box::new(MockDrawingApi {}), data);
        assert_eq!(result.data, vec![vec![1, 2], vec![], vec![]]);
    }

    #[test]
    fn adjacency_matrix_read() {
        let data = "0\n1 1\n0 0 0";
        let result = AdjacencyMatrixGraph::read(Box::new(MockDrawingApi {}), data);
        assert_eq!(result.data, vec![vec![false], vec![true, true], vec![false, false, false]]);
    }
}
