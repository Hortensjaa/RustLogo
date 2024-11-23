use svg::{Document, node::element::Line};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use super::super::evaluator::turtle::Turtle;

pub fn save_image(turtle: Turtle, filename: &str) {
    // create new document
    let document = Document::new()
        .set("width", turtle.img_x)
        .set("height", turtle.img_y);

    // add all lines to document
    let mut document = document;
    for line in turtle.lines {
        let color = line.color.clone();
        let line_element = Line::new()
            .set("x1", line.start_x)
            .set("y1", line.start_y)
            .set("x2", line.end_x)
            .set("y2", line.end_y)
            .set("stroke", color)
            .set("stroke-width", 2);
        document = document.add(line_element);
    }

    // save file
    let path_str = format!("images/{}.svg", filename);
    let output_path = Path::new(path_str.as_str());
    let mut file = File::create(output_path).unwrap();
    file.write_all(document.to_string().as_bytes()).unwrap();
}
