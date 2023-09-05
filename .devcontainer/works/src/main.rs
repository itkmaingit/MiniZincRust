mod constants;
mod dataclass;
mod rules;
mod utils;

use rules::predicates::graph_predicate::constraint_to_graph_length;
use rules::rules::line_length;

use crate::constants::constants::{C, E, EC, EP, EPC, P, THREAD_LIMITS};
#[allow(unused_imports)]
use crate::dataclass::graphs::{
    extract_branched_graph_from_board, extract_straight_graph_from_board, TGraph,
};
use crate::utils::input::{input_c, input_e, input_ec, input_ep, input_epc, input_p};
use crate::utils::output::{output_c, output_e, output_ec, output_ep, output_epc, output_p};
use crate::utils::remove_file::remove_file_if_exists;

use std::env;
use std::fs;
use std::fs::File;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

#[allow(non_snake_case, unused_mut, unused_variables)]
fn main() {
    // 初期化
    // コマンドライン引数の受取
    let args: Vec<String> = env::args().collect();
    let input_dir_path = &args[1];
    let epc_txt_path = &args[2];
    let output_txt_path = &args[3];

    // ファイルオブジェクトや変数の用意
    remove_file_if_exists(output_txt_path).unwrap();
    let candidates_txt_pathes = fs::read_dir(input_dir_path).unwrap();
    let file = File::create(output_txt_path).unwrap();
    let file = Arc::new(Mutex::new(file));
    let pool = ThreadPool::new(THREAD_LIMITS);

    //epc, H, Wの読み込み
    let meta_data = fs::read_to_string(epc_txt_path).unwrap();
    let v: Vec<&str> = meta_data.split_whitespace().collect();
    let epc = Arc::new(v[0].trim_end().to_string());
    let H: usize = v[1].parse().unwrap();
    let W: usize = v[2].parse().unwrap();

    for entry in candidates_txt_pathes {
        // inputするファイルを読み込み
        // （所有権の関係で）メソッドを分割している
        let tmp_input_txt_path = entry.unwrap().path();
        let ttmp_input_txt_path = tmp_input_txt_path.to_str().unwrap();
        let input_txt_path = ttmp_input_txt_path.to_string();

        // 共有メモリからファイルをクローン、内部で排他制御
        let file_clone = file.clone();
        let epc = Arc::clone(&epc);

        // threadpoolを用いて、スレッドを作成
        pool.execute(move || {
            // epcによって場合分け
            if epc.as_str() == E {
                let (edges, points, cells) = input_e::<i32>(&input_txt_path, H, W);
                let graphs = extract_branched_graph_from_board(&edges, &points);
                let is_satisfy = constraint_to_graph_length(&graphs, 4);

                if is_satisfy {
                    let file_lock = file_clone.lock().unwrap();
                    output_e(file_lock, &epc, H, W, &edges).unwrap();
                }
            } else if epc.as_str() == P {
                let (edges, points, cells) = input_p::<i32>(&input_txt_path, H, W);
                let is_satisfy = true;
                if is_satisfy {
                    let file_lock = file_clone.lock().unwrap();
                    output_p(file_lock, &epc, H, W, &points).unwrap();
                }
            } else if epc.as_str() == C {
                let (edges, points, cells) = input_c::<i32>(&input_txt_path, H, W);
                let is_satisfy = true;
                if is_satisfy {
                    let file_lock = file_clone.lock().unwrap();
                    output_c(file_lock, &epc, H, W, &cells).unwrap();
                }
            } else if epc.as_str() == EP {
                let (edges, points, cells) = input_ep::<i32, i32>(&input_txt_path, H, W);
                let is_satisfy = true;
                if is_satisfy {
                    let file_lock = file_clone.lock().unwrap();
                    output_ep(file_lock, &epc, H, W, &edges, &points).unwrap();
                }
            } else if epc.as_str() == EC {
                let (edges, points, cells) = input_ec::<i32, i32>(&input_txt_path, H, W);
                let graphs = extract_branched_graph_from_board(&edges, &points);
                let is_satisfy = line_length(&graphs, 4);
                if is_satisfy {
                    let file_lock = file_clone.lock().unwrap();
                    output_ec(file_lock, &epc, H, W, &edges, &cells).unwrap();
                }
            } else if epc.as_str() == EPC {
                let (edges, points, cells) = input_epc::<i32, i32, i32>(&input_txt_path, H, W);
                let is_satisfy = true;
                if is_satisfy {
                    let file_lock = file_clone.lock().unwrap();
                    output_epc(file_lock, &epc, H, W, &edges, &points, &cells).unwrap();
                }
            }
        })
    }
    pool.join()
}
