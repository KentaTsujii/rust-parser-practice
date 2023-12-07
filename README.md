# rust-parser-practice

rust手習い用リポジトリ。テーマとして、sqlのnon validating parserを作ってみる。
いつも調べてばかりで、手を動かせてないので、とりあえずリポジトリだけ作ってみる。

## TODO

* まずはrustでファイル読み込み & print

## メモ

* `cargo new` と `cargo init` の違い
  `cargo init` はカレントディレクリをrootディレクリとしてプロジェクトをセットアップする
  `cargo new` は引数にプロジェクト名を指定する必要があり、カレントプロジェクトの下にフォルダを作成して、そこにプロジェクトのひな形を生成する。

* Cargo.lockの扱い
  libではgitignoreにいれる。binではいれないでバージョン管理する

