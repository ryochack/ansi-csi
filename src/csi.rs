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

pub trait Csi {
    /// CUU: cursor up
    fn cuu(&mut self, n: usize) -> io::Result<()>;
    /// CUD: cursor down
    fn cud(&mut self, n: usize) -> io::Result<()>;
    /// CUF: cursor forward
    fn cuf(&mut self, n: usize) -> io::Result<()>;
    /// CUB: cursor back
    fn cub(&mut self, n: usize) -> io::Result<()>;
    /// CNL: cursor next line
    fn cnl(&mut self, n: usize) -> io::Result<()>;
    /// CPL: cursor previous line
    fn cpl(&mut self, n: usize) -> io::Result<()>;
    /// CHA: cursor horizontal absolute
    fn cha(&mut self, n: usize) -> io::Result<()>;
    /// CUP: cursor position
    fn cup(&mut self, row: usize, col: usize) -> io::Result<()>;
    /// ED: erase in display
    /// If n is 0 (or missing), clear from cursor to end of screen.
    /// If n is 1, clear from cursor to beginning of the screen.
    /// If n {\displaystyle n} n is 2, clear entire screen (and moves cursor to upper left on DOS ANSI.SYS).
    /// If n {\displaystyle n} n is 3, clear entire screen and delete all lines saved in the scrollback buffer (this feature was added for xterm and is supported by other terminal applications).
    fn ed(&mut self, n: EdClear) -> io::Result<()>;
    /// EL: erase in line
    /// If n is 0 (or missing), clear from cursor to the end of the line.
    /// If n is 1, clear from cursor to beginning of the line.
    /// If n is 2, clear entire line. Cursor position does not change.
    fn el(&mut self, n: ElClear) -> io::Result<()>;
    /// SU: scroll up
    fn su(&mut self, n: usize) -> io::Result<()>;
    /// SD: scroll down
    fn sd(&mut self, n: usize) -> io::Result<()>;
    /// SGR: select graphic rendition
    /// SGR parameters: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
    fn sgr(&mut self, c: SgrCode) -> io::Result<()>;
    fn sgr_color(&mut self, c: SgrColor) -> io::Result<()>;
    /// DSR: device status report
    /// return (row, col)
    fn dsr(&mut self, r: &mut io::Read) -> Option<(usize, usize)>;
    /// SCP: save cursor position
    fn scp(&mut self) -> io::Result<()>;
    /// RCP: restore cursor position
    fn rcp(&mut self) -> io::Result<()>;
    /// SM: set mode
    /// mode: http://ttssh2.osdn.jp/manual/ja/about/ctrlseq.html#mode
    fn sm(&mut self, n: usize) -> io::Result<()>;
    /// RM: reset mode
    fn rm(&mut self, n: usize) -> io::Result<()>;
    /// DECSCUSR: set cursor style
    /// 0,1: blinking block
    /// 2: steady block
    /// 3: blinking underline
    /// 4: steady underline
    /// 5: blinking bar
    /// 6: steady bar
    fn decscusr(&mut self, s: DecscusrStyle) -> io::Result<()>;
}

// avoid zero
fn _nz(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n
    }
}

impl<W: io::Write> Csi for W {
    fn cuu(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}A"), _nz(n)))?;
        Ok(())
    }
    fn cud(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}B"), _nz(n)))?;
        Ok(())
    }
    fn cuf(&mut self, n: usize) -> io::Result<()> {
        //self.write(format!(csi!("{}C"), _nz(n)).as_bytes())?;
        self.write_fmt(format_args!(csi!("{}C"), _nz(n)))?;
        Ok(())
    }
    fn cub(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}D"), _nz(n)))?;
        Ok(())
    }
    fn cnl(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}E"), _nz(n)))?;
        Ok(())
    }
    fn cpl(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}F"), _nz(n)))?;
        Ok(())
    }
    fn cha(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}G"), _nz(n)))?;
        Ok(())
    }
    fn cup(&mut self, row: usize, col: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{};{}H"), _nz(row), _nz(col)))?;
        Ok(())
    }
    fn ed(&mut self, n: EdClear) -> io::Result<()> {
        let n = n as usize;
        if n <= 3 {
            self.write_fmt(format_args!(csi!("{}J"), n))?;
        }
        Ok(())
    }
    fn el(&mut self, n: ElClear) -> io::Result<()> {
        let n = n as usize;
        if n <= 2 {
            self.write_fmt(format_args!(csi!("{}K"), n))?;
        }
        Ok(())
    }
    fn su(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}S"), _nz(n)))?;
        Ok(())
    }
    fn sd(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}T"), _nz(n)))?;
        Ok(())
    }
    fn sgr(&mut self, c: SgrCode) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}m"), (c as i32)))?;
        Ok(())
    }
    fn sgr_color(&mut self, c: SgrColor) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}m"),
        match c {
            SgrColor::FgColor8bit(color) => format!("38;5;{}", color),
            SgrColor::FgColor24bit((r, g, b)) => format!("38;2;{};{};{}", r, g, b),
            SgrColor::BgColor8bit(color) => format!("48;5;{}", color),
            SgrColor::BgColor24bit((r, g, b)) => format!("48;2;{};{};{}", r, g, b),
        }
        ))?;
        Ok(())
    }
    fn dsr(&mut self, r: &mut io::Read) -> Option<(usize, usize)> {
        let oldstat: Box<termios::Termios> = Box::new(echo_off());
        self.write_fmt(format_args!(csi!("6n"))).ok()?;
        self.flush().ok()?;
        let (mut row, mut col, mut tmp) = (0usize, 0usize, 0usize);
        // => "[${row};${col}R"
        for b in r.bytes().filter_map(|v| v.ok()) {
            match b {
                // '0' ... '9'
                0x30...0x39 => {
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
    fn scp(&mut self) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("s")))?;
        Ok(())
    }
    fn rcp(&mut self) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("u")))?;
        Ok(())
    }
    fn sm(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}h"), n))?;
        Ok(())
    }
    fn rm(&mut self, n: usize) -> io::Result<()> {
        self.write_fmt(format_args!(csi!("{}l"), n))?;
        Ok(())
    }
    fn decscusr(&mut self, s: DecscusrStyle) -> io::Result<()> {
        let n = s as usize;
        if n <= 6 {
            self.write_fmt(format_args!(csi!("{} q"), n))?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    fn setup<W: io::Write>(w: &mut W) {
        w.scp().unwrap();
    }

    fn teardown<W: io::Write>(w: &mut W) {
        w.rcp().unwrap();
    }

    #[test]
    fn test_cup() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        // terminal cordinate start from (1,1)
        w.cup(0, 0).unwrap();
        assert_eq!(w.dsr(&mut r), Some((1, 1)));
        // cup(1, 1) -> (1, 1)
        w.cup(1, 1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((1, 1)));
        // cup(3, 5) -> (3, 5)
        w.cup(3, 5).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_cuu() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(5, 3).unwrap();
        w.cuu(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((4, 3)));
        w.cuu(2).unwrap();
        assert_eq!(w.dsr(&mut r), Some((2, 3)));
        teardown(&mut w);
    }

    #[test]
    fn test_cud() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(1, 5).unwrap();
        w.cud(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((2, 5)));
        w.cud(2).unwrap();
        assert_eq!(w.dsr(&mut r), Some((4, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_cuf() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(3, 1).unwrap();
        w.cuf(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 2)));
        w.cuf(2).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 4)));
        teardown(&mut w);
    }

    #[test]
    fn test_cub() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(3, 5).unwrap();
        w.cub(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 4)));
        w.cub(2).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 2)));
        teardown(&mut w);
    }

    #[test]
    fn test_cnl() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(3, 5).unwrap();
        w.cnl(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((4, 1)));
        w.cup(3, 5).unwrap();
        w.cnl(2).unwrap();
        assert_eq!(w.dsr(&mut r), Some((5, 1)));
        teardown(&mut w);
    }

    #[test]
    fn test_cpl() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(3, 5).unwrap();
        w.cpl(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((2, 1)));
        w.cup(3, 5).unwrap();
        w.cpl(2).unwrap();
        assert_eq!(w.dsr(&mut r), Some((1, 1)));
        teardown(&mut w);
    }

    #[test]
    fn test_cha() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(3, 5).unwrap();
        w.cha(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 1)));
        w.cha(7).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 7)));
        teardown(&mut w);
    }

    #[test]
    fn test_su() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(3, 5).unwrap();
        w.su(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_sd() {
        let w = io::stdout();
        let mut w = w.lock();
        let r = io::stdin();
        let mut r = r.lock();
        setup(&mut w);
        w.cup(3, 5).unwrap();
        w.sd(1).unwrap();
        assert_eq!(w.dsr(&mut r), Some((3, 5)));
        teardown(&mut w);
    }

    #[test]
    fn test_sgr() {
        let w = io::stdout();
        let mut w = w.lock();
        // setup(&mut w);

        w.sgr(SgrCode::Normal).unwrap(); // reset
        w.write(b"0").unwrap();

        w.sgr(SgrCode::Bold).unwrap(); // bold on
        w.write(b"1").unwrap();
        w.sgr(SgrCode::BoldFaintOff).unwrap(); // bold off

        w.sgr(SgrCode::Faint).unwrap(); // faint on
        w.write(b"2").unwrap();
        w.sgr(SgrCode::BoldFaintOff).unwrap(); // faint off

        w.sgr(SgrCode::Italic).unwrap(); // italic on
        w.write(b"3").unwrap();
        w.sgr(SgrCode::ItalicOff).unwrap(); // italic off

        w.sgr(SgrCode::Underline).unwrap(); // underline on
        w.write(b"4").unwrap();
        w.sgr(SgrCode::UnderlineOff).unwrap(); // underline off

        w.sgr(SgrCode::SlowBlink).unwrap(); // blink on
        w.write(b"5").unwrap();
        w.sgr(SgrCode::Steady).unwrap(); // blink off

        w.sgr(SgrCode::RapidBlink).unwrap(); // blink on
        w.write(b"6").unwrap();
        w.sgr(SgrCode::Steady).unwrap(); // blink off

        w.sgr(SgrCode::Inverse).unwrap(); // inverse on
        w.write(b"7").unwrap();
        w.sgr(SgrCode::Positive).unwrap(); // inverse off

        w.sgr(SgrCode::Strikethrough).unwrap(); // strikethrough on
        w.write(b"9").unwrap();
        w.sgr(SgrCode::StrikethroughOff).unwrap(); // strikethrough off

        w.sgr(SgrCode::FgColorBlack).unwrap(); // fg: black
        w.sgr(SgrCode::BgColorRed).unwrap(); // bg: red
        w.write(b"30").unwrap();
        w.sgr(SgrCode::BgColorDefault).unwrap(); // bg: default
        w.sgr(SgrCode::FgColorRed).unwrap(); // fg: red
        w.write(b"31").unwrap();
        w.sgr(SgrCode::FgColorGreen).unwrap(); // fg: green
        w.write(b"32").unwrap();
        w.sgr(SgrCode::FgColorYellow).unwrap(); // fg: yellow
        w.write(b"33").unwrap();
        w.sgr(SgrCode::FgColorBlue).unwrap(); // fg: blue
        w.write(b"34").unwrap();
        w.sgr(SgrCode::FgColorMagenta).unwrap(); // fg: magenta
        w.write(b"35").unwrap();
        w.sgr(SgrCode::FgColorCyan).unwrap(); // fg: cyan
        w.write(b"36").unwrap();
        w.sgr(SgrCode::FgColorWhite).unwrap(); // fg: white
        w.write(b"37").unwrap();
        w.sgr(SgrCode::FgColorDefault).unwrap(); // fg: default

        w.sgr(SgrCode::BgColorBlack).unwrap(); // bg: black
        w.write(b"40").unwrap();
        w.sgr(SgrCode::BgColorRed).unwrap(); // bg: red
        w.write(b"41").unwrap();
        w.sgr(SgrCode::BgColorGreen).unwrap(); // bg: green
        w.write(b"42").unwrap();
        w.sgr(SgrCode::BgColorYellow).unwrap(); // bg: yellow
        w.write(b"43").unwrap();
        w.sgr(SgrCode::BgColorBlue).unwrap(); // bg: blue
        w.write(b"44").unwrap();
        w.sgr(SgrCode::BgColorMagenta).unwrap(); // bg: magenta
        w.write(b"45").unwrap();
        w.sgr(SgrCode::BgColorCyan).unwrap(); // bg: cyan
        w.write(b"46").unwrap();
        w.sgr(SgrCode::FgColorRed).unwrap(); // fg: red
        w.sgr(SgrCode::BgColorWhite).unwrap(); // bg: white
        w.write(b"47").unwrap();
        w.sgr(SgrCode::FgColorWhite).unwrap(); // fg: reset
        w.sgr(SgrCode::BgColorWhite).unwrap(); // bg: default

        w.sgr(SgrCode::Bold).unwrap(); // bold on
        w.sgr(SgrCode::Underline).unwrap(); // underline on
        w.sgr(SgrCode::SlowBlink).unwrap(); // blink on
        w.sgr(SgrCode::Normal).unwrap(); // reset
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
        w.decscusr(DecscusrStyle::BlinkingBlock).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        w.decscusr(DecscusrStyle::SteadyBlock).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        w.decscusr(DecscusrStyle::BlinkingUnderline).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        w.decscusr(DecscusrStyle::SteadyUnderline).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        w.decscusr(DecscusrStyle::BlinkingBar).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        w.decscusr(DecscusrStyle::SteadyBar).unwrap();
        w.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        w.decscusr(DecscusrStyle::SteadyBlock).unwrap();
        w.flush().unwrap();
        teardown(&mut w);
    }
}
