Rustで書いた落ちものパズルゲーム。
コンソールでプレイできる。

# 構造
## src/gameplay
ゲームデータとロジック。utilityに依存

## src/console_renderer
コンソールで表示するためのレンダラー。utilityに依存

## src/console_renderer_sender
レンダラーにゲームデータを送る。gameplay、console_renderer、utilityに依存

## src/utility
いろんなところで使われている便利機能

## src/console_key_input.rs
コンソールでのキー入力を受け付ける。gameplay内に依存している。

## src/lib.rs / src/main.rs
エントリポイント

