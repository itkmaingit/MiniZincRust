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

以下では研究における順序と、それを実行するためのコマンド、更にディレクトリ構成についての説明を行います。しかし、この説の最後の方にあるコマンド集を実行すればディレクトリ構成については強く意識しないでも済むはずです。これらコマンドの意味を知りたい場合は Docker 内にて`code ~/.bashrc`と入力してください。
