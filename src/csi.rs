use termios;

// CSI(Control Sequence Introducer) of Escapse sequence

use crate::{echo_off, echo_on};
use std::io::{self, Read, Write};

pub type Result<T> = std::result::Result<T, std::io::Error>;

#[macro_export]
macro_rules! csi {
    ($( $s:expr ),*) => { concat!("\x1b[", $( $s ),*) };
}

fn _flush(w: &mut Write) {
    w.flush().unwrap();
}

// avoid zero
fn _nz(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n
    }
}

/// CUU: cursor up
pub fn cuu(w: &mut Write, n: u32) -> Result<()> {
    // w.write_fmt(format_args!(csi!("{}A"), _nz(n)));
    w.write_all(format!(csi!("{}A"), _nz(n)).as_bytes())?;
    Ok(())
}

/// CUD: cursor down
pub fn cud(w: &mut Write, n: u32) -> Result<()> {
    w.write_all(format!(csi!("{}B"), _nz(n)).as_bytes())?;
    Ok(())
}

/// CUF: cursor forward
pub fn cuf(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}C"), _nz(n)).as_bytes());
}

/// CUB: cursor back
pub fn cub(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}D"), _nz(n)).as_bytes());
}

/// CNL: cursor next line
pub fn cnl(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}E"), _nz(n)).as_bytes());
}

/// CPL: cursor previous line
pub fn cpl(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}F"), _nz(n)).as_bytes());
}

/// CHA: cursor horizontal absolute
pub fn cha(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}G"), _nz(n)).as_bytes());
}

/// CUP: cursor position
pub fn cup(w: &mut Write, row: u32, col: u32) {
    w.write(format!(csi!("{};{}H"), _nz(row), _nz(col)).as_bytes());
}

pub enum EdClear {
    FromCurToEos = 0,
    FromCurToBos = 1,
    EntireScreen = 2,
    EntireScreenAndDeleteAllScrollBuffer = 3,
}
/// ED: erase in display
/// If n is 0 (or missing), clear from cursor to end of screen.
/// If n is 1, clear from cursor to beginning of the screen.
/// If n {\displaystyle n} n is 2, clear entire screen (and moves cursor to upper left on DOS ANSI.SYS).
/// If n {\displaystyle n} n is 3, clear entire screen and delete all lines saved in the scrollback buffer (this feature was added for xterm and is supported by other terminal applications).
pub fn ed(w: &mut Write, n: u32) {
    if n <= 3 {
        w.write(format!(csi!("{}J"), n).as_bytes());
    }
}

pub enum ElClear {
    FromCurToEol = 0,
    FromCurToBol = 1,
    EntireLine = 2,
}
/// EL: erase in line
/// If n is 0 (or missing), clear from cursor to the end of the line.
/// If n is 1, clear from cursor to beginning of the line.
/// If n is 2, clear entire line. Cursor position does not change.
pub fn el(w: &mut Write, n: u32) {
    if n <= 2 {
        w.write(format!(csi!("{}K"), n).as_bytes());
    }
}

/// SU: scroll up
pub fn su(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}S"), _nz(n)).as_bytes());
}

/// SD: scroll down
pub fn sd(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}T"), _nz(n)).as_bytes());
}

pub enum SgrCode {
    Normal = 0,
    Bold = 1,
    Faint = 2,
    Italic = 3,
    Underline = 4,
    SlowBlink = 5,
    RapidBlink = 6,
    Inverse = 7,
    Invisible = 8,
    Strikethrough = 9,
    PrimaryFont = 10,
    AltFont1 = 11,
    AltFont2 = 12,
    AltFont3 = 13,
    AltFont4 = 14,
    AltFont5 = 15,
    AltFont6 = 16,
    AltFont7 = 17,
    AltFont8 = 18,
    AltFont9 = 19,
    DoubleUnderline = 21,
    BoldFaintOff = 22,
    ItalicOff = 23,
    UnderlineOff = 24,
    Steady = 25,   // not blinking
    Positive = 27, // not inverse
    Visible = 28,
    StrikethroughOff = 29,
    FgColorBlack = 30,
    FgColorRed = 31,
    FgColorGreen = 32,
    FgColorYellow = 33,
    FgColorBlue = 34,
    FgColorMagenta = 35,
    FgColorCyan = 36,
    FgColorWhite = 37,

    // FgColor8bit(u8),
    // FgColor24bit((u8, u8, u8)),
    FgColorDefault = 39,
    BgColorBlack = 40,
    BgColorRed = 41,
    BgColorGreen = 42,
    BgColorYellow = 43,
    BgColorBlue = 44,
    BgColorMagenta = 45,
    BgColorCyan = 46,
    BgColorWhite = 47,
    // BgColor8bit(u8),
    // BgColor24bit((u8, u8, u8)),
    BgColorDefault = 49,
    Frame = 51,
    Encircle = 52,
    Overline = 53,
    FrameEncircleOff = 54,
    OverlineOff = 55,
    RightSideLine = 60,
    RightSideDoublLine = 61,
    LeftSideLine = 62,
    LeftSideDoublLine = 63,
    DoubleStrikethrough = 64,
    LineOff = 65,
    FgColorBrightBlack = 90,
    FgColorBrightRed = 91,
    FgColorBrightGreen = 92,
    FgColorBrightYellow = 93,
    FgColorBrightBlue = 94,
    FgColorBrightMagenta = 95,
    FgColorBrightCyan = 96,
    FgColorBrightWhite = 97,
    BgColorBrightBlack = 100,
    BgColorBrightRed = 101,
    BgColorBrightGreen = 102,
    BgColorBrightYellow = 103,
    BgColorBrightBlue = 104,
    BgColorBrightMagenta = 105,
    BgColorBrightCyan = 106,
    BgColorBrightWhite = 107,
}
/// SGR: select graphic rendition
/// SGR parameters: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
pub fn sgr(w: &mut Write, c: SgrCode) {
    w.write(format!(csi!("{}m"), (c as i32)).as_bytes());
}

pub enum SgrColor {
    FgColor8bit(u8),
    FgColor24bit((u8, u8, u8)),
    BgColor8bit(u8),
    BgColor24bit((u8, u8, u8)),
}
pub fn sgr_color(w: &mut Write, c: SgrColor) {
    w.write(format!(csi!("{}m"),
        match c {
            SgrColor::FgColor8bit(color) => format!("38;5;{}", color),
            SgrColor::FgColor24bit((r, g, b)) => format!("38;2;{};{};{}", r, g, b),
            SgrColor::BgColor8bit(color) => format!("48;5;{}", color),
            SgrColor::BgColor24bit((r, g, b)) => format!("48;2;{};{};{}", r, g, b),
        }
    ).as_bytes());
}

/// DSR: device status report
/// return (row, col)
pub fn dsr(w: &mut Write) -> Option<(u32, u32)> {
    let oldstat: Box<termios::Termios> = Box::new(echo_off());
    w.write(csi!("6n").as_bytes());
    _flush(w);
    let (mut row, mut col, mut tmp) = (0u32, 0u32, 0u32);
    let s = io::stdin();
    // => "[${row};${col}R"
    for b in s.lock().bytes().filter_map(|v| v.ok()) {
        match b {
            // '0' ... '9'
            0x30...0x39 => {
                tmp = tmp * 10 + u32::from(b - 0x30);
            }
            // ';'
            0x3b => {
                row = tmp;
                tmp = 0;
            }
            // 'R'
            0x52 => {
                col = tmp;
                break;
            }
            _ => {}
        }
    }
    echo_on(&*oldstat);
    Some((row, col))
}

/// SCP: save cursor position
pub fn scp(w: &mut Write) {
    w.write(csi!("s").as_bytes());
}

/// RCP: restore cursor position
pub fn rcp(w: &mut Write) {
    w.write(csi!("u").as_bytes());
}

/// SM: set mode
/// mode: http://ttssh2.osdn.jp/manual/ja/about/ctrlseq.html#mode
pub fn sm(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}h"), n).as_bytes());
}

/// RM: reset mode
pub fn rm(w: &mut Write, n: u32) {
    w.write(format!(csi!("{}l"), n).as_bytes());
}

pub enum DecscusrStyle {
    BlinkingBlock = 1,
    SteadyBlock = 2,
    BlinkingUnderline = 3,
    SteadyUnderline = 4,
    BlinkingBar = 5,
    SteadyBar = 6,
}
/// DECSCUSR: set cursor style
/// 0,1: blinking block
/// 2: steady block
/// 3: blinking underline
/// 4: steady underline
/// 5: blinking bar
/// 6: steady bar
pub fn decscusr(w: &mut Write, n: u32) {
    if n <= 6 {
        w.write(format!(csi!("{} q"), n).as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    fn setup(w: &mut Write) {
        scp(w);
    }

    fn teardown(w: &mut Write) {
        rcp(w);
    }

    #[test]
    fn test_cup() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        // terminal cordinate start from (1,1)
        cup(w, 0, 0);
        assert_eq!(dsr(w), Some((1, 1)));
        // cup(1, 1) -> (1, 1)
        cup(w, 1, 1);
        assert_eq!(dsr(w), Some((1, 1)));
        // cup(3, 5) -> (3, 5)
        cup(w, 3, 5);
        assert_eq!(dsr(w), Some((3, 5)));
        teardown(w);
    }

    #[test]
    fn test_cuu() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 5, 3);
        cuu(w, 1);
        assert_eq!(dsr(w), Some((4, 3)));
        cuu(w, 2);
        assert_eq!(dsr(w), Some((2, 3)));
        teardown(w);
    }

    #[test]
    fn test_cud() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 1, 5);
        cud(w, 1);
        assert_eq!(dsr(w), Some((2, 5)));
        cud(w, 2);
        assert_eq!(dsr(w), Some((4, 5)));
        teardown(w);
    }

    #[test]
    fn test_cuf() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 3, 1);
        cuf(w, 1);
        assert_eq!(dsr(w), Some((3, 2)));
        cuf(w, 2);
        assert_eq!(dsr(w), Some((3, 4)));
        teardown(w);
    }

    #[test]
    fn test_cub() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 3, 5);
        cub(w, 1);
        assert_eq!(dsr(w), Some((3, 4)));
        cub(w, 2);
        assert_eq!(dsr(w), Some((3, 2)));
        teardown(w);
    }

    #[test]
    fn test_cnl() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 3, 5);
        cnl(w, 1);
        assert_eq!(dsr(w), Some((4, 1)));
        cup(w, 3, 5);
        cnl(w, 2);
        assert_eq!(dsr(w), Some((5, 1)));
        teardown(w);
    }

    #[test]
    fn test_cpl() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 3, 5);
        cpl(w, 1);
        assert_eq!(dsr(w), Some((2, 1)));
        cup(w, 3, 5);
        cpl(w, 2);
        assert_eq!(dsr(w), Some((1, 1)));
        teardown(w);
    }

    #[test]
    fn test_cha() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 3, 5);
        cha(w, 1);
        assert_eq!(dsr(w), Some((3, 1)));
        cha(w, 7);
        assert_eq!(dsr(w), Some((3, 7)));
        teardown(w);
    }

    #[test]
    fn test_su() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 3, 5);
        su(w, 1);
        assert_eq!(dsr(w), Some((3, 5)));
        teardown(w);
    }

    #[test]
    fn test_sd() {
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        cup(w, 3, 5);
        sd(w, 1);
        assert_eq!(dsr(w), Some((3, 5)));
        teardown(w);
    }

    #[test]
    fn test_sgr() {
        let mut w = io::stdout();
        let w = &mut w;
        // setup(w);

        sgr(w, SgrCode::Normal); // reset
        w.write("0");

        sgr(w, SgrCode::Bold); // bold on
        w.write("1");
        sgr(w, SgrCode::BoldFaintOff); // bold off

        sgr(w, SgrCode::Faint); // faint on
        w.write("2");
        sgr(w, SgrCode::BoldFaintOff); // faint off

        sgr(w, SgrCode::Italic); // italic on
        w.write("3");
        sgr(w, SgrCode::ItalicOff); // italic off

        sgr(w, SgrCode::Underline); // underline on
        w.write("4");
        sgr(w, SgrCode::UnderlineOff); // underline off

        sgr(w, SgrCode::SlowBlink); // blink on
        w.write("5");
        sgr(w, SgrCode::Steady); // blink off

        sgr(w, SgrCode::RapidBlink); // blink on
        w.write("6");
        sgr(w, SgrCode::Steady); // blink off

        sgr(w, SgrCode::Inverse); // inverse on
        w.write("7");
        sgr(w, SgrCode::Positive); // inverse off

        sgr(w, SgrCode::Strikethrough); // strikethrough on
        w.write("9");
        sgr(w, SgrCode::StrikethroughOff); // strikethrough off

        sgr(w, SgrCode::FgColorBlack); // fg: black
        sgr(w, SgrCode::BgColorRed); // bg: red
        w.write("30");
        sgr(w, SgrCode::BgColorDefault); // bg: default
        sgr(w, SgrCode::FgColorRed); // fg: red
        w.write("31");
        sgr(w, SgrCode::FgColorGreen); // fg: green
        w.write("32");
        sgr(w, SgrCode::FgColorYellow); // fg: yellow
        w.write("33");
        sgr(w, SgrCode::FgColorBlue); // fg: blue
        w.write("34");
        sgr(w, SgrCode::FgColorMagenta); // fg: magenta
        w.write("35");
        sgr(w, SgrCode::FgColorCyan); // fg: cyan
        w.write("36");
        sgr(w, SgrCode::FgColorWhite); // fg: white
        w.write("37");
        sgr(w, SgrCode::FgColorDefault); // fg: default

        sgr(w, SgrCode::BgColorBlack); // bg: black
        w.write("40");
        sgr(w, SgrCode::BgColorRed); // bg: red
        w.write("41");
        sgr(w, SgrCode::BgColorGreen); // bg: green
        w.write("42");
        sgr(w, SgrCode::BgColorYellow); // bg: yellow
        w.write("43");
        sgr(w, SgrCode::BgColorBlue); // bg: blue
        w.write("44");
        sgr(w, SgrCode::BgColorMagenta); // bg: magenta
        w.write("45");
        sgr(w, SgrCode::BgColorCyan); // bg: cyan
        w.write("46");
        sgr(w, SgrCode::FgColorRed); // fg: red
        sgr(w, SgrCode::BgColorWhite); // bg: white
        w.write("47");
        sgr(w, SgrCode::FgColorWhite); // fg: reset
        sgr(w, SgrCode::BgColorWhite); // bg: default

        sgr(w, SgrCode::Bold); // bold on
        sgr(w, SgrCode::Underline); // underline on
        sgr(w, SgrCode::SlowBlink); // blink on
        sgr(w, SgrCode::Normal); // reset
        w.write("x");

        writeln!(w);
        _flush(w);

        // teardown(w);
    }

    #[test]
    fn test_decscusr() {
        use std::{thread, time};
        let mut w = io::stdout();
        let w = &mut w;
        setup(w);
        decscusr(w, 1);
        _flush(w);
        thread::sleep(time::Duration::from_millis(1000));
        decscusr(w, 2);
        _flush(w);
        thread::sleep(time::Duration::from_millis(1000));
        decscusr(w, 3);
        _flush(w);
        thread::sleep(time::Duration::from_millis(1000));
        decscusr(w, 4);
        _flush(w);
        thread::sleep(time::Duration::from_millis(1000));
        decscusr(w, 5);
        _flush(w);
        thread::sleep(time::Duration::from_millis(1000));
        decscusr(w, 6);
        _flush(w);
        thread::sleep(time::Duration::from_millis(1000));
        decscusr(w, 2);
        _flush(w);
        teardown(w);
    }
}
