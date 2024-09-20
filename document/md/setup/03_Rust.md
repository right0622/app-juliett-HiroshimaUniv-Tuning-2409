# Rust 開発環境構築ガイド (VSCode 編)

このドキュメントでは、Rust の開発環境を Visual Studio Code（VSCode）上で構築する手順を説明します。Rust の基本的なツールセットに加えて、VSCode のおすすめ拡張機能も紹介します。

---

## Rust のインストール

1. Rustup のインストール
   Rust の公式ツールチェーンマネージャである `rustup` をインストールします。

   [Rust をインストール - Rustプログラミング言語](https://www.rust-lang.org/ja/tools/install)

2. Rust のツールチェーンを確認
   インストールが完了したら、以下のコマンドで Rust が正しくインストールされたか確認します。

   ```bash
   rustc --version
   ```

3. Cargo のインストール確認
   Cargo は Rust のパッケージマネージャです。次のコマンドで Cargo がインストールされているか確認します。

   ```bash
   cargo --version
   ```

## Rust 拡張機能のインストール

VSCode では Rust 開発における便利な拡張機能が提供されています。

- Rust Analyzer
   [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) は Rust プロジェクトに対する高度なインテリセンスを提供しています。こちらは非常に高速で、公式推奨のツールです。
