use proconio::input;
use proconio::marker::Chars;

fn main() {
    loop {
        input! {
            epc: proconio::marker::Chars
        }

        // 入力が "===" の場合、ループを終了する
        if epc == vec!['=', '=', '='] {
            break;
        }

        input! {
            h:usize,
            w:usize,
            a: [[i32; w]; h],
            _boundary: Chars,
        }
        println!(
            "Got: epc = {:?}, h = {:?}, w = {:?}, a = {:?}",
            epc, h, w, a
        );
    }
}
