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
    let meta_data = fs::read_to_string(epc_txt_path).unwrap();
    let v: Vec<&str> = meta_data.split_whitespace().collect();
    let epc = Arc::new(v[0].trim_end().to_string());
    let H: usize = v[1].parse().unwrap();
    let W: usize = v[2].parse().unwrap();

    for entry in candidates_txt_pathes {
        let tmp_input_txt_path = entry.unwrap().path();
        let ttmp_input_txt_path = tmp_input_txt_path.to_str().unwrap();
        let input_txt_path = ttmp_input_txt_path.to_string();
        // epcによって場合分け
        let file_clone = file.clone();
        let solutions = Arc::clone(&solutions);
        let epc = Arc::clone(&epc);

        if epc.as_str() == E {
            let handle = thread::spawn(move || {
                let mut flag = true;
                let edges = input_e::<i32>(&input_txt_path, H, W);
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
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
