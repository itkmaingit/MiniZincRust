include "globals.mzn";

% 縦列、横列ともに数値が重複しない制約
predicate number_place(array[int, int] of var int: c) =

let {
	int: rows = card(index_set_1of2(c)),
	int: cols = card(index_set_2of2(c)),
}
in
(
	forall(i in 1..rows) (
		alldifferent([c[i, j] | j in 1..cols])
		)
		/\
		forall(j in 1..cols) (
		alldifferent([c[i, j] | i in 1..rows])
		)
);