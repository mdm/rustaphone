use nom::{
    branch::alt,
    bytes::tag,
    character::complete::{char, digit1, one_of, space0, space1},
    combinator::{opt, verify},
    IResult, Parser,
};

use super::{Fx, FxCommand, Note};

fn dec_frac(input: &str) -> IResult<&str, f32> {
    let (input, _) = char('.').parse(input)?;
    let (input, fractional_part) = digit1(input)?;

    let result =
        fractional_part.parse::<f32>().unwrap() * 0.1_f32.powi(fractional_part.len() as i32);

    Ok((input, result))
}

fn dec(input: &str) -> IResult<&str, f32> {
    let (input, integer_part) = digit1(input)?;
    let (input, fractional_part) = opt(dec_frac).parse(input)?;

    let result = integer_part.parse::<f32>().unwrap() + fractional_part.unwrap_or(0.0);

    Ok((input, result))
}

fn float_neg(input: &str) -> IResult<&str, f32> {
    let (input, _) = char('-').parse(input)?;
    let (input, dec) = dec.parse(input)?;

    Ok((input, -dec))
}

fn float(input: &str) -> IResult<&str, f32> {
    alt((float_neg, dec)).parse(input)
}

fn fxcmd_volume(input: &str) -> IResult<&str, FxCommand> {
    tag("volume").map(|_| FxCommand::Volume).parse(input)
}

fn fxcmd_punch(input: &str) -> IResult<&str, FxCommand> {
    tag("punch").map(|_| FxCommand::Punch).parse(input)
}

fn fxcmd_attack(input: &str) -> IResult<&str, FxCommand> {
    tag("attack").map(|_| FxCommand::Attack).parse(input)
}

fn fxcmd_sustain(input: &str) -> IResult<&str, FxCommand> {
    tag("sustain").map(|_| FxCommand::Sustain).parse(input)
}

fn fxcmd_decay(input: &str) -> IResult<&str, FxCommand> {
    tag("decay").map(|_| FxCommand::Decay).parse(input)
}

fn fxcmd_square(input: &str) -> IResult<&str, FxCommand> {
    tag("square").map(|_| FxCommand::Square).parse(input)
}

fn fxcmd_sweep(input: &str) -> IResult<&str, FxCommand> {
    tag("sweep").map(|_| FxCommand::Sweep).parse(input)
}

fn fxcmd_vibe(input: &str) -> IResult<&str, FxCommand> {
    tag("vibe").map(|_| FxCommand::Vibe).parse(input)
}

fn fxcmd_vspeed(input: &str) -> IResult<&str, FxCommand> {
    tag("vspeed").map(|_| FxCommand::VSpeed).parse(input)
}

fn fxcmd_vdelay(input: &str) -> IResult<&str, FxCommand> {
    tag("vdelay").map(|_| FxCommand::VDelay).parse(input)
}

fn fxcmd_lpf(input: &str) -> IResult<&str, FxCommand> {
    tag("lpf").map(|_| FxCommand::Lpf).parse(input)
}

fn fxcmd_lsweep(input: &str) -> IResult<&str, FxCommand> {
    tag("lsweep").map(|_| FxCommand::LSweep).parse(input)
}

fn fxcmd_resonance(input: &str) -> IResult<&str, FxCommand> {
    tag("resonance").map(|_| FxCommand::Resonance).parse(input)
}

fn fxcmd_hpf(input: &str) -> IResult<&str, FxCommand> {
    tag("hpf").map(|_| FxCommand::Hpf).parse(input)
}

fn fxcmd_hsweep(input: &str) -> IResult<&str, FxCommand> {
    tag("hsweep").map(|_| FxCommand::HSweep).parse(input)
}

fn fxcmd_arp(input: &str) -> IResult<&str, FxCommand> {
    tag("arp").map(|_| FxCommand::Arp).parse(input)
}

fn fxcmd_aspeed(input: &str) -> IResult<&str, FxCommand> {
    tag("aspeed").map(|_| FxCommand::ASpeed).parse(input)
}

fn fxcmd_phase(input: &str) -> IResult<&str, FxCommand> {
    tag("phase").map(|_| FxCommand::Phase).parse(input)
}

fn fxcmd_psweep(input: &str) -> IResult<&str, FxCommand> {
    tag("psweep").map(|_| FxCommand::PSweep).parse(input)
}

fn fxcmd_repeat(input: &str) -> IResult<&str, FxCommand> {
    tag("repeat").map(|_| FxCommand::Repeat).parse(input)
}

fn fxcmd(input: &str) -> IResult<&str, FxCommand> {
    alt((
        fxcmd_volume,
        fxcmd_punch,
        fxcmd_attack,
        fxcmd_sustain,
        fxcmd_decay,
        fxcmd_square,
        fxcmd_sweep,
        fxcmd_vibe,
        fxcmd_vspeed,
        fxcmd_vdelay,
        fxcmd_lpf,
        fxcmd_lsweep,
        fxcmd_resonance,
        fxcmd_hpf,
        fxcmd_hsweep,
        fxcmd_arp,
        fxcmd_aspeed,
        fxcmd_phase,
        fxcmd_psweep,
        fxcmd_repeat,
    ))
    .parse(input)
}

fn len(input: &str) -> IResult<&str, u8> {
    let (input, len) =
        verify(digit1, |len: &str| len.chars().next().unwrap() != '0').parse(input)?;
    let (input, _) = opt(char(':')).parse(input)?;

    let len = len.parse::<u8>().unwrap();

    Ok((input, len))
}

fn up(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('+').parse(input)?;
    let (input, len) = opt(len).parse(input)?;

    Ok((input, len.unwrap_or(1)))
}

fn down(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('-').parse(input)?;
    let (input, len) = opt(len).parse(input)?;

    Ok((input, len.unwrap_or(1)))
}

fn modifier(input: &str) -> IResult<&str, char> {
    alt((char('b'), char('#'))).parse(input)
}

fn oct(input: &str) -> IResult<&str, u8> {
    let (input, oct) = one_of("12345678").parse(input)?;

    let oct = oct.to_digit(10).unwrap() as u8;

    Ok((input, oct))
}

fn fxmod(input: &str) -> IResult<&str, char> {
    let (input, fxmod) = alt((char('+'), char('-'))).parse(input)?;
    let (input, _) = alt((tag(":"), space1)).parse(input)?;

    Ok((input, fxmod))
}

fn fx(input: &str) -> IResult<&str, Fx> {
    let (input, _) = char('[').parse(input)?;
    let (input, fxcmd) = fxcmd.parse(input)?;
    let (input, _) = alt((tag(":"), space0)).parse(input)?;
    let (input, fxmod) = opt(fxmod).parse(input)?;
    let (input, float) = float.parse(input)?;
    let (input, _) = char(']').parse(input)?;

    let fx = Fx {
        command: fxcmd,
        val: float,
        r#mod: fxmod.unwrap_or('\0'),
    };

    Ok((input, fx))
}

struct RawNote {
    len: Option<u8>,
    tone: char,
    modifier: Option<char>,
    octave: Option<u8>,
    fx: Vec<Fx>,
}

fn note(input: &str) -> IResult<&str, RawNote> {
    let (input, len) = opt(len).parse(input)?;
    let (input, tone) = one_of("abcdefgABCDEFG").parse(input)?;
    let (input, modifier) = opt(modifier).parse(input)?;
    let (input, oct) = opt(oct).parse(input)?;

    let mut effects = Vec::new();
    let (mut input, mut effect) = opt(fx).parse(input)?;
    while effect.is_some() {
        effects.push(effect.unwrap());
        (input, effect) = opt(fx).parse(input)?;
    }

    let note = RawNote {
        len,
        tone,
        modifier,
        octave: oct,
        fx: effects,
    };

    Ok((input, note))
}

fn track(input: &str) -> IResult<&str, Vec<Note>> {
    let mut current_oct = 4;
    let mut current_len = 4;
    let mut notes = Vec::new();
    let mut rest = input;
    loop {
        let (input, len) = opt(len).parse(rest)?;
        if let Some(len) = len {
            current_len = len;
            notes.push(Note {
                tone: '\0',
                octave: current_oct,
                duration: current_len,
                fx: Vec::new(),
            });
            rest = input;
            continue;
        }
        let (input, raw_note) = opt(note).parse(rest)?;
        if let Some(raw_note) = raw_note {
            current_len = raw_note.len.unwrap_or(current_len);
            current_oct = raw_note.octave.unwrap_or(current_oct);
            let tone = match raw_note.tone {
                'a' | 'A' => match raw_note.modifier {
                    Some('b') => 'a',
                    Some('#') => 'b',
                    _ => 'A',
                },
                'b' | 'B' => match raw_note.modifier {
                    Some('b') => 'b',
                    Some('#') => 'C',
                    _ => 'B',
                },
                'c' | 'C' => match raw_note.modifier {
                    Some('b') => 'B',
                    Some('#') => 'd',
                    _ => 'C',
                },
                'd' | 'D' => match raw_note.modifier {
                    Some('b') => 'd',
                    Some('#') => 'e',
                    _ => 'D',
                },
                'e' | 'E' => match raw_note.modifier {
                    Some('b') => 'e',
                    Some('#') => 'f',
                    _ => 'E',
                },
                'f' | 'F' => match raw_note.modifier {
                    Some('b') => 'E',
                    Some('#') => 'g',
                    _ => 'F',
                },
                'g' | 'G' => match raw_note.modifier {
                    Some('b') => 'g',
                    Some('#') => 'a',
                    _ => 'G',
                },
                _ => '\0',
            };
            notes.push(Note {
                tone,
                octave: current_oct,
                duration: current_len,
                fx: raw_note.fx,
            });
            rest = input;
            continue;
        }
        let (input, len) = opt(up).parse(rest)?;
        if let Some(_len) = len {
            current_oct += 1;
            current_len = 4;
            rest = input;
            continue;
        }
        let (input, len) = opt(down).parse(rest)?;
        if let Some(_len) = len {
            current_oct -= 1;
            current_len = 4;
            rest = input;
            continue;
        }
        let (input, space) = opt(space1).parse(rest)?;
        if let Some(_) = space {
            rest = input;
            continue;
        }

        break; // if we reach this point, we're done
    }

    Ok((rest, notes))
}
