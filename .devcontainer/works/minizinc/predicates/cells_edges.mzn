include "globals.mzn";

% cellの周りの数値が線の本数と一致
predicate around_edges(array[int, int] of var 0..1: h, array[int, int] of var 0..1: v, array[int, int] of var int: c) =

let {
	int: c_rows = card(index_set_1of2(c)),
	int: c_cols = card(index_set_2of2(c)),
}
in
(
	forall(i in 1..c_rows, j in 1..c_cols) (
		let { var int: left = v[i, j];
          var int: right = v[i, j+1];
          var int: top = h[i, j];
          var int: bottom = h[i+1, j];
        }
        in
        left + right + top + bottom = c[i, j]
		)
);