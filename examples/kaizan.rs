use std::io::{self, Read, Write};
use ansi_csi;
use ansi_csi::csi::{self, Csi};

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Normal,
    Insert,
    Quit,
}

fn op_normal(mut w: &mut io::Write, key: u8) -> io::Result<Mode> {
    let mut next_mode = Mode::Normal;
    match key {
        b'j' => w.cud(1)?,
        b'k' => w.cuu(1)?,
        b'l' => w.cuf(1)?,
        b'h' => w.cub(1)?,
        b'a' => w.cha(1)?,
        b'e' => w.cha(1000)?,
        b'n' => w.cnl(1)?,
        b'p' => w.cpl(1)?,

        b'K' => w.sd(1)?,
        b'J' => w.su(1)?,

        b'c' => w.ed(csi::EdClear::FromCurToEos)?,
        b'C' => w.ed(csi::EdClear::EntireScreen)?,
        b'd' => w.el(csi::ElClear::FromCurToEol)?,
        b'D' => w.el(csi::ElClear::EntireLine)?,

        b'i' => {
            w.decscusr(csi::DecscusrStyle::SteadyBar)?;
            next_mode = Mode::Insert;
        },
        b'q' => next_mode = Mode::Quit,

        b'v' => w.sgr(csi::SgrCode::Inverse)?,
        b'V' => w.sgr(csi::SgrCode::Normal)?,

        _ => {}
    }
    Ok(next_mode)
}

fn op_insert(mut w: &mut io::Write, key: u8) -> io::Result<Mode> {
    let mut next_mode = Mode::Insert;
    match key {
        27u8 => {  // ESC
            w.decscusr(csi::DecscusrStyle::SteadyBlock)?;
            next_mode = Mode::Normal;
        },
        _ => write!(w, "{}", key as char)?,
    }
    Ok(next_mode)
}

fn main() -> io::Result<()> {
    let r = io::stdin();
    let mut r = r.lock();
    let w = io::stdout();
    let mut w = w.lock();

    let mut mode = Mode::Normal;
    let termios = ansi_csi::echo_off();

    loop {
        let mut key = [0u8];

        match r.read(&mut key) {
            Ok(n) => if n == 0 {
                break;
            }
            Err(_) => break,
        }

        mode = match mode {
            Mode::Normal => op_normal(&mut w, key[0]).unwrap(),
            Mode::Insert => op_insert(&mut w, key[0]).unwrap(),
            _ => Mode::Quit,
        };
        w.flush()?;

        if mode == Mode::Quit {
            w.decscusr(csi::DecscusrStyle::SteadyBlock)?;
            w.cha(1)?;
            w.ed(csi::EdClear::FromCurToEos)?;
            break;
        }
    }

    ansi_csi::echo_on(&termios);
    Ok(())
}

