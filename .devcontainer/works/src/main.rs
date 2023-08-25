mod constants;
mod dataclass;
mod utils;

use crate::constants::constants::{C, E, EC, EP, EPC, P};
use crate::utils::input::{input_c, input_e, input_ec, input_ep, input_epc, input_p};
use crate::utils::output::{output_c, output_e, output_ec, output_ep, output_epc, output_p};
use crate::utils::remove_file::remove_file_if_exists;

use proconio::input;
use proconio::marker::Chars;

const FILE_NAME: &str = "test.txt";

#[allow(non_snake_case)]
fn main() {
    // 初期化
    let mut solutions = 0;
    let mut is_first_loop = true;
    remove_file_if_exists(FILE_NAME).unwrap();

    //最初の行の読み込み
    input! {
        epc: Chars
    }

    // epcによって場合分け
    if epc == E {
        loop {
            let (h, w, edges) = input_e(solutions, is_first_loop);
            output_e(FILE_NAME, "100", h, w, &edges).unwrap();
            solutions += 1;
            is_first_loop = false;
        }
    } else if epc == P {
        loop {
            let (h, w, points) = input_p(solutions, is_first_loop);
            output_p(FILE_NAME, "010", h, w, &points).unwrap();
            solutions += 1;
            is_first_loop = false;
        }
    } else if epc == C {
        loop {
            let (h, w, cells) = input_c(solutions, is_first_loop);
            output_c(FILE_NAME, "001", h, w, &cells).unwrap();
            solutions += 1;
            is_first_loop = false;
        }
    } else if epc == EP {
        loop {
            let (h, w, edges, points) = input_ep(solutions, is_first_loop);
            output_ep(FILE_NAME, "100", h, w, &edges, &points).unwrap();
            solutions += 1;
            is_first_loop = false;
        }
    } else if epc == EC {
        loop {
            let (h, w, edges, cells) = input_ec(solutions, is_first_loop);
            output_ec(FILE_NAME, "100", h, w, &edges, &cells).unwrap();
            solutions += 1;
            is_first_loop = false;
        }
    } else if epc == EPC {
        loop {
            let (h, w, edges, points, cells) = input_epc(solutions, is_first_loop);
            output_epc(FILE_NAME, "100", h, w, &edges, &points, &cells).unwrap();
            solutions += 1;
            is_first_loop = false;
        }
    }
}
