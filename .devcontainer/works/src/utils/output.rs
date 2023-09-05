use crate::dataclass::dataclass::{Cells, Edges, Points};
use std;
use std::fs::File;
use std::io::prelude::*;

pub fn output_e<E: std::fmt::Display>(
    mut file: std::sync::MutexGuard<'_, File>,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<E>,
) -> std::io::Result<()> {
    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &edges.h);
    create_iterate_strings(&mut content, &edges.v);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}
pub fn output_p<P: std::fmt::Display>(
    mut file: std::sync::MutexGuard<'_, File>,
    epc: &str,
    h: usize,
    w: usize,
    points: &Points<P>,
) -> std::io::Result<()> {
    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &points);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}
pub fn output_c<C: std::fmt::Display>(
    mut file: std::sync::MutexGuard<'_, File>,
    epc: &str,
    h: usize,
    w: usize,
    cells: &Cells<C>,
) -> std::io::Result<()> {
    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &cells);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}
pub fn output_ep<E: std::fmt::Display, P: std::fmt::Display>(
    mut file: std::sync::MutexGuard<'_, File>,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<E>,
    points: &Points<P>,
) -> std::io::Result<()> {
    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &edges.h);
    create_iterate_strings(&mut content, &edges.v);
    create_iterate_strings(&mut content, &points);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}
pub fn output_ec<E: std::fmt::Display, C: std::fmt::Display>(
    mut file: std::sync::MutexGuard<'_, File>,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<E>,
    cells: &Cells<C>,
) -> std::io::Result<()> {
    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &edges.h);
    create_iterate_strings(&mut content, &edges.v);
    create_iterate_strings(&mut content, &cells);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}
pub fn output_epc<E: std::fmt::Display, P: std::fmt::Display, C: std::fmt::Display>(
    mut file: std::sync::MutexGuard<'_, File>,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<E>,
    points: &Points<P>,
    cells: &Cells<C>,
) -> std::io::Result<()> {
    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &edges.h);
    create_iterate_strings(&mut content, &edges.v);
    create_iterate_strings(&mut content, &points);
    create_iterate_strings(&mut content, &cells);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}

fn append_firstline<T: std::fmt::Display>(content: &mut String, epc: &str, h: T, w: T) {
    let firstline = format!("{} {} {}\n", epc, h.to_string(), w.to_string());
    content.push_str(&firstline);
}

fn append_delimiter(content: &mut String) {
    content.push_str("-----------------------------");
}

fn create_iterate_strings<T: std::fmt::Display>(content: &mut String, matrix: &Vec<Vec<T>>) {
    for row in matrix {
        content.push_str(
            &row.iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        );
        content.push('\n');
    }
}
