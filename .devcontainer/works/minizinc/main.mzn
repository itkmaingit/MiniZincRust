include "predicates/edges.mzn";
% include "predicates/points.mzn";
include "predicates/cells.mzn";
include "utilities/output.mzn";
string: EPC = "100";
int: W=2;
int: H=2;

% array[1..H+1, 1..W] of {0, 1, 9}: initial_h;
% array[1..H, 1..W+1] of {0, 1, 9}: initial_v;
% array[1..H+1, 1..W+1] of int: initial_p;
% array[1..H, 1..W] of int: initial_c;

array[1..H+1, 1..W] of var 0..1: h;
array[1..H, 1..W+1] of var 0..1: v;
% array[1..H+1, 1..W+1] of var int: p;
% array[1..H, 1..W] of var 1..W: c;

% constraint forall(i in 1..H+1, j in 1..W)(
%         if initial_h[i,j] != 9 then h[i,j] = initial_h[i,j] else true endif
% );

% constraint forall(i in 1..H, j in 1..W+1)(
%         if initial_v[i,j] != 9 then v[i,j] = initial_v[i,j] else true endif
% );

% constraint forall(i in 1..H+1, j in 1..W+1)(
%         if initial_p[i,j] != 0 then p[i,j] = initial_p[i,j] else true endif
% );

% constraint forall(i in 1..H, j in 1..W+1)(
%         if initial_c[i,j] != 0 then c[i,j] = initial_c[i,j] else true endif
% );

constraint cross_constraint(h, v, [0, 2]);

solve satisfy;

output[output_puzzle(EPC, H, W, h, v)];
