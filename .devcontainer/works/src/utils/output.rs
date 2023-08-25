use crate::constants::constants::DIR;
use crate::dataclass::dataclass::{Cells, Edges, Points};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

pub fn output_e<T: std::fmt::Display>(
    file_name: &str,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<T>,
) -> std::io::Result<()> {
    //ファイルオブジェクトの作成
    let mut file = initialize_file(file_name).unwrap();

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

pub fn output_p<T: std::fmt::Display>(
    file_name: &str,
    epc: &str,
    h: usize,
    w: usize,
    points: &Points<T>,
) -> std::io::Result<()> {
    //ファイルオブジェクトの作成
    let mut file = initialize_file(file_name).unwrap();

    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, points);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}

pub fn output_c<T: std::fmt::Display>(
    file_name: &str,
    epc: &str,
    h: usize,
    w: usize,
    cells: &Cells<T>,
) -> std::io::Result<()> {
    //ファイルオブジェクトの作成
    let mut file = initialize_file(file_name).unwrap();

    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, cells);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}

pub fn output_ep<T: std::fmt::Display>(
    file_name: &str,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<T>,
    points: &Points<T>,
) -> std::io::Result<()> {
    //ファイルオブジェクトの作成
    let mut file = initialize_file(file_name).unwrap();

    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &edges.h);
    create_iterate_strings(&mut content, &edges.v);
    create_iterate_strings(&mut content, points);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}

pub fn output_ec<T: std::fmt::Display>(
    file_name: &str,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<T>,
    cells: &Cells<T>,
) -> std::io::Result<()> {
    //ファイルオブジェクトの作成
    let mut file = initialize_file(file_name).unwrap();

    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &edges.h);
    create_iterate_strings(&mut content, &edges.v);
    create_iterate_strings(&mut content, cells);

    // 区切り文字を挿入
    append_delimiter(&mut content);

    writeln!(file, "{}", content)?;

    Ok(())
}

pub fn output_epc<T: std::fmt::Display>(
    file_name: &str,
    epc: &str,
    h: usize,
    w: usize,
    edges: &Edges<T>,
    points: &Points<T>,
    cells: &Cells<T>,
) -> std::io::Result<()> {
    //ファイルオブジェクトの作成
    let mut file = initialize_file(file_name).unwrap();

    // 記入する文字列の実体の作成
    let mut content = String::new();

    // 最初の行に epc H W の記述
    append_firstline(&mut content, epc, h, w);

    // それぞれの変数をイテレート
    create_iterate_strings(&mut content, &edges.h);
    create_iterate_strings(&mut content, &edges.v);
    create_iterate_strings(&mut content, points);
    create_iterate_strings(&mut content, cells);

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

fn initialize_file(file_name: &str) -> std::io::Result<File> {
    let dir = Path::new(DIR);
    let file_path = dir.join(file_name);
    let file = OpenOptions::new()
        .write(true)
        .create(true) // ファイルが存在しない場合、新しいファイルを作成
        .append(true) // 既存のファイルに追記
        .open(file_path)?;

    return Ok(file);
}
