predicate sample(array[int, int] of var 0..1: matrix) =

let {
% index_set_1of2 -> 二次元配列の行数の取得
% index_set_2of2 -> 二次元配列の列数の取得
    int: rows = card(index_set_1of2(matrix)),
    int: cols = card(index_set_2of2(matrix))
  }
  in
    forall(i in 1..rows, j in 1..cols) (
    if j mod 2 = 0 then matrix[i,j] = 1 endif
);
