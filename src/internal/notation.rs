use nom::{
    branch::alt,
    bytes::tag,
    character::complete::{char, digit1, one_of, space0, space1},
    combinator::{opt, verify},
    error::{Error, ErrorKind, ParseError},
    IResult, Parser,
};

use super::{Fx, FxCommand, Note};

#[derive(Clone)]
struct ParseState {}

#[derive(Clone)]
struct StatefulInput<'a> {
    input: &'a str,
    state: ParseState,
}

impl<'a> ParseError<StatefulInput<'a>> for Error<&'a str> {
    fn from_error_kind(input: StatefulInput<'a>, kind: ErrorKind) -> Self {
        Error::new(input.input, kind)
    }

    fn append(_input: StatefulInput<'a>, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

fn dec_frac(input: StatefulInput) -> IResult<StatefulInput, f32, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, _) = char('.').parse(input)?;
    let (input, fractional_part) = digit1(input)?;

    let result =
        fractional_part.parse::<f32>().unwrap() * 0.1_f32.powi(fractional_part.len() as i32);

    Ok((StatefulInput { input, state }, result))
}

fn dec(input: StatefulInput) -> IResult<StatefulInput, f32, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, integer_part) = digit1(input)?;
    let (input, fractional_part) = opt(dec_frac).parse(StatefulInput { input, state })?;

    let result = integer_part.parse::<f32>().unwrap() + fractional_part.unwrap_or(0.0);

    Ok((input, result))
}

fn float_neg(input: StatefulInput) -> IResult<StatefulInput, f32, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, _) = char('-').parse(input)?;
    let (input, dec) = dec.parse(StatefulInput { input, state })?;

    Ok((input, -dec))
}

fn float(input: StatefulInput) -> IResult<StatefulInput, f32, Error<&str>> {
    alt((float_neg, dec)).parse(input)
}

fn fxcmd_volume(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("volume").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_punch(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("punch").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_attack(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("attack").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_sustain(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("sustain").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_decay(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("decay").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_square(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("square").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_sweep(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("sweep").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_vibe(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("vibe").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_vspeed(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("vspeed").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_vdelay(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("vdelay").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_lpf(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("lpf").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_lsweep(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("lsweep").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_resonance(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("resonance").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_hpf(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("hpf").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_hsweep(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("hsweep").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_arp(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("arp").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_aspeed(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("aspeed").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_phase(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("phase").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_psweep(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("psweep").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd_repeat(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, result) = tag("repeat").map(|_| FxCommand::Volume).parse(input)?;

    Ok((StatefulInput { input, state }, result))
}

fn fxcmd(input: StatefulInput) -> IResult<StatefulInput, FxCommand, Error<&str>> {
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

fn len(input: StatefulInput) -> IResult<StatefulInput, u8, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, len) = verify(digit1, |len: &str| !len.starts_with('0')).parse(input)?;
    let (input, _) = opt(char(':')).parse(input)?;

    let len = len.parse::<u8>().unwrap();

    Ok((StatefulInput { input, state }, len))
}

fn up(input: StatefulInput) -> IResult<StatefulInput, u8, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, _) = char('+').parse(input)?;
    let (input, len) = opt(len).parse(StatefulInput { input, state })?;

    Ok((input, len.unwrap_or(1)))
}

fn down(input: StatefulInput) -> IResult<StatefulInput, u8, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, _) = char('-').parse(input)?;
    let (input, len) = opt(len).parse(StatefulInput { input, state })?;

    Ok((input, len.unwrap_or(1)))
}

fn modifier(input: StatefulInput) -> IResult<StatefulInput, char, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, modifier) = alt((char('b'), char('#'))).parse(input)?;

    Ok((StatefulInput { input, state }, modifier))
}

fn oct(input: StatefulInput) -> IResult<StatefulInput, u8, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, oct) = one_of("12345678").parse(input)?;

    let oct = oct.to_digit(10).unwrap() as u8;

    Ok((StatefulInput { input, state }, oct))
}

fn fxmod(input: StatefulInput) -> IResult<StatefulInput, char, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, fxmod) = alt((char('+'), char('-'))).parse(input)?;
    let (input, _) = alt((tag(":"), space1)).parse(input)?;

    Ok((StatefulInput { input, state }, fxmod))
}

fn fx(input: StatefulInput) -> IResult<StatefulInput, Fx, Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, _) = char('[').parse(input)?;
    let (input, fxcmd) = fxcmd.parse(StatefulInput { input, state })?;
    let StatefulInput { input, state } = input;
    let (input, _) = alt((tag(":"), space0)).parse(input)?;
    let (input, fxmod) = opt(fxmod).parse(StatefulInput { input, state })?;
    let (input, float) = float.parse(input)?;
    let StatefulInput { input, state } = input;
    let (input, _) = char(']').parse(input)?;

    let fx = Fx {
        command: fxcmd,
        val: float,
        r#mod: fxmod.unwrap_or('\0'),
    };

    Ok((StatefulInput { input, state }, fx))
}

struct RawNote {
    len: Option<u8>,
    tone: char,
    modifier: Option<char>,
    octave: Option<u8>,
    fx: Vec<Fx>,
}

fn note(input: StatefulInput) -> IResult<StatefulInput, RawNote, Error<&str>> {
    let (input, len) = opt(len).parse(input)?;
    let StatefulInput { input, state } = input;
    let (input, _) = char(']').parse(input)?;
    let (input, tone) = one_of("abcdefgABCDEFG").parse(input)?;
    let (input, modifier) = opt(modifier).parse(StatefulInput { input, state })?;
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

pub fn tune(input: &str) -> IResult<&str, Vec<Note>, Error<&str>> {
    let mut current_oct = 4;
    let mut current_len = 4;
    let mut notes = Vec::new();
    let mut rest = StatefulInput {
        input,
        state: ParseState {},
    };
    loop {
        let (input, len) = opt(len).parse(rest.clone())?;
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
        let (input, raw_note) = opt(note).parse(rest.clone())?;
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
        let (input, len) = opt(up).parse(rest.clone())?;
        if let Some(_len) = len {
            current_oct += 1;
            current_len = 4;
            rest = input;
            continue;
        }
        let (input, len) = opt(down).parse(rest.clone())?;
        if let Some(_len) = len {
            current_oct -= 1;
            current_len = 4;
            rest = input;
            continue;
        }
        let StatefulInput { input, state } = rest.clone();
        let (input, space) = opt(space1).parse(input)?;
        if space.is_some() {
            rest = StatefulInput { input, state };
            continue;
        }

        break; // if we reach this point, we're done
    }

    Ok((rest.input, notes))
}
