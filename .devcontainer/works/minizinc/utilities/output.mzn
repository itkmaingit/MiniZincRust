function string: output_2d_array(int: rows, int: cols, array[int, int] of var int: arr, ) =
    join("\n", [ join(" ", [ show(arr[i,j]) | j in 1..cols]) | i in 1..rows]);

function string: output_puzzle(string: epc, int: h, int: w, array[int, int] of var int: arr1) =
let {
% index_set_1of2 -> 二次元配列の1次元目の配列を取得
% index_set_2of2 -> 二次元配列の2次元目の配列を取得
    int: rows1 = card(index_set_1of2(arr1)),
    int: cols1 = card(index_set_2of2(arr1))
  }
  in
	epc ++ " " ++ format(h) ++ " " ++ format(w) ++ "\n" ++ output_2d_array(rows1, cols1, arr1);

function string: output_puzzle(string: epc, int: h, int: w, array[int, int] of var int: arr1, array[int, int] of var int: arr2) =
let {

    int: rows1 = card(index_set_1of2(arr1)),
    int: cols1 = card(index_set_2of2(arr1)),
    int: rows2 = card(index_set_1of2(arr2)),
    int: cols2 = card(index_set_2of2(arr2)),
  }
  in
	epc ++ " " ++ format(h) ++ " " ++ format(w) ++ "\n" ++ output_2d_array(rows1, cols1, arr1) ++ "\n" ++ output_2d_array(rows2, cols2, arr2);

function string: output_puzzle(string: epc, int: h, int: w, array[int, int] of var int: arr1, array[int, int] of var int: arr2, array[int, int] of var int: arr3) =
let {
    int: rows1 = card(index_set_1of2(arr1)),
    int: cols1 = card(index_set_2of2(arr1)),
    int: rows2 = card(index_set_1of2(arr2)),
    int: cols2 = card(index_set_2of2(arr2)),
    int: rows3 = card(index_set_1of2(arr3)),
    int: cols3 = card(index_set_2of2(arr3)),
  }
  in
  	epc ++ " " ++ format(h) ++ " " ++ format(w) ++ "\n" ++ output_2d_array(rows1, cols1, arr1) ++ "\n" ++ output_2d_array(rows2, cols2, arr2) ++ "\n" ++ output_2d_array(rows3, cols3, arr3);

function string: output_puzzle(string: epc, int: h, int: w, array[int, int] of var int: arr1, array[int, int] of var int: arr2, array[int, int] of var int: arr3, array[int, int] of var int: arr4) =
let {

    int: rows1 = card(index_set_1of2(arr1)),
    int: cols1 = card(index_set_2of2(arr1)),
    int: rows2 = card(index_set_1of2(arr2)),
    int: cols2 = card(index_set_2of2(arr2)),
    int: rows3 = card(index_set_1of2(arr3)),
    int: cols3 = card(index_set_2of2(arr3)),
    int: rows4 = card(index_set_1of2(arr4)),
    int: cols4 = card(index_set_2of2(arr4))
  }
  in
	epc ++ " " ++ format(h) ++ " " ++ format(w) ++ "\n" ++ output_2d_array(rows1, cols1, arr1) ++ "\n" ++ output_2d_array(rows2, cols2, arr2) ++ "\n" ++ output_2d_array(rows3, cols3, arr3) ++ "\n" ++ output_2d_array(rows4, cols4, arr4);
