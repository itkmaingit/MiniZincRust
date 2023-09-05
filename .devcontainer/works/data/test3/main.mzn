include "../../minizinc/predicates/edges.mzn";
include "../../minizinc/predicates/points.mzn";
include "../../minizinc/predicates/cells.mzn";
include "../../minizinc/predicates/cells_edges.mzn";
include "../../minizinc/utilities/output.mzn";

int: UNKNOWN_NUMBER;

string: EPC;
int: W;
int: H;

array[1..H+1, 1..W] of {0, 1, UNKNOWN_NUMBER}: initial_h;
array[1..H, 1..W+1] of {0, 1, UNKNOWN_NUMBER}: initial_v;
array[1..H+1, 1..W+1] of int: initial_p;
array[1..H, 1..W] of int: initial_c;

array[1..H+1, 1..W] of var 0..1: h;
array[1..H, 1..W+1] of var 0..1: v;
% array[1..H+1, 1..W+1] of var int: p;
array[1..H, 1..W] of var 0..4: c;

constraint forall(i in 1..H+1, j in 1..W)(
        if initial_h[i,j] != UNKNOWN_NUMBER then h[i,j] = initial_h[i,j] else true endif
);

constraint forall(i in 1..H, j in 1..W+1)(
        if initial_v[i,j] != UNKNOWN_NUMBER then v[i,j] = initial_v[i,j] else true endif
);

% constraint forall(i in 1..H+1, j in 1..W+1)(
%         if initial_p[i,j] != UNKNOWN_NUMBER then p[i,j] = initial_p[i,j] else true endif
% );

constraint forall(i in 1..H, j in 1..W)(
        if initial_c[i,j] != UNKNOWN_NUMBER then c[i,j] = initial_c[i,j] else true endif
);

% constraint non_exists_one_length_edges(h, v);
constraint exists_under_n_length_edges(h, v, 4);
constraint around_edges(h, v, c);

solve satisfy;

output[output_puzzle(EPC, H, W, h, v, c)];