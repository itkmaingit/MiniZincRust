# Documentation

これは「パズルルールの自動作成」というテーマの研究開発に用いる計算環境及びコードのリポジトリです。本研究を開始するにあたっての注意をここに記します。

## Requirements

まず、本研究を始めるにあたって以下のツールが必要になります。Windows 上でのみの動作を確認しているので Mac, 及び LinuxOS の方は Windows を用意してください。また、WSL2 上に Docker を建てるので、WSL2 のインストールを行っておいてください。WSL2 のインストール方法についての記述は省きます。

- [VSCode](https://code.visualstudio.com/)
- Remote Development(VSCode の拡張機能です。)
- [Docker (Desktop for Windows)](https://docs.docker.com/desktop/install/windows-install/)
- [Git](https://git-scm.com/downloads)

これらをインストールしてから以下の Setup を読んでください。Git 等の基本コマンドは別途習得していることを前提とします。Docker については学習しなくても使用できるようにレイヤー化は行っていますが、分かる方はカスタマイズしても良いかもしれません。

## Setup

以下のコマンドを実行してください。

```powershell
git clone https://github.com/<your-name>/MiniZincRust.git
```

そのあと、VSCode 内で MiniZincRust フォルダを開いてから F1 ボタンを押し、`Dev Containers: Reopen in Container`というコマンドを実行して下さい。

しばらく待つと、Docker 環境の中に入れているはずです。後は Docker 環境内で以下のコマンドを実行してください。

```bash
sudo chown -R node /usr/local/cargo/registry/
```

これで環境構築は終了です。

## Overview

以下では研究における順序と、それを実行するためのコマンド、更にディレクトリ構成についての説明を行います。しかし、この節の最後の方にあるコマンド集を実行すればディレクトリ構成については強く意識しないでも済むはずです。これらコマンドの意味を知りたい場合は Docker 内にて`code ~/.bashrc`と入力してください。

### Commands

全てのファイルは/home/node/works/data ディレクトリに出力されます。出力場所を変更したいときは~/.bashrc 内の`WORKDIR`を編集してください。

上で触れたように、順序としては

`pgt` -> (data.csv を編集し、初期状態の決定) -> `pg` -> `mas` -> `pcs` -> `cr` -> `pv`

の順番でコマンドを実行することになるでしょう。あまりに名称が短いと感じた場合は~/.bashrc 内で適宜関数名を変えればよいです。

以下、コマンド内の`puzzle_rule`は全て data ディレクトリ直下に作成するディレクトリ名を指すこととします。文字通りパズルルールに即した名称をつけるのが良いでしょう。

#### pgt(python generate template)

```bash
pgt puzzle_rule epc H W
```

指定された epc に即した、H 行 W 列の data.csv を作成します。h, v, p,c のデフォルト値も決めることができます。

#### pg(python generate)

```bash
pg puzzle_rule
```

作成した data.csv から初期状態を dzn 形式に変換します。実行するだけで良いので、特に使用者が内部実装を気にする必要はないでしょう。
data.csv で作成した初期状態がどのようなものかが data.txt、コンソールに出力されます。どのような盤面になっているかはちゃんと確認しましょう。正しいデータを与えることはとても大切です。

#### mar(minizinc all solve)

```bash
mar puzzle_rule
```

実際に MiniZinc を用いて解を求めます。時間のかかる場合もありますが気長に待ちましょう。

#### pcs(python candidate splitter)

```bash
pcs puzzle_rule
```

MiniZinc から出力された candidates.txt を次に行う Rust での処理のためにファイル分割を行います。candidates 直下に 0.txt, 1.txt...と凄まじい勢いでファイルが生成されますが気にする必要はありません。

#### cr(cargo run)

```bash
cr puzzle_rule
```

MiniZinc で求めた解が真にパズルルールを満たしているかを Rust のプログラムによって確かめます。メモリアロケーションエラーが発生する場合は物理的にメモリを増やすか、`threadpool`クレートを使用してスレッド数に制限をかけるとよいでしょう。基本的に本プロジェクトでは 18 と設定しています。スレッド数の上限は`nproc`コマンドで確かめることができます。

#### pv(python visualize)

```bash
pv puzzle_rule
```

Rust で求めた解を視覚化します。どんな解があるのか、また書いたコードが実際にあっているのかを確かめるためにとりあえず実行しておけばよいでしょう。

#### allrun

```bash
allrun puzzle_rule
```

`mar` -> ... -> `pv`を一括で行うコマンドです。ある程度慣れてきたらこのコマンドで色々なパズルルールを検証するとよいでしょう。
