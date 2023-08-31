use crate::dataclass::dataclass::{Cells, Edges, Points};
use core::str::FromStr;
use proconio::input;
use proconio::marker::Chars;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::process;
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
pub fn input_e<E: FromStr>(input_txt_path: &str, h: usize, w: usize) -> Edges<E> {
    let file = File::open(input_txt_path).unwrap();
    let reader = BufReader::new(file);

    let h_rows = h + 1;
    let h_cols = w;
    let v_rows = h;
    let v_cols = w + 1;

    let mut line_iter = reader.lines();

    let H = parse_variable(&mut line_iter, h_rows, h_cols);
    let V = parse_variable(&mut line_iter, v_rows, v_cols);

    let edges = Edges::new(H, V).unwrap();

    return edges;
}

// #[allow(non_snake_case)]
// pub fn input_p(solutions: usize, is_first_loop: bool) -> (usize, usize, Points<i32>) {
//     //最初のループであればepcは既に読み込まれているのでスキップ
//     let epc: Vec<char>;
//     if !is_first_loop {
//         input! {
//             epc_inner: Chars
//         }
//         epc = epc_inner;
//     } else {
//         epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
//     }

//     // 入力が "===" の場合、ループを終了する
//     if epc == FINISHED_SIGNAL {
//         println!("solutions: {}", solutions);
//         process::exit(0);
//     }

//     input! {
//         h:usize,
//         w:usize,
//         points: [[i32; w+1]; h+1],
//         _boundary: Chars,
//     };

//     return (h, w, points);
// }

// #[allow(non_snake_case)]
// pub fn input_c(solutions: usize, is_first_loop: bool) -> (usize, usize, Cells<i32>) {
//     //最初のループであればepcは既に読み込まれているのでスキップ
//     let epc: Vec<char>;
//     if !is_first_loop {
//         input! {
//             epc_inner: Chars
//         }
//         epc = epc_inner;
//     } else {
//         epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
//     }

//     // 入力が "===" の場合、ループを終了する
//     if epc == FINISHED_SIGNAL {
//         println!("solutions: {}", solutions);
//         process::exit(0);
//     }

//     input! {
//         h:usize,
//         w:usize,
//         cells: [[i32; w]; h],
//         _boundary: Chars,
//     };

//     return (h, w, cells);
// }

// #[allow(non_snake_case)]
// pub fn input_ep(solutions: usize, is_first_loop: bool) -> (usize, usize, Edges<i32>, Points<i32>) {
//     //最初のループであればepcは既に読み込まれているのでスキップ
//     let epc: Vec<char>;
//     if !is_first_loop {
//         input! {
//             epc_inner: Chars
//         }
//         epc = epc_inner;
//     } else {
//         epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
//     }

//     // 入力が "===" の場合、ループを終了する
//     if epc == FINISHED_SIGNAL {
//         println!("solutions: {}", solutions);
//         process::exit(0);
//     }

//     input! {
//         h:usize,
//         w:usize,
//         H: [[i32; w]; h+1],
//         V: [[i32; w+1]; h],
//         points: [[i32; w+1]; h+1],
//         _boundary: Chars,
//     };

//     let edges = Edges::new(H, V).unwrap();

//     return (h, w, edges, points);
// }

// #[allow(non_snake_case)]
// pub fn input_ec(solutions: usize, is_first_loop: bool) -> (usize, usize, Edges<i32>, Cells<i32>) {
//     //最初のループであればepcは既に読み込まれているのでスキップ
//     let epc: Vec<char>;
//     if !is_first_loop {
//         input! {
//             epc_inner: Chars
//         }
//         epc = epc_inner;
//     } else {
//         epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
//     }

//     // 入力が "===" の場合、ループを終了する
//     if epc == FINISHED_SIGNAL {
//         println!("solutions: {}", solutions);
//         process::exit(0);
//     }

//     input! {
//         h:usize,
//         w:usize,
//         H: [[i32; w]; h+1],
//         V: [[i32; w+1]; h],
//         cells: [[i32; w]; h],
//         _boundary: Chars,
//     };

//     let edges = Edges::new(H, V).unwrap();

//     return (h, w, edges, cells);
// }

// #[allow(non_snake_case)]
// pub fn input_epc(
//     solutions: usize,
//     is_first_loop: bool,
// ) -> (usize, usize, Edges<i32>, Points<i32>, Cells<i32>) {
//     //最初のループであればepcは既に読み込まれているのでスキップ
//     let epc: Vec<char>;
//     if !is_first_loop {
//         input! {
//             epc_inner: Chars
//         }
//         epc = epc_inner;
//     } else {
//         epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
//     }

//     // 入力が "===" の場合、ループを終了する
//     if epc == FINISHED_SIGNAL {
//         println!("solutions: {}", solutions);
//         process::exit(0);
//     }

//     input! {
//         h:usize,
//         w:usize,
//         H: [[i32; w]; h+1],
//         V: [[i32; w+1]; h],
//         points: [[i32; w+1]; h+1],
//         cells: [[i32; w]; h],
//         _boundary: Chars,
//     };

//     let edges = Edges::new(H, V).unwrap();

//     return (h, w, edges, points, cells);
// }

fn parse_variable<T: FromStr>(
    line_iter: &mut Lines<BufReader<File>>,
    rows: usize,
    cols: usize,
) -> Vec<Vec<T>> {
    let mut matrix: Vec<Vec<T>> = Vec::new();
    for _ in 0..rows {
        if let Some(Ok(line)) = line_iter.next() {
            let row: Vec<T> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            if row.len() == cols {
                matrix.push(row);
            } else {
                eprintln!("Invalid number of columns in a row for h.");
                process::exit(1);
            }
        } else {
            eprintln!("File ended before reading all rows for h.");
            process::exit(1);
        }
    }

    return matrix;
}
