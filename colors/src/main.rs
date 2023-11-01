# Base on https://github.com/rustyworks/colorized/blob/master/colorized.py
# But Rust doesn't have octal escape sequences. You have to use hexadecimal.
# Ref: https://stackoverflow.com/a/69982036
# Ref: https://github.com/rust-lang/rust/issues/30491

const BLACK: &str = "\x1b[0;30m";
const RED: &str = "\x1b[0;31m";
const GREEN: &str = "\x1b[0;32m";
const ORANGE: &str = "\x1b[0;33m";
const BLUE: &str = "\x1b[0;34m";
const PURPLE: &str = "\x1b[0;35m";
const CYAN: &str = "\x1b[0;36m";
const DARK_GRAY: &str = "\x1b[1;30m";
const LIGHT_RED: &str = "\x1b[1;31m";
const LIGHT_GREEN: &str = "\x1b[1;32m";
const YELLOW: &str = "\x1b[1;33m";
const LIGHT_BLUE: &str = "\x1b[1;34m";
const LIGHT_PURPLE: &str = "\x1b[1;35m";
const LIGHT_CYAN: &str = "\x1b[1;36m";
const NO_COLOR: &str = "\x1b[0m";

fn main() {
    let colors: Vec<&str> = vec![
        BLACK,
        RED,
        GREEN,
        ORANGE,
        BLUE,
        PURPLE,
        CYAN,
        DARK_GRAY,
        LIGHT_RED,
        LIGHT_GREEN,
        YELLOW,
        LIGHT_BLUE,
        LIGHT_PURPLE,
        LIGHT_CYAN,
        NO_COLOR,
    ];

    for color in colors {
        print!("{} COLOR", color);
        println!("{}", NO_COLOR);
    }
}
