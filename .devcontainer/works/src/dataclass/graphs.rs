use std::collections::HashSet;
use std::hash::Hash;

use crate::dataclass::dataclass::{Edges, Points};
use crate::dataclass::search_functions::{dfs_h, dfs_v, find_first_false, is_falsy, Graph};

//Option<Point<P>>は、グラフにその点が含まれる場合はSome(Point)を表し、含まれない場合はNoneを表す
#[derive(Debug)]
pub struct Point<P> {
    pub value: P,
    pub cross: usize,
}

#[derive(Debug)]
pub struct Endpoint<P>(P, usize, usize);

// 一直線の（曲がることもある）グラフ
#[derive(Debug)]
pub struct StraightGraph<P> {
    pub length: usize,
    pub points: Vec<Vec<Option<Point<P>>>>,
    pub graph: Graph<P>,
    pub endpoints: Vec<Endpoint<P>>,
    pub curve_counts: usize,
}

// 一般的なグラフ
#[derive(Debug)]
pub struct BranchedGraph<P> {
    pub length: usize,
    pub points: Vec<Vec<Option<Point<P>>>>,
    pub graph: Graph<P>,
}

impl<P: Copy + Hash> BranchedGraph<P> {
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

        let included_points: Vec<Vec<Option<Point<P>>>> = (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| {
                        if graph[i][j].len() != 0 {
                            Some(Point {
                                value: points[i][j],
                                cross: graph[i][j].len(),
                            })
                        } else {
                            None
                        }
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

#[allow(dead_code)]
impl<P: Copy> StraightGraph<P> {
    pub fn new(
        (rows, cols): (usize, usize),
        graph: &Graph<P>,
        points: &Points<P>,
    ) -> Result<StraightGraph<P>, &'static str> {
        let mut endpoints: Vec<Endpoint<P>> = Vec::new();
        let mut curve_counts: usize = 0;
        //バリデーション & endpointsの探索
        for i in 0..rows {
            for j in 0..cols {
                // crossが3以上ならStraightGraphではない
                if graph[i][j].len() > 2 {
                    return Err("crossが3以上(直線でない)グラフが存在します！");

                // crossが1なら端点
                } else if graph[i][j].len() == 1 {
                    endpoints.push(Endpoint(points[i][j], i, j));
                // crossが2ならcurveかどうかを判定し、加算
                } else if graph[i][j].len() == 2 {
                    let v: Vec<_> = graph[i][j].iter().collect();
                    if v[0].1 != v[1].1 && v[0].2 != v[1].2 {
                        curve_counts += 1;
                    }
                }
            }
        }

        //グラフの長さを測定
        let length = (0..rows)
            .map(|i| {
                (0..cols).map(|j| graph[i][j].len()).sum::<usize>() // 各行の合計を計算
            })
            .sum::<usize>()
            / 2; // 全行の合計を計算

        //含まれる格子点を探索
        let included_points: Vec<Vec<Option<Point<P>>>> = (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| {
                        if graph[i][j].len() != 0 {
                            Some(Point {
                                value: points[i][j],
                                cross: graph[i][j].len(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        return Ok(StraightGraph {
            length,
            points: included_points,
            graph: graph.clone(),
            curve_counts,
            endpoints,
        });
    }
}

// 盤面からBranchGraphの配列を取得する関数
#[allow(dead_code)]
pub fn extract_branched_graph_from_board<E: PartialEq + Default, P: Clone + Copy + Eq + Hash>(
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
        let mut graph: Graph<P> = vec![vec![HashSet::new(); v_cols]; h_rows];
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
        let mut graph: Graph<P> = vec![vec![HashSet::new(); v_cols]; h_rows];
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

// 盤面からStraightGraphの配列を取得する関数
#[allow(dead_code)]
pub fn extract_straight_graph_from_board<E: PartialEq + Default, P: Clone + Copy + Eq + Hash>(
    edges: &Edges<E>,
    points: &Points<P>,
) -> Vec<StraightGraph<P>> {
    let mut straight_graphs: Vec<StraightGraph<P>> = Vec::new();
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
        let mut graph: Graph<P> = vec![vec![HashSet::new(); v_cols]; h_rows];
        dfs_h(
            (i, j),
            &mut seen_h,
            &mut seen_v,
            points,
            &mut graph,
            h_cols,
            v_rows,
        );
        let straight_graph = StraightGraph::new((h_rows, v_cols), &graph, points).unwrap();
        straight_graphs.push(straight_graph);
    }

    while let Some((i, j)) = find_first_false(&seen_v) {
        let mut graph: Graph<P> = vec![vec![HashSet::new(); v_cols]; h_rows];
        dfs_v(
            (i, j),
            &mut seen_h,
            &mut seen_v,
            points,
            &mut graph,
            h_cols,
            v_rows,
        );
        let straight_graph = StraightGraph::new((h_rows, v_cols), &graph, points).unwrap();
        straight_graphs.push(straight_graph);
    }

    return straight_graphs;
}
