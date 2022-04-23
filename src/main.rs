use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let witdh = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), witdh, &mut writer).unwrap();
}
