% 盤面に1本の線がないことを保証する制約
predicate non_exists_one_length_edges(array[int, int] of var 0..1: h, array[int, int] of var 0..1: v) =

let {
    int: h_rows = card(index_set_1of2(h)),
    int: h_cols = card(index_set_2of2(h)),
    int: v_rows = card(index_set_1of2(v)),
    int: v_cols = card(index_set_2of2(v))
}
in
(
    % For h
    forall(i in 1..h_rows, j in 1..h_cols) (
        let { var int: h_left = if j-1 >= 1 then h[i,j-1] else 0 endif;
              var int: h_right = if j+1 <= h_cols then h[i,j+1] else 0 endif;
              var int: v_top_left = if i-1 >= 1 then v[i-1,j] else 0 endif;
              var int: v_top_right = if i-1 >= 1  then v[i-1,j+1] else 0 endif;
              var int: v_bottom_left = if i <=v_rows then v[i,j] else 0 endif;
              var int: v_bottom_right = if  i <=v_rows then v[i,j+1] else 0 endif;
        }
        in
        h_left + h_right + v_top_left + v_bottom_left + v_top_right + v_bottom_right >= 1
    )
    /\
    % For v
    forall(i in 1..v_rows, j in 1..v_cols) (
        let { var int: v_top = if i-1 >= 1 then v[i-1,j] else 0 endif;
              var int: v_bottom = if i+1 <= v_rows then v[i+1,j] else 0 endif;
              var int: h_top_left = if j-1 >= 1 then h[i,j-1] else 0 endif;
              var int: h_bottom_left = if j-1 >= 1 then h[i+1,j-1] else 0 endif;
              var int: h_top_right = if j <= h_cols then h[i,j] else 0 endif;
              var int: h_bottom_right = if j <= h_cols then h[i+1,j] else 0 endif;
        }
        in
        v_top + v_bottom + h_top_left + h_top_right + h_bottom_left + h_bottom_right >= 1
    )
);

% 盤面に存在する線の長さがn以下であるための必要条件
predicate exists_under_n_length_edges(array[int, int] of var 0..1: h, array[int, int] of var 0..1: v, int: n) =

let {
    int: h_rows = card(index_set_1of2(h)),
    int: h_cols = card(index_set_2of2(h)),
    int: v_rows = card(index_set_1of2(v)),
    int: v_cols = card(index_set_2of2(v))
}
in
(
    % For h
    forall(i in 1..h_rows, j in 1..h_cols) (
        let { var int: h_left = if j-1 >= 1 then h[i,j-1] else 0 endif;
              var int: h_right = if j+1 <= h_cols then h[i,j+1] else 0 endif;
              var int: v_top_left = if i-1 >= 1 then v[i-1,j] else 0 endif;
              var int: v_top_right = if i-1 >= 1  then v[i-1,j+1] else 0 endif;
              var int: v_bottom_left = if i <=v_rows then v[i,j] else 0 endif;
              var int: v_bottom_right = if  i <=v_rows then v[i,j+1] else 0 endif;
              var int: h_self = h[i, j];
        }
        in
        h_self = 1 -> h_left + h_right + v_top_left + v_bottom_left + v_top_right + v_bottom_right <= n - 1
    )
    /\
    % For v
    forall(i in 1..v_rows, j in 1..v_cols) (
        let { var int: v_top = if i-1 >= 1 then v[i-1,j] else 0 endif;
              var int: v_bottom = if i+1 <= v_rows then v[i+1,j] else 0 endif;
              var int: h_top_left = if j-1 >= 1 then h[i,j-1] else 0 endif;
              var int: h_bottom_left = if j-1 >= 1 then h[i+1,j-1] else 0 endif;
              var int: h_top_right = if j <= h_cols then h[i,j] else 0 endif;
              var int: h_bottom_right = if j <= h_cols then h[i+1,j] else 0 endif;
              var int: v_self = v[i, j];
        }
        in
        v_self = 1 -> v_top + v_bottom + h_top_left + h_top_right + h_bottom_left + h_bottom_right <= n
    )
);

% crossに対する静的制約。
% nはcrossの制限数。配列で受け取るので、候補を渡す。
predicate cross_constraint(array[int, int] of var 0..1: h, array[int, int] of var 0..1: v, array[int] of int: n) =

let {
    int: H = card(index_set_1of2(v)),
    int: W = card(index_set_2of2(h))
}
in
(
    forall(i in 1..H+1, j in 1..W+1) (
        let { var int: v_top = if i-1 >= 1 then v[i-1,j] else 0 endif;
              var int: v_bottom = if i <= H then v[i,j] else 0 endif;
              var int: h_left = if j-1 >= 1 then h[i,j-1] else 0 endif;
              var int: h_right = if j <= W then h[i,j] else 0 endif;
              var int: sum_edges = v_top + v_bottom + h_left + h_right;
        }
        in
        exists(value in n) (sum_edges = value)
    )
);

% cycleに対する静的制約。
% nはcycleの制限数。配列で受け取るので、候補を渡す。

predicate cycle_constraint(array[int, int] of var 0..1: h, array[int, int] of var 0..1: v, array[int] of int: n) =

let {
    int: H = card(index_set_1of2(v)),
    int: W = card(index_set_2of2(h))
}
in
(
    forall(i in 1..H, j in 1..W) (
        let { var int: h_top = h[i,j];
              var int: h_bottom = h[i+1,j];
              var int: v_left = v[i,j];
              var int: v_right = v[i,j+1];
              var int: sum_edges = h_top + h_bottom + v_left + v_right
        }
        in
        exists(value in n) (sum_edges = value)
    )
);