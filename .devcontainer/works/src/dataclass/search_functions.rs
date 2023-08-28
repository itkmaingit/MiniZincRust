use crate::dataclass::dataclass::{Attribute, Points, Variable};

pub type Graph<P> = Vec<Vec<Vec<(P, usize, usize)>>>;

pub fn is_falsy<T: PartialEq + Default>(value: &T) -> bool {
    *value == T::default()
}

pub fn find_first_false(seen: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
    for (i, row) in seen.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if !value {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn dfs_h<P: Copy>(
    (i, j): (usize, usize),
    seen_h: &mut Vec<Vec<bool>>,
    seen_v: &mut Vec<Vec<bool>>,
    points: &Points<P>,
    graph: &mut Graph<P>,
    h_cols: usize,
    v_rows: usize,
) {
    let mut candidates: Vec<Variable> = Vec::new();
    seen_h[i][j] = true;
    graph[i][j].push((points[i][j + 1], i, j + 1));
    graph[i][j + 1].push((points[i][j], i, j));

    // 上にはみ出さないようにする
    if i > 0 {
        if !seen_v[i - 1][j] {
            seen_v[i - 1][j] = true;
            candidates.push(Variable(Attribute::V, i - 1, j));
        }
        if !seen_v[i - 1][j + 1] {
            seen_v[i - 1][j + 1] = true;
            candidates.push(Variable(Attribute::V, i - 1, j + 1));
        }
    }

    // 下にはみ出さないようにする
    if i < v_rows {
        if !seen_v[i][j] {
            seen_v[i][j] = true;
            candidates.push(Variable(Attribute::V, i, j));
        }
        if !seen_v[i][j + 1] {
            seen_v[i][j + 1] = true;
            candidates.push(Variable(Attribute::V, i, j + 1));
        }
    }

    // 左にはみ出さないようにする
    if j > 0 && seen_h[i - 1][j] {
        seen_h[i][j] = true;
        candidates.push(Variable(Attribute::H, i - 1, j));
    }

    // 右にはみ出さないようにする
    if j < h_cols && seen_h[i + 1][j] {
        seen_h[i][j] = true;
        candidates.push(Variable(Attribute::H, i + 1, j));
    }

    for candidate in candidates {
        if candidate.0 == Attribute::H {
            dfs_h(
                (candidate.1, candidate.2),
                seen_h,
                seen_v,
                points,
                graph,
                h_cols,
                v_rows,
            )
        } else if candidate.0 == Attribute::V {
            dfs_v(
                (candidate.1, candidate.2),
                seen_h,
                seen_v,
                points,
                graph,
                h_cols,
                v_rows,
            )
        }
    }
}

pub fn dfs_v<P: Copy>(
    (i, j): (usize, usize),
    seen_h: &mut Vec<Vec<bool>>,
    seen_v: &mut Vec<Vec<bool>>,
    points: &Points<P>,
    graph: &mut Graph<P>,
    h_cols: usize,
    v_rows: usize,
) {
    let mut candidates: Vec<Variable> = Vec::new();
    seen_h[i][j] = true;
    graph[i][j].push((points[i + 1][j], i + 1, j));
    graph[i + 1][j].push((points[i][j], i, j));

    // 上にはみ出さないようにする
    if i > 0 && !seen_v[i - 1][j] {
        seen_v[i - 1][j] = true;
        candidates.push(Variable(Attribute::V, i - 1, j));
    }

    // 下にはみ出さないようにする
    if i < v_rows && !seen_v[i + 1][j] {
        seen_v[i + 1][j] = true;
        candidates.push(Variable(Attribute::V, i + 1, j));
    }

    // 左にはみ出さないようにする
    if j > 0 {
        if !seen_h[i][j - 1] {
            seen_h[i][j - 1] = true;
            candidates.push(Variable(Attribute::H, i, j - 1));
        }

        if !seen_h[i + 1][j - 1] {
            seen_h[i + 1][j - 1] = true;
            candidates.push(Variable(Attribute::H, i + 1, j - 1));
        }
    }

    // 右にはみ出さないようにする
    if j < h_cols {
        if !seen_h[i][j] {
            seen_h[i][j] = true;
            candidates.push(Variable(Attribute::H, i, j));
        }

        if !seen_h[i + 1][j] {
            seen_h[i + 1][j] = true;
            candidates.push(Variable(Attribute::H, i + 1, j));
        }
    }

    for candidate in candidates {
        if candidate.0 == Attribute::H {
            dfs_h(
                (candidate.1, candidate.2),
                seen_h,
                seen_v,
                points,
                graph,
                h_cols,
                v_rows,
            )
        } else if candidate.0 == Attribute::V {
            dfs_v(
                (candidate.1, candidate.2),
                seen_h,
                seen_v,
                points,
                graph,
                h_cols,
                v_rows,
            )
        }
    }
}
