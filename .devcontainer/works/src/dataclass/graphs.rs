use crate::dataclass::dataclass::{Edges, Points};
use crate::dataclass::search_functions::{dfs_h, dfs_v, find_first_false, is_falsy, Graph};

// pub struct StraightGraph<T1: PartialEq, T2> {
//     edges: Edges<T1>,
//     points: Points<T2>,
// }

pub struct Point<P> {
    pub value: P,
    pub cross: usize,
}

pub struct BranchedGraph<P> {
    pub length: usize,
    pub points: Vec<Vec<Point<P>>>,
    pub graph: Graph<P>,
}

impl<P: Copy> BranchedGraph<P> {
    pub fn new(
        (rows, cols): (usize, usize),
        graph: &Graph<P>,
        points: &Points<P>,
    ) -> BranchedGraph<P> {
        let length = (0..rows)
            .map(|i| {
                (0..cols).map(|j| graph[i][j].len()).sum::<usize>() // 各行の合計を計算
            })
            .sum::<usize>()
            / 2; // 全行の合計を計算

        let included_points: Vec<Vec<Point<P>>> = (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| Point {
                        value: points[i][j],
                        cross: graph[i][j].len(),
                    })
                    .collect()
            })
            .collect();
        return BranchedGraph {
            length,
            points: included_points,
            graph: graph.clone(),
        };
    }
}

// 盤面からBranchGraphの配列を取得する関数
pub fn extract_branched_graph_from_board<E: PartialEq + Default, P: Clone + Copy>(
    edges: &Edges<E>,
    points: &Points<P>,
) -> Vec<BranchedGraph<P>> {
    let mut branched_graphs: Vec<BranchedGraph<P>> = Vec::new();
    let (h_rows, h_cols) = edges.h_size();
    let (v_rows, v_cols) = edges.v_size();

    // hの値がfalsyならtrue、そうでなければfalseを返す
    let mut seen_h: Vec<Vec<bool>> = (0..h_rows)
        .map(|i| {
            (0..h_cols)
                .map(|j| is_falsy(&edges.h[i][j])) // ここでfalsyを判定しています
                .collect()
        })
        .collect();

    // vの値がfalsyならtrue、そうでなければfalseを返す
    let mut seen_v: Vec<Vec<bool>> = (0..v_rows)
        .map(|i| {
            (0..v_cols)
                .map(|j| is_falsy(&edges.v[i][j])) // ここでfalsyを判定しています
                .collect()
        })
        .collect();

    while let Some((i, j)) = find_first_false(&seen_h) {
        let mut graph: Graph<P> = vec![vec![Vec::new(); v_cols]; h_rows];
        dfs_h(
            (i, j),
            &mut seen_h,
            &mut seen_v,
            points,
            &mut graph,
            h_cols,
            v_rows,
        );
        let branched_graph = BranchedGraph::new((h_rows, v_cols), &graph, points);
        branched_graphs.push(branched_graph);
    }

    while let Some((i, j)) = find_first_false(&seen_v) {
        let mut graph: Graph<P> = vec![vec![Vec::new(); v_cols]; h_rows];
        dfs_v(
            (i, j),
            &mut seen_h,
            &mut seen_v,
            points,
            &mut graph,
            h_cols,
            v_rows,
        );
        let branched_graph = BranchedGraph::new((h_rows, v_cols), &graph, points);
        branched_graphs.push(branched_graph);
    }

    return branched_graphs;
}
