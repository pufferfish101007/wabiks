pub type Facelets = [u8; 54];
pub type PermutationMatrix = [u8; 54];

#[derive(Debug)]
pub struct Rubiks {
    pub facelets: Facelets,
}

pub enum Side {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

pub enum Direction {
    Clockwise,
    Anticlockwise,
    Twice,
}

include!(concat!(env!("OUT_DIR"), "/move_matrices.rs"));

/*const fn perm_matrix_by_side(s: Side) -> PermutationMatrix {
    //! Returns the permutation matrix for rotating this side anticlockwise
    match s {
        Side::Up => [
            02, 05, 08, 01, 04, 07, 00, 03, 06,
            36, 37, 38, 12, 13, 14, 15, 16, 17,
            09, 10, 11, 21, 22, 23, 24, 25, 26,
            18, 19, 20, 30, 31, 32, 33, 34, 35,
            27, 28, 29, 39, 40, 41, 42, 43, 44,
            45, 46, 47, 48, 49, 50, 51, 52, 53
        ],
        Side::Down  => [
            00, 01, 02, 03, 04, 05, 06, 07, 08,
            09, 10, 11, 12, 13, 14, 24, 25, 26,
            18, 19, 20, 21, 22, 23, 33, 34, 35,
            27, 28, 29, 30, 31, 32, 42, 43, 44,
            36, 37, 38, 39, 40, 41, 15, 16, 17,
            47, 50, 53, 46, 49, 52, 45, 48, 51
        ],
        Side::Left => [
            18, 01, 02, 21, 04, 05, 24, 07, 08,
            11, 14, 17, 10, 13, 16, 08, 12, 15,
            45, 19, 20, 48, 22, 23, 51, 25, 26,
            27, 28, 29, 30, 31, 32, 33, 34, 35,
            36, 37, 06, 39, 40, 03, 42, 43, 00,
            44, 46, 47, 41, 49, 50, 38, 52, 53,
        ],
        _ => todo!()
    }
}*/
fn multiply_matrix(a: Facelets, b: PermutationMatrix) -> PermutationMatrix {
    unsafe {[
        *a.get_unchecked(*b.get_unchecked(0) as usize), *a.get_unchecked(*b.get_unchecked(1) as usize), *a.get_unchecked(*b.get_unchecked(2) as usize), *a.get_unchecked(*b.get_unchecked(3) as usize), *a.get_unchecked(4), *a.get_unchecked(*b.get_unchecked(5) as usize), *a.get_unchecked(*b.get_unchecked(6) as usize), *a.get_unchecked(*b.get_unchecked(7) as usize), *a.get_unchecked(*b.get_unchecked(8) as usize), *a.get_unchecked(*b.get_unchecked(9) as usize),
        *a.get_unchecked(*b.get_unchecked(10) as usize), *a.get_unchecked(*b.get_unchecked(11) as usize), *a.get_unchecked(*b.get_unchecked(12) as usize), *a.get_unchecked(13), *a.get_unchecked(*b.get_unchecked(14) as usize), *a.get_unchecked(*b.get_unchecked(15) as usize), *a.get_unchecked(*b.get_unchecked(16) as usize), *a.get_unchecked(*b.get_unchecked(17) as usize), *a.get_unchecked(*b.get_unchecked(18) as usize), *a.get_unchecked(*b.get_unchecked(19) as usize),
        *a.get_unchecked(*b.get_unchecked(20) as usize), *a.get_unchecked(*b.get_unchecked(21) as usize), *a.get_unchecked(22), *a.get_unchecked(*b.get_unchecked(23) as usize), *a.get_unchecked(*b.get_unchecked(24) as usize), *a.get_unchecked(*b.get_unchecked(25) as usize), *a.get_unchecked(*b.get_unchecked(26) as usize), *a.get_unchecked(*b.get_unchecked(27) as usize), *a.get_unchecked(*b.get_unchecked(28) as usize), *a.get_unchecked(*b.get_unchecked(29) as usize),
        *a.get_unchecked(*b.get_unchecked(30) as usize), *a.get_unchecked(31), *a.get_unchecked(*b.get_unchecked(32) as usize), *a.get_unchecked(*b.get_unchecked(33) as usize), *a.get_unchecked(*b.get_unchecked(34) as usize), *a.get_unchecked(*b.get_unchecked(35) as usize), *a.get_unchecked(*b.get_unchecked(36) as usize), *a.get_unchecked(*b.get_unchecked(37) as usize), *a.get_unchecked(*b.get_unchecked(38) as usize), *a.get_unchecked(*b.get_unchecked(39) as usize),
        *a.get_unchecked(40), *a.get_unchecked(*b.get_unchecked(41) as usize), *a.get_unchecked(*b.get_unchecked(42) as usize), *a.get_unchecked(*b.get_unchecked(43) as usize), *a.get_unchecked(*b.get_unchecked(44) as usize), *a.get_unchecked(*b.get_unchecked(45) as usize), *a.get_unchecked(*b.get_unchecked(46) as usize), *a.get_unchecked(*b.get_unchecked(47) as usize), *a.get_unchecked(*b.get_unchecked(48) as usize), *a.get_unchecked(49),
        *a.get_unchecked(*b.get_unchecked(50) as usize), *a.get_unchecked(*b.get_unchecked(51) as usize), *a.get_unchecked(*b.get_unchecked(52) as usize), *a.get_unchecked(*b.get_unchecked(53) as usize)
    ]}
}

/*const fn perm_matrix(s: Side, d: Direction) -> PermutationMatrix{
    let M: PermutationMatrix = perm_matrix_by_side(s);
    match d {
        Direction::Anticlockwise => M,
        Direction::Twice => multiply_matrix(M, M),
        Direction::Clockwise => multiply_matrix(multiply_matrix(M, M), M),
    }
}*/

impl Rubiks {
    pub fn new(facelets: Facelets) -> Self {
        Rubiks { facelets }
    }
    pub fn solved() -> Self {
        Self::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5
        ])
    }
    pub fn move_side(&self, s: Side, d: Direction) -> Self {
        Self::new(multiply_matrix(self.facelets, perm_matrix(s, d)))
    }
    pub fn ms(&mut self, s: Side, d: Direction) {
        permute(&mut self.facelets, s, d);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move() {
        use Side::*;
        use Direction::*;
        let mut r = Rubiks::solved();
        r.ms(Up, Anticlockwise);
        r.ms(Up, Twice);
        r.ms(Up, Clockwise);
        r.ms(Down, Anticlockwise);
        r.ms(Down, Twice);
        r.ms(Down, Clockwise);
        r.ms(Left, Anticlockwise);
        r.ms(Left, Twice);
        r.ms(Left, Clockwise);
        r.ms(Right, Anticlockwise);
        r.ms(Right, Twice);
        r.ms(Right, Clockwise);
        r.ms(Front, Anticlockwise);
        r.ms(Front, Twice);
        r.ms(Front, Clockwise);
        r.ms(Back, Anticlockwise);
        r.ms(Back, Twice);
        r.ms(Back, Clockwise);
    }
}