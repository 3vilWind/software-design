use eframe::egui;
use fltk::enums::Color;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

use crate::shape::{Circle, Line, Shape, Size};

pub trait DrawingApi {
    fn draw_circle(&mut self, circle: Circle);
    fn draw_line(&mut self, line: Line);
    fn blocking_run(self: Box<Self>);

    fn size(&self) -> Size;
}


struct MyApp {
    shapes: Vec<egui::Shape>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for x in self.shapes.iter() {
                ui.painter().add(x.clone());
            }
        });
    }
}

pub struct EGuiDrawingApi {
    shapes: Vec<egui::Shape>,
    size: Size,
}

impl EGuiDrawingApi {
    pub fn new(size: Size) -> Self {
        Self { shapes: vec![], size }
    }
}

impl DrawingApi for EGuiDrawingApi {
    fn draw_circle(&mut self, circle: Circle) {
        self.shapes.push(egui::Shape::circle_stroke(
            egui::Pos2::new(circle.x as f32, circle.y as f32),
            circle.radius,
            egui::Stroke::new(2.0, egui::Color32::RED))
        );
    }

    fn draw_line(&mut self, line: Line) {
        self.shapes.push(egui::Shape::line_segment(
            [egui::Pos2::new(line.from.x as f32, line.from.y as f32), egui::Pos2::new(line.to.x as f32, line.to.y as f32)],
            egui::Stroke::new(2.0, egui::Color32::RED))
        );
    }

    fn blocking_run(self: Box<Self>) {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(self.size.width as f32, self.size.height as f32)),
            ..Default::default()
        };
        // let data = self.shapes.clone();
        eframe::run_native(
            "Graph Visualization",
            options,
            Box::new(|_cc| Box::new(MyApp { shapes: self.shapes })));
    }

    fn size(&self) -> Size {
        self.size.clone()
    }
}

pub struct FltkDrawingApi {
    shapes: Vec<Shape>,
    size: Size,
}

impl FltkDrawingApi {
    pub fn new(size: Size) -> Self {
        Self { shapes: vec![], size }
    }
}

impl DrawingApi for FltkDrawingApi {
    fn draw_circle(&mut self, circle: Circle) {
        self.shapes.push(Shape::Circle(circle));
    }

    fn draw_line(&mut self, line: Line) {
        self.shapes.push(Shape::Line(line));
    }

    fn blocking_run(self: Box<Self>) {
        let app = fltk::app::App::default();
        let mut wind = fltk::window::Window::new(100, 100,
                                                 self.size.width as i32, self.size.height as i32,
                                                 "Graph Visualization");
        let mut frame = fltk::frame::Frame::default().size_of(&wind);
        wind.end();
        wind.show();
        frame.draw(move |_| {
            fltk::draw::set_draw_color(Color::Red);
            for sh in self.shapes.iter() {
                match sh {
                    Shape::Circle(circle) => fltk::draw::draw_circle(circle.x as f64, circle.y as f64, circle.radius as f64),
                    Shape::Line(line) => fltk::draw::draw_line(line.from.x, line.from.y, line.to.x, line.to.y)
                }
            }
        });

        app.run().unwrap();
    }

    fn size(&self) -> Size {
        self.size.clone()
    }
}