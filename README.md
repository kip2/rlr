<h1 align="center">
  <strong style="font-size:2em;">rlr</strong>
</h1>
<h4 align="center">Recursion Local Runner</h4>

---

rlr(Recursion Local Runner)は、RecursionCSで公開されている問題を、ローカル環境で実行するためのツールです。

### 特徴

以下の特徴があります。

- ReucsionCSから、問題のテストケースをローカルに取得する。
- 指定されたコマンドで、取得したテストケースのローカルジャッジを実行する。
- ローカルで実行できるので、RecusrionCSのエディタでサポートされていない言語でも問題を解くことが可能。

また、ローカルで実行できるため、好きなエディタで編集、実行が可能となっています。
- `vim`
- `neovim`
- `Emacs`
- `Spacemacs`
- `nano`
- `VScode`
- `Eclipse`
- `IntelliJ`
- `サクラエディタ`
- `秀丸エディタ`
- `メモ帳`

and more...

## Install

### Macの場合

#### インストール

インストールで必要な作業は2つです。

1. ファイルの取得
2. インストールの実行

##### ファイルの取得(Windows/Mac/Linux共通)

ファイルの取得は2つの方法があります。

###### リリースページよりダウンロードする

[リリースページ](https://github.com/kip2/rlr/releases)の最新のリリースから、ダウンロードして下さい。

Linux/Macなら`install.sh`を、Windowsなら`install.bat`をダウンロードして下さい。

###### リポジトリをクローンする

Gitリポジトリをクローン、もしくは`zip`でダウンロードして下さい。

![ダウンロード方法](./pic/pic1.png)

`zip`でダウンロードの場合は、画像の様に、`Download ZIP`をクリックして下さい。
ファイルがローカルに保存されますので、ファイルを解凍して下さい。

ローカルに`git`コマンドがインストールされている場合は、以下の様にクローンすることでもファイルを取得できます。
こちらは`zip`圧縮されていないので、解凍の必要はありません。

```sh
git clone https://github.com/kip2/rlr.git
```

##### インストールの実行

インストール方法は、プラットフォームにより差があります。

##### Mac/Linuxの場合

ダウンロードした`install.sh`に実行権限を与え、実行して下さい。

```sh
# 実行権限の付与
chmod +x install.sh

# インストールの実行
./install.sh
# もしくはbashで起動する
bash install.sh
```

インストールの確認として、バージョン情報の出力を確認して下さい。

```sh
# オプションのVは大文字なことに注意する
rlr -V

# もしくは
rlr --version
```

##### windowsの場合

コマンドプロンプト(`cmd`)、もしくはPowerShellでディレクトリを開き、 `install.bat`を実行して下さい。

```cmd
install.bat
```

インストールの確認として、バージョン情報の出力を確認して下さい。

```sh
# オプションのVは大文字なことに注意する
rlr -V

# もしくは
rlr --version
```

#### アンインストール

#### Mac/Linuxの場合

`uninstall.sh`に実行権限を与え、その後実行して下さい。

```sh
# 実行権限の付与
chmod +x uninstal.sh

# uninstallの実行
./uninstall.sh

# もしくは
bash uninstall.sh
```

#### Windowsの場合

コマンドプロンプト(`cmd`)、もしくはPowerShellでダウンロードした本リポジトリのディレクトリを開き、 `uninstall.bat`を実行して下さい。

```cmd
uninstall.bat
```

## Usage

`rlr`の使用にあたっては、以下の手順が必要です。

1. RecursionCSへの初回ログインを実施する(`login`コマンド)。
2. RecursionCSから問題のテストケースをダウンロードする(`download`コマンド)。
3. 問題を解くコードを、好きなエディタで書く。
4. 3で書いたコードで、問題が解けるかのジャッジを実行する(`judge`コマンド)。

### 初回ログイン

RecursionCSは会員制サイトとなっています。
そのため、あらかじめログインする必要があります。

初回ログインとして、以下のログインコマンドを実施して下さい。

```sh
rlr login 

# もしくは
rlr l
```

コマンド実行後、メールアドレスとパスワードを入力すると、ログイン処理が実行されます。

> **NOTE:** なお、一度ログインをした後は、認証期限がこないうちは再度のログイン実行は不要です。
> 認証期限が切れている場合は再度ログインを実行して下さい。

### テストケースのダウンロード

ログイン後、問題ページのテストケースをダウンロードします。

ダウンロードしたい問題ページを開き、URLを確認、もしくはコピーして以下のコマンドを実行して下さい。

```sh
rlr download "https://recursionist.io/dashboard/problems/1"

# もしくは
rlr d "https://recursionist.io/dashboard/problems/1"
```

ダウンロードされると、`p-問題番号`の名前でディレクトリを作成し、その中にテストケースを記載したファイルの`testcase`ディレクトリが作成されます。

```sh
└── p-1
    └── testcase
        ├── testcase-1.in
        ├── testcase-1.out
        ├── testcase-2.in
        ├── testcase-2.out
        ├── testcase-3.in
        ├── testcase-3.out
        ├── testcase-4.in
        ├── testcase-4.out
        ├── testcase-5.in
        └── testcase-5.out
```

> **NOTE:** `template`ディレクトリが配置されている場合、その中身も一緒にコピーしてくれます。
> 解きたい言語のテンプレートファイルなどを用意して置くと、毎回ファイル作成して環境を構築する手間が省けます。
> `template`の例を含め、環境構築の方法については[各言語でのrlrの使い方](./document/usage/)に、言語毎の例が記載されていますので、参照して下さい。

### 問題を解くコードを好きなエディタで記載する

好きなエディタを用いて、問題を解くコードを記載して下さい。

初心者の方向けに、問題の解き方で基礎となる考え方を記載しました。

詳細は[問題の解き方](./document/問題の解き方.md)を参照して下さい。

### ジャッジを行う

問題を解くコードを記載したら、テストケースを用いてジャッジを行います。

ジャッジを実行するには、ダウンロードしたテストケース(`p-1`)が存在するディレクトリで、`judge`コマンドを使用する必要があります。

```sh
└── p-1   # <- このディレクトリに移動して実行する。
    └── testcase
        ├── testcase-1.in
        ├── testcase-1.out
        ├── testcase-2.in
        ├── testcase-2.out
        ├── testcase-3.in
        ├── testcase-3.out
        ├── testcase-4.in
        ├── testcase-4.out
        ├── testcase-5.in
        └── testcase-5.out
```

具体的には、`p-1`ディレクトリに移動してから`judge`コマンドを実行すると良いでしょう。

```sh
# p-1の例
cd p-1
```

`judege`コマンドは以下のような構造になっています。

```sh
# <command>はシェルで実行可能なコマンドを実行して下さい。
rlr judge "<command>"
```

#### 指定できるcommandについて

指定できるコマンドは、シェルやコマンドプロンプトで実行できるコマンドを指定して下さい。

例えば、`python`コードを`main.py`に記載した場合は、以下のような`command`を指定して下さい。


```sh
# pythonの例。main.pyの実行を行う。
rlr judge "python3 main.py"
```

> **NOTE:** 指定する`command`は必ず文字列である必要があります。`""`で囲って下さい。

> **NOTE:** `rlr`は、`input`となるテストケースを標準入力で受け取り、`judge`で指定した`command`に対してパイプ処理で受け渡して処理した後、標準出力を`output`のケースと比較します。
> つまり、標準入力を受け取って出力可能な`command`なら何でも使えます。手元で実行ができるならどんな言語でも大丈夫ということです。

処理が実行され、各テストケースの結果が表示されます。

### その他

コマンドを忘れた場合は、`help`コマンドを実行して下さい。

> **NOTE:** 他のコマンドと違って、`-`が頭につくことに注意して下さい。

```sh
rlr -h

# 詳細なhelp
rlr --help
```

#### cookie-path取得コマンド

保存したCookieパスを取得するコマンドがあります。
`rlr`を利用する際には基本的に使用する必要はありませんが、確認やデバッグの際に活用して下さい。

```sh
rlr cookie-path
```

## ご協力のお願い

当リポジトリは、皆さんからのフィードバックをお待ちしています。

![Uncle kip2](./pic/Uncle_kip2.png)

`rlr`はまだ作成されたばかりの未熟なアプリです。 そのため、ぜひ、皆さんのフィードバックをいただければと考えています。

- 使い勝手で改善してほしい部分,
- 欲しい機能,

などがあればissueを投げて下さい。

また、ツールとして最低限動く部分については作成しましたが、実際に各言語でどの様に実行するか、といったところのドキュメントが未整備となっています。

現在、いくつかの言語の例は追加していますが、Recursion側がサポートしている言語のドキュメントも未整備の状態です。

また、自分の好きな言語での解き方ドキュメントも歓迎しています。

手元で実行さえできればどんな言語でも大丈夫です(`COBOL`, `FORTRAN`, `zig`, `Hasklell`, `brainf*ck`, etc...)。

ご協力いただける方は、ぜひドキュメント追加のPull Requestを投げて下さい。

「チーム開発参加怖いけど、PRの練習をしたい！」というかつての私のような方でも歓迎しています。

Pull Requestの出し方についてもまとめましたので、ご協力いただける方は以下のドキュメントをご参照下さい。

[PRのやり方](https://github.com/kip2/rlr/tree/main/document/usage)

## Special Thanks

`rlr`は [oj](https://github.com/online-judge-tools/oj) に、インスパイアされて作成しています。

## License

This project is licensed under the MIT License.