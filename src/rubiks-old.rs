/*
    what a rubiks cube looks like in terms of edge/corner numbers
    facelets with captial letters are where cubie orientations are based upon

                 ------------
                | C1  E0  C2 |
                | e3   U  e1 |
                | C0  E2  C3 |
     ----------- ------------ ---------- -----------
    | c1 E3  c0 | c0  e2  c3 | c3 E1 c2 | c2 e0  c1 |
    | e7  L  e4 | E4   F  E5 | e5 R  e6 | E6  B  E7 |
    | c7 E11 c4 | c4  e8  c5 | c5 E9 c6 | c6 e10 c7 |
     ----------- ------------ ---------- -----------
                | C4  E8  C5 |
                | e11  B  e9 |
                | C7  E10 C6 |
                 ------------
*/

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

pub struct Cube (
    /*
    One u128 stores the entire cube's state.
    Orientations & permutations are stored in terms of places where cubies should be in relation to where they are,
    as opposed to cubies relative to their solved position.
    This makes calculating orientation (& permutation?) easier.
    A solved cube has state 0.
    A cube in G1 has corner orientations = 0, edge orientations 4, 5, 6 & 7 = 0, and UD slice = 0
    
    The first 60 least significant bits represent edge orientation & permutation - 1 bit per cubie.
      The least significant bit of each cubie represents orientation:
      0 -> Ex-Ey, ey-ex
      1 -> Ex-ey, Ey-ex
      
      The 4 most significant bits of each cubie represent edge permutation - 4 bits per cubie:
      0 -> in solved permutation
      n -> solved permutation + n (modulo 12)
      So e8 in the solved position of e2 has permutation 8,
      and e3 in the solved position of e6 has permutation 3
    
    The next 16 least significant bits represent corner orientation - 2 bits per cubie:
    0b00 -> solved position
    0b01 -> rotated clockwise
    0b10 -> rotated Anticlockwise
    
    The next 24 least significant bits represent corner permutation - 3 bits per cubie.
    These work in the same manner as edge orientation, but modulo 8 rather than 12.
    
    The next 4 least sugnificant bits represent whether the UD slice cubies are in the UD slice or not.
    From least significant to most, the bits represent e4, e5, e6 & e7.
    0 = in UD slice
    1 = not in UD slice
    UD slice could be calculated from the edge permutations, but this should be more efficient™.
    This is needed to check whether we're in G1 or not.
    
    And we still have 24 spare bits!
    (if only unsigned 104-bit integers existed)
    */
    u128
)

use Direction::*;
use Side::*;

impl Cube {
    fn new_solved() -> Self {
        Cube(0)
    }
    
    fn is_solved(&self) -> bool {
        self.0 == 0
    }
    
    fn is_g1(&self) -> bool {
        self.0 & 0b11110000000000000000000000001111111111111111000000000000000000000000000000000000000000000000111111111111 == 0
    }

    #[inline]
    fn turn_side_edge_ori(&self, side: Side, dir: Direction) {
        match dir {
            Twice => (),
            Clockwise | Anticlockwise => {
                self.0 ^= match side {
                    Up => 0b000000001111,
                    Down => 0b111100000000,
                    Left => 0b100010011000,
                    Right => 0b001001100010,
                    Front => 0b000100110100,
                    Back => 0b010011000001,
                };
            }
        };
    }
    
    #[inline]
    fn turn_side_ud_slice(&self, side: Side, dir: Direction) {
        match dir {
            Twice => {},
            Clockwise => {},
            Anticlockwise => {},
        };
    }

    #[inline]
    fn turn_side_corner_ori(&self, side: Side, dir: Direction) {
        match dir {
            Twice => {},
            Clockwise => {},
            Anticlockwise => {},
        };
    }

    pub fn turn_side(&self, side: Side, dir: Direction) {
        self.turn_side_edge_ori(side, dir);
        self.turn_side_corner_ori(side, dir);
    }
}
