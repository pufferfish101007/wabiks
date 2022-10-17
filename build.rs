use std::env;
use std::fs;
use std::path::Path;

fn perm_matrix_by_side(s: u8) -> [u8; 54] {
    //! Returns the permutation matrix for rotating this side anticlockwise
    match s {
        0 => [ // up
            02, 05, 08, 01, 04, 07, 00, 03, 06,
            36, 37, 38, 12, 13, 14, 15, 16, 17,
            09, 10, 11, 21, 22, 23, 24, 25, 26,
            18, 19, 20, 30, 31, 32, 33, 34, 35,
            27, 28, 29, 39, 40, 41, 42, 43, 44,
            45, 46, 47, 48, 49, 50, 51, 52, 53
        ],
        1  => [ // down
            00, 01, 02, 03, 04, 05, 06, 07, 08,
            09, 10, 11, 12, 13, 14, 24, 25, 26,
            18, 19, 20, 21, 22, 23, 33, 34, 35,
            27, 28, 29, 30, 31, 32, 42, 43, 44,
            36, 37, 38, 39, 40, 41, 15, 16, 17,
            47, 50, 53, 46, 49, 52, 45, 48, 51
        ],
        2 => [ // left
            18, 01, 02, 21, 04, 05, 24, 07, 08,
            11, 14, 17, 10, 13, 16, 08, 12, 15,
            45, 19, 20, 48, 22, 23, 51, 25, 26,
            27, 28, 29, 30, 31, 32, 33, 34, 35,
            36, 37, 06, 39, 40, 03, 42, 43, 00,
            44, 46, 47, 41, 49, 50, 38, 52, 53
        ],
        3 => [ // right
            00, 01, 42, 03, 04, 39, 06, 07, 36,
            09, 10, 11, 12, 13, 14, 15, 16, 17,
            18, 19, 02, 21, 22, 05, 24, 25, 08,
            29, 32, 35, 28, 31, 34, 27, 30, 33,
            47, 37, 38, 50, 40, 41, 53, 43, 44,
            45, 46, 20, 48, 49, 23, 51, 52, 26
        ],
        4 => [ // front
            00, 01, 02, 03, 04, 05, 27, 30, 33,
            09, 10, 08, 12, 13, 07, 15, 16, 06,
            20, 23, 26, 19, 22, 25, 18, 21, 24,
            47, 28, 29, 46, 31, 32, 45, 34, 35,
            36, 37, 38, 39, 40, 41, 42, 43, 44,
            11, 14, 17, 48, 49, 50, 51, 52, 53
        ],
        5 => [ // back
            15, 12, 09, 03, 04, 05, 06, 07, 08,
            02, 10, 11, 01, 13, 14, 00, 16, 17,
            18, 19, 20, 21, 22, 23, 24, 25, 26,
            27, 28, 53, 30, 31, 52, 33, 34, 51,
            38, 41, 44, 37, 40, 43, 36, 39, 42,
            45, 46, 47, 48, 49, 50, 35, 32, 29
        ],
        _ => unreachable!()
    }
}

fn multiply_matrix(a: [u8; 54], b: [u8; 54]) -> [u8; 54] {
    [
        a[b[0] as usize], a[b[1] as usize], a[b[2] as usize], a[b[3] as usize], a[b[4] as usize], a[b[5] as usize], a[b[6] as usize], a[b[7] as usize], a[b[8] as usize], a[b[9] as usize],
        a[b[10] as usize], a[b[11] as usize], a[b[12] as usize], a[b[13] as usize], a[b[14] as usize], a[b[15] as usize], a[b[16] as usize], a[b[17] as usize], a[b[18] as usize], a[b[19] as usize],
        a[b[20] as usize], a[b[21] as usize], a[b[22] as usize], a[b[23] as usize], a[b[24] as usize], a[b[25] as usize], a[b[26] as usize], a[b[27] as usize], a[b[28] as usize], a[b[29] as usize],
        a[b[30] as usize], a[b[31] as usize], a[b[32] as usize], a[b[33] as usize], a[b[34] as usize], a[b[35] as usize], a[b[36] as usize], a[b[37] as usize], a[b[38] as usize], a[b[39] as usize],
        a[b[40] as usize], a[b[41] as usize], a[b[42] as usize], a[b[43] as usize], a[b[44] as usize], a[b[45] as usize], a[b[46] as usize], a[b[47] as usize], a[b[48] as usize], a[b[49] as usize],
        a[b[50] as usize], a[b[51] as usize], a[b[52] as usize], a[b[53] as usize]
    ]
}

fn perm_matrix(s: u8, d: u8) -> [u8; 54]{
    let M = perm_matrix_by_side(s);
    match d {
        0 => M,
        1 => multiply_matrix(M, M),
        2 => multiply_matrix(multiply_matrix(M, M), M),
        _ => unreachable!()
    }
}

fn main() {
    let mut memswap_matrices: [String; 18] = Default::default();
    for side in 0..6 {
        for dir in 0..3 {
            let mut swap_len = 0;
            let mut last_diff = 55isize; // initialise to an impossible value so that things don't go wrong
            for (dest, src) in perm_matrix(side, dir).iter().enumerate() {
                println!("1");
                let this_diff = *src as isize - dest as isize;
                println!("2");
                if this_diff == last_diff {
                    swap_len += 1;
                } else {
                    println!("3");
                    if swap_len > 0 {
                        if (dest as isize) < *src as isize - swap_len as isize {
                            println!("3a");
                            memswap_matrices[(side * 3 + dir) as usize] += &format!("swap_nonoverlapping(ptr.add({:}), ptr.add({:}), {:});", dest - swap_len as usize, src - swap_len, swap_len)[..];
                        } else {
                            println!("3b");
                            for i in 0..swap_len {
                                memswap_matrices[(side * 3 + dir) as usize] += "panic!(\"overlapping thingies\");";
                            }
                        }
                    }
                    println!("4");
                    swap_len = 1;
                    last_diff = this_diff;
                }
            }
        }
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("move_matrices.rs");
    fs::write(
        &dest_path,
        format!("const fn perm_matrix(s: Side, d: Direction) -> PermutationMatrix {{
            use Side::*;
            use Direction::*;
            match (s, d) {{
                (Up, Anticlockwise) => {:?},
                (Up, Twice) => {:?},
                (Up, Clockwise) => {:?},
                (Down, Anticlockwise) => {:?},
                (Down, Twice) => {:?},
                (Down, Clockwise) => {:?},
                (Left, Anticlockwise) => {:?},
                (Left, Twice) => {:?},
                (Left, Clockwise) => {:?},
                (Right, Anticlockwise) => {:?},
                (Right, Twice) => {:?},
                (Right, Clockwise) => {:?},
                (Front, Anticlockwise) => {:?},
                (Front, Twice) => {:?},
                (Front, Clockwise) => {:?},
                (Back, Anticlockwise) => {:?},
                (Back, Twice) => {:?},
                (Back, Clockwise) => {:?},
            }}
        }}
        fn permute(facelets: &mut Facelets, s: Side, d: Direction) {{
            use std::ptr::swap_nonoverlapping;
            use Side::*;
            use Direction::*;
            let mut ptr = facelets.as_mut_ptr();
            unsafe {{
                match (s, d) {{
                    (Up, Anticlockwise) => {{{:}}},
                    (Up, Twice) => {{{:}}},
                    (Up, Clockwise) => {{{:}}},
                    (Down, Anticlockwise) => {{{:}}},
                    (Down, Twice) => {{{:}}},
                    (Down, Clockwise) => {{{:}}},
                    (Left, Anticlockwise) => {{{:}}},
                    (Left, Twice) => {{{:}}},
                    (Left, Clockwise) => {{{:}}},
                    (Right, Anticlockwise) => {{{:}}},
                    (Right, Twice) => {{{:}}},
                    (Right, Clockwise) => {{{:}}},
                    (Front, Anticlockwise) => {{{:}}},
                    (Front, Twice) => {{{:}}},
                    (Front, Clockwise) => {{{:}}},
                    (Back, Anticlockwise) => {{{:}}},
                    (Back, Twice) => {{{:}}},
                    (Back, Clockwise) => {{{:}}},
                }}
            }}
        }}",
        perm_matrix(0, 0),
        perm_matrix(0, 1),
        perm_matrix(0, 2),
        perm_matrix(1, 0),
        perm_matrix(1, 1),
        perm_matrix(1, 2),
        perm_matrix(2, 0),
        perm_matrix(2, 1),
        perm_matrix(2, 2),
        perm_matrix(3, 0),
        perm_matrix(3, 1),
        perm_matrix(3, 2),
        perm_matrix(4, 0),
        perm_matrix(4, 1),
        perm_matrix(4, 2),
        perm_matrix(5, 0),
        perm_matrix(5, 1),
        perm_matrix(5, 2),
        memswap_matrices[0],
        memswap_matrices[1],
        memswap_matrices[2],
        memswap_matrices[3],
        memswap_matrices[4],
        memswap_matrices[5],
        memswap_matrices[6],
        memswap_matrices[7],
        memswap_matrices[8],
        memswap_matrices[9],
        memswap_matrices[10],
        memswap_matrices[11],
        memswap_matrices[12],
        memswap_matrices[13],
        memswap_matrices[14],
        memswap_matrices[15],
        memswap_matrices[16],
        memswap_matrices[17],
        )
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}