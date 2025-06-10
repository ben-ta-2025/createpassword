use clap::Parser;
use rand::Rng;

/// パスワード生成プログラム
#[derive(Parser)]
/// #[command(arg_required_else_help = true)]
struct Cli {
    /// 生成するパスワードの文字数
    #[clap(short = 'l', long = "length", default_value_t = 12)]
    length: usize,

    /// 数字を含める
    #[clap(short = 'n', long = "number")]
    number: bool,

    /// 記号を含める
    #[clap(short = 's', long = "symbol")]
    symbol: bool,

    /// 大文字を含める
    #[clap(short = 'u', long = "uppercase")]
    uppercase: bool,
}

fn main() {
    let args = Cli::parse();

    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()-_=+[]{};:,.<>?";

    let mut charset = String::from(lowercase);
    let mut required_chars = Vec::new();

    if args.uppercase {
        charset.push_str(uppercase);
        // 必ず1文字含める
        required_chars.push(random_char(uppercase));
    }
    if args.number {
        charset.push_str(numbers);
        required_chars.push(random_char(numbers));
    }
    if args.symbol {
        charset.push_str(symbols);
        required_chars.push(random_char(symbols));
    }

    if args.length < required_chars.len() {
        eprintln!("エラー: パスワード長より多くの必須文字種が指定されています。lengthを増やしてください。");
        std::process::exit(1);
    }

    // 必須文字数を引いた残りをランダムに選ぶ
    let mut rng = rand::thread_rng();
    let mut password_chars: Vec<char> = (0..(args.length - required_chars.len()))
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    // 必須文字を追加
    password_chars.extend(required_chars);

    // シャッフルしてランダムな順番に
    use rand::seq::SliceRandom;
    password_chars.shuffle(&mut rng);

    let password: String = password_chars.into_iter().collect();

    println!("生成されたパスワード: {}", password);
}

fn random_char(charset: &str) -> char {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..charset.len());
    charset.chars().nth(idx).unwrap()
}