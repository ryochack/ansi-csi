use termios;

// CSI(Control Sequence Introducer) of Escapse sequence

use crate::{echo_off, echo_on};
use std::io::{self, Read};

pub type Result<T> = std::result::Result<T, std::io::Error>;

#[macro_export]
macro_rules! csi {
    ($( $s:expr ),*) => { concat!("\x1b[", $( $s ),*) };
}

pub enum EdClear {
    FromCurToEos = 0,
    FromCurToBos = 1,
    EntireScreen = 2,
    EntireScreenAndDeleteAllScrollBuffer = 3,
}

pub enum ElClear {
    FromCurToEol = 0,
    FromCurToBol = 1,
    EntireLine = 2,
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

pub enum SgrColor {
    FgColor8bit(u8),
    FgColor24bit((u8, u8, u8)),
    BgColor8bit(u8),
    BgColor24bit((u8, u8, u8)),
}

pub enum DecscusrStyle {
    BlinkingBlock = 1,
    SteadyBlock = 2,
    BlinkingUnderline = 3,
    SteadyUnderline = 4,
    BlinkingBar = 5,
    SteadyBar = 6,
}

// avoid zero
fn _nz(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n
    }
}

/// CUU: cursor up
pub fn cuu<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}A"), _nz(n)))?;
    Ok(())
}

/// CUD: cursor down
pub fn cud<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}B"), _nz(n)))?;
    Ok(())
}

/// CUF: cursor forward
pub fn cuf<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    //w.write(format!(csi!("{}C"), _nz(n)).as_bytes())?;
    w.write_fmt(format_args!(csi!("{}C"), _nz(n)))?;
    Ok(())
}

/// CUB: cursor back
pub fn cub<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}D"), _nz(n)))?;
    Ok(())
}

/// CNL: cursor next line
pub fn cnl<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}E"), _nz(n)))?;
    Ok(())
}

/// CPL: cursor previous line
pub fn cpl<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}F"), _nz(n)))?;
    Ok(())
}

/// CHA: cursor horizontal absolute
pub fn cha<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}G"), _nz(n)))?;
    Ok(())
}

/// CUP: cursor position
pub fn cup<W: io::Write>(w: &mut W, row: usize, col: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{};{}H"), _nz(row), _nz(col)))?;
    Ok(())
}

/// ED: erase in display
/// If n is 0 (or missing), clear from cursor to end of screen.
/// If n is 1, clear from cursor to beginning of the screen.
/// If n {\displaystyle n} n is 2, clear entire screen (and moves cursor to upper left on DOS ANSI.SYS).
/// If n {\displaystyle n} n is 3, clear entire screen and delete all lines saved in the scrollback buffer (this feature was added for xterm and is supported by other terminal applications).
pub fn ed<W: io::Write>(w: &mut W, n: EdClear) -> io::Result<()> {
    let n = n as usize;
    if n <= 3 {
        w.write_fmt(format_args!(csi!("{}J"), n))?;
    }
    Ok(())
}

/// EL: erase in line
/// If n is 0 (or missing), clear from cursor to the end of the line.
/// If n is 1, clear from cursor to beginning of the line.
/// If n is 2, clear entire line. Cursor position does not change.
pub fn el<W: io::Write>(w: &mut W, n: ElClear) -> io::Result<()> {
    let n = n as usize;
    if n <= 2 {
        w.write_fmt(format_args!(csi!("{}K"), n))?;
    }
    Ok(())
}

/// SU: scroll up
pub fn su<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}S"), _nz(n)))?;
    Ok(())
}

/// SD: scroll down
pub fn sd<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}T"), _nz(n)))?;
    Ok(())
}

/// SGR: select graphic rendition
/// SGR parameters: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
pub fn sgr<W: io::Write>(w: &mut W, c: SgrCode) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}m"), (c as i32)))?;
    Ok(())
}

pub fn sgr_color<W: io::Write>(w: &mut W, c: SgrColor) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}m"),
    match c {
        SgrColor::FgColor8bit(color) => format!("38;5;{}", color),
        SgrColor::FgColor24bit((r, g, b)) => format!("38;2;{};{};{}", r, g, b),
        SgrColor::BgColor8bit(color) => format!("48;5;{}", color),
        SgrColor::BgColor24bit((r, g, b)) => format!("48;2;{};{};{}", r, g, b),
    }
    ))?;
    Ok(())
}

/// DSR: device status report
/// return (row, col)
pub fn dsr<W: io::Write, R: io::Read>(w: &mut W, r: &mut R) -> Option<(usize, usize)> {
    let oldstat: Box<termios::Termios> = Box::new(echo_off());
    w.write_fmt(format_args!(csi!("6n"))).ok()?;
    w.flush().ok()?;
    let (mut row, mut col, mut tmp) = (0usize, 0usize, 0usize);
    // => "[${row};${col}R"
    for b in r.bytes().filter_map(|v| v.ok()) {
        match b {
            // '0' ... '9'
            0x30..=0x39 => {
                tmp = tmp * 10 + usize::from(b - 0x30);
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
pub fn scp<W: io::Write>(w: &mut W) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("s")))?;
    Ok(())
}

/// RCP: restore cursor position
pub fn rcp<W: io::Write>(w: &mut W) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("u")))?;
    Ok(())
}

/// SM: set mode
/// mode: http://ttssh2.osdn.jp/manual/ja/about/ctrlseq.html#mode
pub fn sm<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}h"), n))?;
    Ok(())
}

/// RM: reset mode
pub fn rm<W: io::Write>(w: &mut W, n: usize) -> io::Result<()> {
    w.write_fmt(format_args!(csi!("{}l"), n))?;
    Ok(())
}

/// DECSCUSR: set cursor style
/// 0,1: blinking block
/// 2: steady block
/// 3: blinking underline
/// 4: steady underline
/// 5: blinking bar
/// 6: steady bar
pub fn decscusr<W: io::Write>(w: &mut W, s: DecscusrStyle) -> io::Result<()> {
    let n = s as usize;
    if n <= 6 {
        w.write_fmt(format_args!(csi!("{} q"), n))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    fn setup<W: io::Write>(w: &mut W) {
        scp(w).unwrap();
    }

    fn teardown<W: io::Write>(w: &mut W) {
        rcp(w).unwrap();
    }

    #[test]
    fn test_cup() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        // terminal cordinate start from (1,1)
        cup(&mut w, 0, 0).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((1, 1)));
        // cup(1, 1) -> (1, 1)
        cup(&mut w, 1, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((1, 1)));
        // cup(3, 5) -> (3, 5)
        cup(&mut w, 3, 5).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_cuu() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 5, 3).unwrap();
        cuu(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((4, 3)));
        cuu(&mut w, 2).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((2, 3)));
        teardown(&mut w);
    }

    #[test]
    fn test_cud() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 1, 5).unwrap();
        cud(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((2, 5)));
        cud(&mut w, 2).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((4, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_cuf() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 3, 1).unwrap();
        cuf(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 2)));
        cuf(&mut w, 2).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 4)));
        teardown(&mut w);
    }

    #[test]
    fn test_cub() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 3, 5).unwrap();
        cub(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 4)));
        cub(&mut w, 2).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 2)));
        teardown(&mut w);
    }

    #[test]
    fn test_cnl() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 3, 5).unwrap();
        cnl(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((4, 1)));
        cup(&mut w, 3, 5).unwrap();
        cnl(&mut w, 2).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((5, 1)));
        teardown(&mut w);
    }

    #[test]
    fn test_cpl() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 3, 5).unwrap();
        cpl(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((2, 1)));
        cup(&mut w, 3, 5).unwrap();
        cpl(&mut w, 2).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((1, 1)));
        teardown(&mut w);
    }

    #[test]
    fn test_cha() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 3, 5).unwrap();
        cha(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 1)));
        cha(&mut w, 7).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 7)));
        teardown(&mut w);
    }

    #[test]
    fn test_su() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 3, 5).unwrap();
        su(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_sd() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        cup(&mut w, 3, 5).unwrap();
        sd(&mut w, 1).unwrap();
        assert_eq!(dsr(&mut w, &mut r), Some((3, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_sgr() {
        let w = io::stdout();
        let mut w = w.lock();
        // setup(&mut w);

        sgr(&mut w, SgrCode::Normal).unwrap(); // reset
        w.write(b"0").unwrap();

        sgr(&mut w, SgrCode::Bold).unwrap(); // bold on
        w.write(b"1").unwrap();
        sgr(&mut w, SgrCode::BoldFaintOff).unwrap(); // bold off

        sgr(&mut w, SgrCode::Faint).unwrap(); // faint on
        w.write(b"2").unwrap();
        sgr(&mut w, SgrCode::BoldFaintOff).unwrap(); // faint off

        sgr(&mut w, SgrCode::Italic).unwrap(); // italic on
        w.write(b"3").unwrap();
        sgr(&mut w, SgrCode::ItalicOff).unwrap(); // italic off

        sgr(&mut w, SgrCode::Underline).unwrap(); // underline on
        w.write(b"4").unwrap();
        sgr(&mut w, SgrCode::UnderlineOff).unwrap(); // underline off

        sgr(&mut w, SgrCode::SlowBlink).unwrap(); // blink on
        w.write(b"5").unwrap();
        sgr(&mut w, SgrCode::Steady).unwrap(); // blink off

        sgr(&mut w, SgrCode::RapidBlink).unwrap(); // blink on
        w.write(b"6").unwrap();
        sgr(&mut w, SgrCode::Steady).unwrap(); // blink off

        sgr(&mut w, SgrCode::Inverse).unwrap(); // inverse on
        w.write(b"7").unwrap();
        sgr(&mut w, SgrCode::Positive).unwrap(); // inverse off

        sgr(&mut w, SgrCode::Strikethrough).unwrap(); // strikethrough on
        w.write(b"9").unwrap();
        sgr(&mut w, SgrCode::StrikethroughOff).unwrap(); // strikethrough off

        sgr(&mut w, SgrCode::FgColorBlack).unwrap(); // fg: black
        sgr(&mut w, SgrCode::BgColorRed).unwrap(); // bg: red
        w.write(b"30").unwrap();
        sgr(&mut w, SgrCode::BgColorDefault).unwrap(); // bg: default
        sgr(&mut w, SgrCode::FgColorRed).unwrap(); // fg: red
        w.write(b"31").unwrap();
        sgr(&mut w, SgrCode::FgColorGreen).unwrap(); // fg: green
        w.write(b"32").unwrap();
        sgr(&mut w, SgrCode::FgColorYellow).unwrap(); // fg: yellow
        w.write(b"33").unwrap();
        sgr(&mut w, SgrCode::FgColorBlue).unwrap(); // fg: blue
        w.write(b"34").unwrap();
        sgr(&mut w, SgrCode::FgColorMagenta).unwrap(); // fg: magenta
        w.write(b"35").unwrap();
        sgr(&mut w, SgrCode::FgColorCyan).unwrap(); // fg: cyan
        w.write(b"36").unwrap();
        sgr(&mut w, SgrCode::FgColorWhite).unwrap(); // fg: white
        w.write(b"37").unwrap();
        sgr(&mut w, SgrCode::FgColorDefault).unwrap(); // fg: default

        sgr(&mut w, SgrCode::BgColorBlack).unwrap(); // bg: black
        w.write(b"40").unwrap();
        sgr(&mut w, SgrCode::BgColorRed).unwrap(); // bg: red
        w.write(b"41").unwrap();
        sgr(&mut w, SgrCode::BgColorGreen).unwrap(); // bg: green
        w.write(b"42").unwrap();
        sgr(&mut w, SgrCode::BgColorYellow).unwrap(); // bg: yellow
        w.write(b"43").unwrap();
        sgr(&mut w, SgrCode::BgColorBlue).unwrap(); // bg: blue
        w.write(b"44").unwrap();
        sgr(&mut w, SgrCode::BgColorMagenta).unwrap(); // bg: magenta
        w.write(b"45").unwrap();
        sgr(&mut w, SgrCode::BgColorCyan).unwrap(); // bg: cyan
        w.write(b"46").unwrap();
        sgr(&mut w, SgrCode::FgColorRed).unwrap(); // fg: red
        sgr(&mut w, SgrCode::BgColorWhite).unwrap(); // bg: white
        w.write(b"47").unwrap();
        sgr(&mut w, SgrCode::FgColorWhite).unwrap(); // fg: reset
        sgr(&mut w, SgrCode::BgColorWhite).unwrap(); // bg: default

        sgr(&mut w, SgrCode::Bold).unwrap(); // bold on
        sgr(&mut w, SgrCode::Underline).unwrap(); // underline on
        sgr(&mut w, SgrCode::SlowBlink).unwrap(); // blink on
        sgr(&mut w, SgrCode::Normal).unwrap(); // reset
        w.write(b"x").unwrap();

        writeln!(w).unwrap();
        w.flush().unwrap();

        // teardown(&mut w);
    }

    #[test]
    fn test_decscusr() {
        use std::{thread, time};
        let w = io::stdout();
        let mut w = w.lock();
        setup(&mut w);
        decscusr(&mut w, DecscusrStyle::BlinkingBlock).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        decscusr(&mut w, DecscusrStyle::SteadyBlock).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        decscusr(&mut w, DecscusrStyle::BlinkingUnderline).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        decscusr(&mut w, DecscusrStyle::SteadyUnderline).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        decscusr(&mut w, DecscusrStyle::BlinkingBar).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        decscusr(&mut w, DecscusrStyle::SteadyBar).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        decscusr(&mut w, DecscusrStyle::SteadyBlock).unwrap();
        w.flush().unwrap();
        teardown(&mut w);
    }
}
