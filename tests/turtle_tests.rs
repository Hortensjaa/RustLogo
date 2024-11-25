use lista7::evaluator::turtle::Turtle;

#[test]
fn test_turtle_creation() {
    let turtle = Turtle::new(800.0, 600.0);
    assert_eq!(turtle.x, 400.0);
    assert_eq!(turtle.y, 300.0);
    assert_eq!(turtle.angle, 270.0);
    assert_eq!(turtle.pen_down, true);
    assert_eq!(turtle.lines.len(), 0);
}

#[test]
fn test_rotate_right() {
    let mut turtle = Turtle::new(800.0, 600.0);
    turtle.rotate("right", 90.0);
    assert_eq!(turtle.angle, 360.0);

    turtle.go("forward", 100.0);
    let line = &turtle.lines[0];
    assert_eq!(line.start_x, 400.0);
    assert_eq!(line.start_y, 300.0);
    assert_eq!(line.end_x, 400.0 + 100.0);
    assert_eq!(line.end_y, 300.0);
}

#[test]
fn test_rotate_left() {
    let mut turtle = Turtle::new(800.0, 600.0);
    turtle.rotate("left", 90.0);
    assert_eq!(turtle.angle, 180.0);

    turtle.go("forward", 100.0);
    let line = &turtle.lines[0];
    assert_eq!(line.start_x, 400.0);
    assert_eq!(line.start_y, 300.0);
    assert_eq!(line.end_x, 400.0 - 100.0);
    assert_eq!(line.end_y, 300.0);
}

#[test]
fn test_move_forward() {
    let mut turtle = Turtle::new(800.0, 600.0);
    turtle.go("forward", 100.0);

    assert_eq!(turtle.x, 400.0); 
    assert_eq!(turtle.y, 300.0 - 100.0);
    
    assert_eq!(turtle.lines.len(), 1);
    let line = &turtle.lines[0];
    assert_eq!(line.start_x, 400.0);
    assert_eq!(line.start_y, 300.0);
    assert_eq!(line.end_x, 400.0);
    assert_eq!(line.end_y, 300.0 - 100.0);
}

#[test]
fn test_move_back() {
    let mut turtle = Turtle::new(800.0, 600.0);
    turtle.go("back", 50.0);

    assert_eq!(turtle.x, 400.0); 
    assert_eq!(turtle.y, 300.0 + 50.0);

    assert_eq!(turtle.lines.len(), 1);
    let line = &turtle.lines[0];
    assert_eq!(line.start_x, 400.0);
    assert_eq!(line.start_y, 300.0);
    assert_eq!(line.end_x, 400.0);
    assert_eq!(line.end_y, 300.0 + 50.0);
}

#[test]
fn test_pen_up_and_down() {
    let mut turtle = Turtle::new(800.0, 600.0);
    assert_eq!(turtle.pen_down, true);
    turtle.move_pen("up");
    assert_eq!(turtle.pen_down, false);
    turtle.move_pen("down");
    assert_eq!(turtle.pen_down, true);
}

#[test]
fn test_clear_screen() {
    let mut turtle = Turtle::new(800.0, 600.0);
    turtle.go("back", 50.0);
    turtle.go("forward", 250.0);

    assert_eq!(turtle.lines.len(), 2);

    turtle.clear_screen();

    assert_eq!(turtle.lines.len(), 0);
}
