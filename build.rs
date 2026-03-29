use std::env;
use std::fmt::Write;
use std::fs;
use std::path::Path;

// Corner connections: the 3 cube-edge-adjacent corners for each corner 0-7.
// Corner layout:
//     6---7
//    /|  /|
//   4---5 |
//   | 2-|-3
//   |/  |/
//   0---1
const CORNER_CONNS: [[usize; 3]; 8] = [
    [1, 2, 4], // 0 → 1, 2, 4
    [0, 3, 5], // 1 → 0, 3, 5
    [0, 3, 6], // 2 → 0, 3, 6
    [1, 2, 7], // 3 → 1, 2, 7
    [0, 5, 6], // 4 → 0, 5, 6
    [1, 4, 7], // 5 → 1, 4, 7
    [2, 4, 7], // 6 → 2, 4, 7
    [3, 5, 6], // 7 → 3, 5, 6
];

// The two corners each of the 12 edges connects.
const EDGE_DEF: [(usize, usize); 12] = [
    (0, 1), (0, 2), (0, 4), (2, 3), (1, 3), (1, 5),
    (4, 5), (4, 6), (2, 6), (6, 7), (5, 7), (3, 7),
];

fn edge_from_corners(a: usize, b: usize) -> usize {
    EDGE_DEF
        .iter()
        .position(|&(x, y)| (a == x && b == y) || (a == y && b == x))
        .unwrap_or_else(|| panic!("no edge for corners {a}-{b}"))
}

fn visit_all_corners(corner: usize, cell: u32, visited: &mut u32) -> u32 {
    if *visited & (1 << corner) != 0 {
        return 0;
    }
    *visited |= 1 << corner;
    let mut result = 0u32;
    for &adj in &CORNER_CONNS[corner] {
        if cell & (1 << adj) != 0 {
            result |= visit_all_corners(adj, cell, visited);
        } else {
            result |= 1 << edge_from_corners(corner, adj);
        }
    }
    result
}

fn edges(e: [usize; 3]) -> u32 {
    e.iter().fold(0u32, |acc, &i| acc | (1 << i))
}

// The 6-inside-corner case where the two outside corners are space-diagonal
// opposites requires special handling to produce a manifold mesh.
fn diagonal_case(cell: u32) -> Option<Vec<u32>> {
    if cell.count_ones() != 6 { return None; }
    let inv = (!cell) & 0xFF;
    let lowest = inv.trailing_zeros() as usize;
    if inv & (1 << (7 - lowest)) == 0 { return None; }
    Some(match lowest {
        0 => vec![edges([0, 1, 2]), edges([9, 10, 11])],
        1 => vec![edges([0, 4, 5]), edges([7, 8, 9])],
        2 => vec![edges([1, 3, 8]), edges([5, 6, 10])],
        3 => vec![edges([3, 4, 11]), edges([2, 6, 7])],
        _ => panic!("unexpected diagonal lowest corner {lowest}"),
    })
}

fn cell_config(corners: u8) -> Vec<u32> {
    let cell = u32::from(corners);
    if let Some(special) = diagonal_case(cell) {
        return special;
    }
    let mut result = Vec::new();
    let mut visited = 0u32;
    let mut remaining = cell;
    while remaining != 0 {
        let corner = remaining.trailing_zeros() as usize;
        let connected = visit_all_corners(corner, cell, &mut visited);
        if connected != 0 { result.push(connected); }
        remaining &= remaining - 1;
    }
    result
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("cell_configs_data.rs");

    let mut s = String::new();
    writeln!(s, "&[").unwrap();
    for corners in 0u8..=255 {
        let inner: Vec<String> = cell_config(corners)
            .iter()
            .map(|&bs| format!("BitSet({bs})"))
            .collect();
        writeln!(s, "    &[{}],", inner.join(", ")).unwrap();
    }
    writeln!(s, "]").unwrap();

    fs::write(dest, s).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
