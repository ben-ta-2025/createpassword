# createpassword
コマンドでパスワード生成

# createpassword
Rust製のコマンドライン・パスワード生成ツールです。  
指定した文字数・複雑性（大文字・数字・記号の有無）で安全なパスワードを生成します。

# 使い方
## ビルド
`cargo build --release`
## 実行例
`./createpassword --length 16 --number --symbol --uppercase`
## または短縮形で
`./createpassword -l 16 -n -s -u`
## オプション
| オプション | 説明 | 省略時 |
|:---|:---|:---|
| -l, --length | パスワードの文字数 | 12 |
| -n, --number | 数字を含める | 含めない |
| -s, --symbol | 記号を含める | 含めない |
| -u, --uppercase | 大文字を含める | 含めない |

# A 特徴
指定した種類（大文字・数字・記号）は必ず1文字以上含まれます  
引数なしで実行するとヘルプが表示されます  
Rust製で高速・安全  
