use std::io::{self, Read, Write};
use ansi_csi::{self, csi};

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Normal,
    Insert,
    Quit,
}

fn op_normal<W: io::Write>(mut w: &mut W, key: u8) -> io::Result<Mode> {
    let mut next_mode = Mode::Normal;
    match key {
        b'j' => csi::cud(&mut w, 1)?,
        b'k' => csi::cuu(&mut w, 1)?,
        b'l' => csi::cuf(&mut w, 1)?,
        b'h' => csi::cub(&mut w, 1)?,
        b'a' => csi::cha(&mut w, 1)?,
        b'e' => csi::cha(&mut w, 1000)?,
        b'n' => csi::cnl(&mut w, 1)?,
        b'p' => csi::cpl(&mut w, 1)?,

        b'K' => csi::sd(&mut w, 1)?,
        b'J' => csi::su(&mut w, 1)?,

        b'c' => csi::ed(&mut w, csi::EdClear::FromCurToEos)?,
        b'C' => csi::ed(&mut w, csi::EdClear::EntireScreen)?,
        b'd' => csi::el(&mut w, csi::ElClear::FromCurToEol)?,
        b'D' => csi::el(&mut w, csi::ElClear::EntireLine)?,

        b'i' => {
            csi::decscusr(&mut w, csi::DecscusrStyle::SteadyBar)?;
            next_mode = Mode::Insert;
        },
        b'q' => next_mode = Mode::Quit,

        b'v' => csi::sgr(&mut w, csi::SgrCode::Inverse)?,
        b'V' => csi::sgr(&mut w, csi::SgrCode::Normal)?,

        _ => {}
    }
    Ok(next_mode)
}

fn op_insert<W: io::Write>(mut w: &mut W, key: u8) -> io::Result<Mode> {
    let mut next_mode = Mode::Insert;
    match key {
        27u8 => {  // ESC
            csi::decscusr(&mut w, csi::DecscusrStyle::SteadyBlock)?;
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
            csi::decscusr(&mut w, csi::DecscusrStyle::SteadyBlock)?;
            csi::cha(&mut w, 1)?;
            csi::ed(&mut w, csi::EdClear::FromCurToEos)?;
            break;
        }
    }

    ansi_csi::echo_on(&termios);
    Ok(())
}

