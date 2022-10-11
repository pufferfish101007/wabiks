mod rubiks;

fn main() {
    println!("{}", rubiks::Rubiks::solved().move_side(rubiks::Side::Up, rubiks::Direction::Clockwise).move_side(rubiks::Side::Left, rubiks::Direction::Twice).facelets[0]);
}
