mod constants;
mod dataclass;
mod utils;

use crate::constants::constants::{C, E, EC, EP, EPC, P};
#[allow(unused_imports)]
use crate::dataclass::graphs::{
    extract_branched_graph_from_board, extract_straight_graph_from_board,
};
use crate::utils::input::input_e;
// use crate::utils::input::{input_c, input_e, input_ec, input_ep, input_epc, input_p};
use crate::utils::output::{output_c, output_e, output_ec, output_ep, output_epc, output_p};
use crate::utils::remove_file::remove_file_if_exists;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(non_snake_case)]
fn main() {
    // 初期化
    // コマンドライン引数の受取
    let args: Vec<String> = env::args().collect();
    let input_dir_path = &args[1];
    let epc_txt_path = &args[2];
    let output_txt_path = &args[3];

    // ファイルオブジェクトや、solutionsなどの変数の用意
    remove_file_if_exists(output_txt_path).unwrap();
    let solutions = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let candidates_txt_pathes = fs::read_dir(input_dir_path).unwrap();
    let file = File::create("output.txt").unwrap();
    let file = Arc::new(Mutex::new(file));

    //epc, H, Wの読み込み
    let meta_data = fs::read_to_string(epc_txt_path).unwrap().trim_end();
    let v: Vec<&str> = meta_data.split_whitespace().collect();
    let epc = v[0].to_string();
    let H: usize = v[1].parse().unwrap();
    let W: usize = v[2].parse().unwrap();

    for entry in candidates_txt_pathes {
        let input_txt_path = entry.unwrap().path().to_str().unwrap();
        // epcによって場合分け
        let file_clone = file.clone();
        let solutions = Arc::clone(&solutions);

        if epc == E {
            let handle = thread::spawn(move || {
                let mut flag = true;
                let edges = input_e::<i32>("input_txt_path", H, W);
                // let points = vec![vec![0; W + 1]; H + 1];
                // let straight_graphs = extract_straight_graph_from_board(&edges, &points);
                if flag {
                    let file_lock = file_clone.lock().unwrap();
                    output_e(file_lock, &epc, H, W, &edges).unwrap();
                    let mut num = solutions.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        // else if epc == P {
        //     loop {
        //         let (h, w, points) = input_p(solutions, is_first_loop);
        //         output_p(rule_name, "010", h, w, &points).unwrap();
        //         solutions += 1;
        //         is_first_loop = false;
        //     }
        // } else if epc == C {
        //     loop {
        //         let (h, w, cells) = input_c(solutions, is_first_loop);
        //         output_c(rule_name, "001", h, w, &cells).unwrap();
        //         solutions += 1;
        //         is_first_loop = false;
        //     }
        // } else if epc == EP {
        //     loop {
        //         let (h, w, edges, points) = input_ep(solutions, is_first_loop);

        //         output_ep(rule_name, "110", h, w, &edges, &points).unwrap();
        //         solutions += 1;
        //         is_first_loop = false;
        //     }
        // } else if epc == EC {
        //     loop {
        //         let (h, w, edges, cells) = input_ec(solutions, is_first_loop);
        //         output_ec(rule_name, "101", h, w, &edges, &cells).unwrap();
        //         solutions += 1;
        //         is_first_loop = false;
        //     }
        // } else if epc == EPC {
        //     loop {
        //         let (h, w, edges, points, cells) = input_epc(solutions, is_first_loop);
        //         output_epc(rule_name, "111", h, w, &edges, &points, &cells).unwrap();
        //         solutions += 1;
        //         is_first_loop = false;
        //     }
        // }
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
