use crate::dataclass::dataclass::{Cells, Edges, Points};
use core::str::FromStr;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::process;

#[allow(non_snake_case)]
pub fn input_e<E: FromStr>(
    input_txt_path: &str,
    h: usize,
    w: usize,
) -> (Edges<E>, Points<i32>, Cells<i32>) {
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
    let (_, points, cells) = create_dummy_variables(h, w);

    return (edges, points, cells);
}

#[allow(non_snake_case)]
pub fn input_p<P: FromStr>(
    input_txt_path: &str,
    h: usize,
    w: usize,
) -> (Edges<i32>, Points<P>, Cells<i32>) {
    let file = File::open(input_txt_path).unwrap();
    let reader = BufReader::new(file);

    let p_rows = h + 1;
    let p_cols = w + 1;

    let mut line_iter = reader.lines();

    let points = parse_variable(&mut line_iter, p_rows, p_cols);
    let (edges, _, cells) = create_dummy_variables(h, w);

    return (edges, points, cells);
}
#[allow(non_snake_case)]
pub fn input_c<C: FromStr>(
    input_txt_path: &str,
    h: usize,
    w: usize,
) -> (Edges<i32>, Points<i32>, Cells<C>) {
    let file = File::open(input_txt_path).unwrap();
    let reader = BufReader::new(file);

    let c_rows = h;
    let c_cols = w;

    let mut line_iter = reader.lines();

    let cells = parse_variable(&mut line_iter, c_rows, c_cols);
    let (edges, points, _) = create_dummy_variables(h, w);

    return (edges, points, cells);
}

#[allow(non_snake_case)]
pub fn input_ep<E: FromStr, P: FromStr>(
    input_txt_path: &str,
    h: usize,
    w: usize,
) -> (Edges<E>, Points<P>, Cells<i32>) {
    let file = File::open(input_txt_path).unwrap();
    let reader = BufReader::new(file);

    let h_rows = h + 1;
    let h_cols = w;
    let v_rows = h;
    let v_cols = w + 1;
    let p_rows = h + 1;
    let p_cols = w + 1;

    let mut line_iter = reader.lines();

    let H = parse_variable(&mut line_iter, h_rows, h_cols);
    let V = parse_variable(&mut line_iter, v_rows, v_cols);
    let points = parse_variable(&mut line_iter, p_rows, p_cols);

    let edges = Edges::new(H, V).unwrap();
    let (_, _, cells) = create_dummy_variables(h, w);

    return (edges, points, cells);
}

#[allow(non_snake_case)]
pub fn input_ec<E: FromStr, C: FromStr>(
    input_txt_path: &str,
    h: usize,
    w: usize,
) -> (Edges<E>, Points<i32>, Cells<C>) {
    let file = File::open(input_txt_path).unwrap();
    let reader = BufReader::new(file);

    let h_rows = h + 1;
    let h_cols = w;
    let v_rows = h;
    let v_cols = w + 1;
    let c_rows = h;
    let c_cols = w;

    let mut line_iter = reader.lines();

    let H = parse_variable(&mut line_iter, h_rows, h_cols);
    let V = parse_variable(&mut line_iter, v_rows, v_cols);
    let cells = parse_variable(&mut line_iter, c_rows, c_cols);

    let edges = Edges::new(H, V).unwrap();
    let (_, points, _) = create_dummy_variables(h, w);

    return (edges, points, cells);
}

#[allow(non_snake_case)]
pub fn input_epc<E: FromStr, P: FromStr, C: FromStr>(
    input_txt_path: &str,
    h: usize,
    w: usize,
) -> (Edges<E>, Points<P>, Cells<C>) {
    let file = File::open(input_txt_path).unwrap();
    let reader = BufReader::new(file);

    let h_rows = h + 1;
    let h_cols = w;
    let v_rows = h;
    let v_cols = w + 1;
    let p_rows = h + 1;
    let p_cols = w + 1;
    let c_rows = h;
    let c_cols = w;

    let mut line_iter = reader.lines();

    let H = parse_variable(&mut line_iter, h_rows, h_cols);
    let V = parse_variable(&mut line_iter, v_rows, v_cols);
    let points = parse_variable(&mut line_iter, p_rows, p_cols);
    let cells = parse_variable(&mut line_iter, c_rows, c_cols);

    let edges = Edges::new(H, V).unwrap();

    return (edges, points, cells);
}

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

fn create_dummy_variables(h: usize, w: usize) -> (Edges<i32>, Points<i32>, Cells<i32>) {
    let horizon_edges = vec![vec![0; w]; h + 1];
    let vertical_edges = vec![vec![0; w + 1]; h];
    let edges = Edges::new(horizon_edges, vertical_edges).unwrap();
    let points = vec![vec![0; w + 1]; h + 1];
    let cells = vec![vec![0; w]; h];
    return (edges, points, cells);
}
