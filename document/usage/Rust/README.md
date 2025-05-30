# Rustでの問題の解き方

Rustを使って問題を解く際のやり方を記載します。

## インストールと使い方

`rlr`の基本的な使用方法は、[README](../../../README.md)を参照して下さい。

## 準備

テストケース取得と、実行環境の準備が必要です。
これらを自動で実行できるように、シェルスクリプトを用意しました。

[setup.sh](./setup.sh)

```sh
# 実行権限を与える
chmod +x setup.sh

# 実行する
# 引数に問題の番号を指定する
# 問題の番号は問題ページのURL末尾の番号です(例：https://recursionist.io/dashboard/problems/1 の 1)
./setup.sh 1
```

実行すると、以下のようにディレクトリが準備されます。

```sh
$ tree p-1
.
├── .vscode
│   └── settings.json
└── p-1
    ├── run.sh
    ├── sample
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── testcase
        ├── testcase-1.in
        ├── testcase-1.out
        ├── testcase-2.in
        ├── testcase-2.out
        ├── testcase-3.in
        └── testcase-3.out
```

`.vscode`は、`rust-analyzer`に作成したプロジェクトを認識させるための設定です。

詳しくは以下の記事を参照して下さい。

[rust-nalayzerがうまく働かないときの対処法](https://zenn.dev/kip2/articles/pinning-rust-analyzer-version)

## 問題を解くコードを書く

`sample/src/main.rs`に問題を解くコードを書いて下さい。

どのようなコードを記載したらよいかは、[問題の解き方](../../問題の解き方.md)を参照して下さい。

## 実行

実行用のスクリプトを用意しました。
`run.sh`を実行すると、`rlr`での実行が行われます。

```sh
# ディレクトリを移動
cd p-1

# 実行権限を与える
chmod +x run.sh

# 実行する
./run.sh
```

`参考: run.shの内容`
```sh
#!/bin/bash -v

cd sample
cargo build --release

cd ..
rlr judge ./sample/target/release/sample
```
