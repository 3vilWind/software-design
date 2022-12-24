use clap::{Parser, ValueEnum};

use crate::drawing::{DrawingApi, EGuiDrawingApi, FltkDrawingApi};
use crate::graph::{AdjacencyMatrixGraph, EdgesListGraph, Graph};
use crate::shape::Size;

mod drawing;
mod graph;
mod shape;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    graph_type: GraphType,
    #[arg(value_enum)]
    gui_type: GuiType,
    #[arg(short, long)]
    file: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GraphType {
    EdgesList,
    AdjacencyMatrix,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GuiType {
    EGui,
    Fltk,
}

fn main() {
    let args = Args::parse();

    let contents = std::fs::read_to_string(args.file)
        .expect("Should have been able to read the file");

    let size = Size { width: 400, height: 400 };
    let drawing_api: Box<dyn DrawingApi> = match args.gui_type {
        GuiType::Fltk => Box::new(FltkDrawingApi::new(size)),
        GuiType::EGui => Box::new(EGuiDrawingApi::new(size)),
    };
    let graph: Box<dyn Graph> = match args.graph_type {
        GraphType::EdgesList =>
            Box::new(EdgesListGraph::read(drawing_api, &contents)),
        GraphType::AdjacencyMatrix =>
            Box::new(AdjacencyMatrixGraph::read(drawing_api, &contents)),
    };
    graph.draw();
}
