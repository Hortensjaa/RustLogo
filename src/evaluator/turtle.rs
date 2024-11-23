#[derive(Debug, Clone)]
pub struct Turtle {
    pub img_x: f64,
    pub img_y: f64,
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub pen_down: bool,
    pub pen_color: String,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub color: String
}

impl Turtle {
    pub fn new(img_x: f64, img_y: f64) -> Self {
        Turtle {
            img_x: img_x,
            img_y: img_y,
            x: img_x / 2.0,  // start at the center of the screen
            y: img_y / 2.0,
            angle: 90.0,      // head up :)
            pen_down: true,
            pen_color: "black".to_string(),
            lines: Vec::new(), // lines history
        }
    }

    pub fn rotate(&mut self, direction: &str, degrees: f64) {
        if direction == "left" {
            self.angle += degrees;
        } else if direction == "right" {
            self.angle -= degrees;
        } else {
            eprintln!("rotate: Wrong direction");
        }
    }

    pub fn go(&mut self, direction: &str, distance: f64) {
        let radian_angle = self.angle.to_radians();
        let (new_x, new_y);

        if direction == "forward" {
            new_x = self.x + distance * radian_angle.cos();
            new_y = self.y + distance * radian_angle.sin();
        } else if direction == "back" {
            new_x = self.x - distance * radian_angle.cos();
            new_y = self.y - distance * radian_angle.sin();
        } else {
            eprintln!("go: Wrong direction");
            return;
        }

        if self.pen_down {
            self.lines.push(Line {
                start_x: self.x,
                start_y: self.y,
                end_x: new_x,
                end_y: new_y,
                color: self.pen_color.clone()
            });
        }

        self.x = new_x;
        self.y = new_y;
    }

    pub fn move_pen(&mut self, direction: &str) {
        if direction == "up" {
            self.pen_down = false;
        } else if direction == "down" {
            self.pen_down = true;
        } else {
            eprintln!("move_pen: Wrong direction");
        }
    }

    pub fn clear_screen(&mut self) {
        self.lines.clear();
    }

    pub fn change_color(&mut self, color: &str) {
        let allowed_colors = ["red", "green", "blue", "black", "white", "purple"];
        if allowed_colors.contains(&color.to_lowercase().as_str()) {
            self.pen_color = color.to_string();
        } else {
            eprintln!("Error: Invalid color '{}'. Allowed colors are: {:?}", color, allowed_colors);
        }
    }

}
