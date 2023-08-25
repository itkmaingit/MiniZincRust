use crate::dataclass::dataclass::{Cells, Edges, Points};
use proconio::input;
use proconio::marker::Chars;
use std::process;

const FINISHED_SIGNAL: &[char] = &['=', '=', '=', '=', '=', '=', '=', '=', '=', '='];

#[allow(non_snake_case)]
pub fn input_e(solutions: usize, is_first_loop: bool) -> (usize, usize, Edges<i32>) {
    //最初のループであればepcは既に読み込まれているのでスキップ
    let epc: Vec<char>;
    if !is_first_loop {
        input! {
            epc_inner: Chars
        }
        epc = epc_inner;
    } else {
        epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
    }

    // 入力が "===" の場合、ループを終了する
    if epc == FINISHED_SIGNAL {
        println!("solutions: {}", solutions);
        process::exit(0);
    }

    input! {
        h:usize,
        w:usize,
        H: [[i32; w]; h+1],
        V: [[i32; w+1]; h],
        _boundary: Chars,
    };

    let edges = Edges::new(H, V).unwrap();

    return (h, w, edges);
}

#[allow(non_snake_case)]
pub fn input_p(solutions: usize, is_first_loop: bool) -> (usize, usize, Points<i32>) {
    //最初のループであればepcは既に読み込まれているのでスキップ
    let epc: Vec<char>;
    if !is_first_loop {
        input! {
            epc_inner: Chars
        }
        epc = epc_inner;
    } else {
        epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
    }

    // 入力が "===" の場合、ループを終了する
    if epc == FINISHED_SIGNAL {
        println!("solutions: {}", solutions);
        process::exit(0);
    }

    input! {
        h:usize,
        w:usize,
        points: [[i32; w+1]; h+1],
        _boundary: Chars,
    };

    return (h, w, points);
}

#[allow(non_snake_case)]
pub fn input_c(solutions: usize, is_first_loop: bool) -> (usize, usize, Cells<i32>) {
    //最初のループであればepcは既に読み込まれているのでスキップ
    let epc: Vec<char>;
    if !is_first_loop {
        input! {
            epc_inner: Chars
        }
        epc = epc_inner;
    } else {
        epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
    }

    // 入力が "===" の場合、ループを終了する
    if epc == FINISHED_SIGNAL {
        println!("solutions: {}", solutions);
        process::exit(0);
    }

    input! {
        h:usize,
        w:usize,
        cells: [[i32; w]; h],
        _boundary: Chars,
    };

    return (h, w, cells);
}

#[allow(non_snake_case)]
pub fn input_ep(solutions: usize, is_first_loop: bool) -> (usize, usize, Edges<i32>, Points<i32>) {
    //最初のループであればepcは既に読み込まれているのでスキップ
    let epc: Vec<char>;
    if !is_first_loop {
        input! {
            epc_inner: Chars
        }
        epc = epc_inner;
    } else {
        epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
    }

    // 入力が "===" の場合、ループを終了する
    if epc == FINISHED_SIGNAL {
        println!("solutions: {}", solutions);
        process::exit(0);
    }

    input! {
        h:usize,
        w:usize,
        H: [[i32; w]; h+1],
        V: [[i32; w+1]; h],
        points: [[i32; w+1]; h+1],
        _boundary: Chars,
    };

    let edges = Edges::new(H, V).unwrap();

    return (h, w, edges, points);
}

#[allow(non_snake_case)]
pub fn input_ec(solutions: usize, is_first_loop: bool) -> (usize, usize, Edges<i32>, Cells<i32>) {
    //最初のループであればepcは既に読み込まれているのでスキップ
    let epc: Vec<char>;
    if !is_first_loop {
        input! {
            epc_inner: Chars
        }
        epc = epc_inner;
    } else {
        epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
    }

    // 入力が "===" の場合、ループを終了する
    if epc == FINISHED_SIGNAL {
        println!("solutions: {}", solutions);
        process::exit(0);
    }

    input! {
        h:usize,
        w:usize,
        H: [[i32; w]; h+1],
        V: [[i32; w+1]; h],
        cells: [[i32; w]; h],
        _boundary: Chars,
    };

    let edges = Edges::new(H, V).unwrap();

    return (h, w, edges, cells);
}

#[allow(non_snake_case)]
pub fn input_epc(
    solutions: usize,
    is_first_loop: bool,
) -> (usize, usize, Edges<i32>, Points<i32>, Cells<i32>) {
    //最初のループであればepcは既に読み込まれているのでスキップ
    let epc: Vec<char>;
    if !is_first_loop {
        input! {
            epc_inner: Chars
        }
        epc = epc_inner;
    } else {
        epc = vec![' '; 1]; // あるいは何らかのデフォルトの値を設定
    }

    // 入力が "===" の場合、ループを終了する
    if epc == FINISHED_SIGNAL {
        println!("solutions: {}", solutions);
        process::exit(0);
    }

    input! {
        h:usize,
        w:usize,
        H: [[i32; w]; h+1],
        V: [[i32; w+1]; h],
        points: [[i32; w+1]; h+1],
        cells: [[i32; w]; h],
        _boundary: Chars,
    };

    let edges = Edges::new(H, V).unwrap();

    return (h, w, edges, points, cells);
}
