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
struct ParseState {
    fxcmd: Option<FxCommand>,
    fxmod: Option<char>,
    fxval: f32,
    len: i32,
    oct: i32,
    modifier: Option<char>,
    fx: Vec<Fx>,
    tone: char,
    note: Option<Note>,
}

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

fn dec(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, integer_part) = digit1(input)?;
    state.fxval = integer_part.parse::<f32>().unwrap();
    let (mut input, fractional_part) = opt(dec_frac).parse(StatefulInput { input, state })?;

    input.state.fxval += fractional_part.unwrap_or(0.0);

    Ok((input, ()))
}

fn float(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, neg) = opt(char('-')).parse(input)?;
    let (mut input, _) = dec.parse(StatefulInput { input, state })?;
    if neg.is_some() {
        input.state.fxval *= -1.0;
    }

    Ok((input, ()))
}

fn fxcmd_volume(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("volume").parse(input)?;
    state.fxcmd = Some(FxCommand::Volume);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_punch(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("punch").parse(input)?;
    state.fxcmd = Some(FxCommand::Punch);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_attack(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("attack").parse(input)?;
    state.fxcmd = Some(FxCommand::Attack);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_sustain(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("sustain").parse(input)?;
    state.fxcmd = Some(FxCommand::Sustain);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_decay(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("decay").parse(input)?;
    state.fxcmd = Some(FxCommand::Decay);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_square(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("square").parse(input)?;
    state.fxcmd = Some(FxCommand::Square);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_sweep(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("sweep").parse(input)?;
    state.fxcmd = Some(FxCommand::Sweep);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_vibe(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("vibe").parse(input)?;
    state.fxcmd = Some(FxCommand::Vibe);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_vspeed(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("vspeed").parse(input)?;
    state.fxcmd = Some(FxCommand::VSpeed);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_vdelay(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("vdelay").parse(input)?;
    state.fxcmd = Some(FxCommand::VDelay);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_lpf(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("lpf").parse(input)?;
    state.fxcmd = Some(FxCommand::Lpf);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_lsweep(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("lsweep").parse(input)?;
    state.fxcmd = Some(FxCommand::LSweep);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_resonance(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("resonance").parse(input)?;
    state.fxcmd = Some(FxCommand::Resonance);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_hpf(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("hpf").parse(input)?;
    state.fxcmd = Some(FxCommand::Hpf);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_hsweep(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("hsweep").parse(input)?;
    state.fxcmd = Some(FxCommand::HSweep);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_arp(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("arp").parse(input)?;
    state.fxcmd = Some(FxCommand::Arp);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_aspeed(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("aspeed").parse(input)?;
    state.fxcmd = Some(FxCommand::ASpeed);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_phase(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("phase").parse(input)?;
    state.fxcmd = Some(FxCommand::Phase);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_psweep(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("psweep").parse(input)?;
    state.fxcmd = Some(FxCommand::PSweep);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd_repeat(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = tag("repeat").parse(input)?;
    state.fxcmd = Some(FxCommand::Repeat);

    Ok((StatefulInput { input, state }, ()))
}

fn fxcmd(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
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

fn len(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, len) = verify(digit1, |len: &str| !len.starts_with('0')).parse(input)?;
    let (input, _) = opt(char(':')).parse(input)?;

    state.len = len.parse::<i32>().unwrap();

    Ok((StatefulInput { input, state }, ()))
}

fn up(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = char('+').parse(input)?;
    state.len = 1;
    let (input, _) = opt(len).parse(StatefulInput { input, state })?;

    Ok((input, ()))
}

fn down(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, _) = char('-').parse(input)?;
    state.len = 1;
    let (input, _) = opt(len).parse(StatefulInput { input, state })?;

    Ok((input, ()))
}

fn modifier(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, modifier) = alt((char('b'), char('#'))).parse(input)?;
    state.modifier = Some(modifier);

    Ok((StatefulInput { input, state }, ()))
}

fn oct(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, oct) = one_of("12345678").parse(input)?;

    state.oct = oct.to_digit(10).unwrap() as i32;

    Ok((StatefulInput { input, state }, ()))
}

fn fxmod(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, mut state } = input;
    let (input, fxmod) = alt((char('+'), char('-'))).parse(input)?;
    state.fxmod = Some(fxmod);
    let (input, _) = alt((tag(":"), space1)).parse(input)?;

    Ok((StatefulInput { input, state }, ()))
}

fn fx(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let StatefulInput { input, state } = input;
    let (input, _) = char('[').parse(input)?;
    let (input, _) = fxcmd.parse(StatefulInput { input, state })?;
    let StatefulInput { input, state } = input;
    let (input, _) = alt((tag(":"), space0)).parse(input)?;
    let (input, _) = opt(fxmod).parse(StatefulInput { input, state })?;
    let (input, _) = float.parse(input)?;
    let StatefulInput { input, mut state } = input;
    let (input, _) = char(']').parse(input)?;

    let fx = Fx {
        command: state.fxcmd.unwrap(),
        val: state.fxval,
        r#mod: state.fxmod.unwrap_or('\0'),
    };
    state.fx.push(fx);
    state.fxcmd = None;
    state.fxval = 0.0;
    state.fxmod = None;

    Ok((StatefulInput { input, state }, ()))
}

fn note(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    let (input, _) = opt(len).parse(input)?;
    let StatefulInput { input, mut state } = input;
    let (input, tone) = one_of("abcdefgABCDEFG").parse(input)?;
    state.tone = tone;
    let (input, _) = opt(modifier).parse(StatefulInput { input, state })?;
    let (input, _) = opt(oct).parse(input)?;

    let (mut input, mut effect) = opt(fx).parse(input)?;
    while effect.is_some() {
        (input, effect) = opt(fx).parse(input)?;
    }

    Ok((input, ()))
}

fn tune_len(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    println!("tune_len");
    let (mut input, _) = len.parse(input)?;

    input.state.note = Some(Note {
        tone: '\0',
        octave: input.state.oct as u8,
        duration: input.state.len as u8,
        fx: input.state.fx.clone(),
    });
    input.state.modifier = None;
    input.state.tone = '\0';
    input.state.len = 4;
    input.state.fxmod = None;
    input.state.fxval = 0.0;

    Ok((input, ()))
}

fn tune_note(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    println!("tune_note");
    let (mut input, _) = note.parse(input)?;

    let tone = match input.state.tone {
        'a' | 'A' => match input.state.modifier {
            Some('b') => 'a',
            Some('#') => 'b',
            _ => 'A',
        },
        'b' | 'B' => match input.state.modifier {
            Some('b') => 'b',
            Some('#') => 'C',
            _ => 'B',
        },
        'c' | 'C' => match input.state.modifier {
            Some('b') => 'B',
            Some('#') => 'd',
            _ => 'C',
        },
        'd' | 'D' => match input.state.modifier {
            Some('b') => 'd',
            Some('#') => 'e',
            _ => 'D',
        },
        'e' | 'E' => match input.state.modifier {
            Some('b') => 'e',
            Some('#') => 'f',
            _ => 'E',
        },
        'f' | 'F' => match input.state.modifier {
            Some('b') => 'E',
            Some('#') => 'g',
            _ => 'F',
        },
        'g' | 'G' => match input.state.modifier {
            Some('b') => 'g',
            Some('#') => 'a',
            _ => 'G',
        },
        _ => '\0',
    };

    input.state.note = Some(Note {
        tone,
        octave: input.state.oct as u8,
        duration: input.state.len as u8,
        fx: input.state.fx.clone(),
    });
    input.state.modifier = None;
    input.state.tone = '\0';
    input.state.len = 4;
    input.state.fxmod = None;
    input.state.fxval = 0.0;
    input.state.fx = Vec::new();

    Ok((input, ()))
}

fn tune_up(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    println!("tune_up");
    let (mut input, _) = up.parse(input)?;

    input.state.oct += 1;
    input.state.len = 4;

    Ok((input, ()))
}

fn tune_down(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    println!("tune_down");
    let (mut input, _) = down.parse(input)?;

    input.state.oct += 1;
    input.state.len = 4;

    Ok((input, ()))
}

fn tune_space(input: StatefulInput) -> IResult<StatefulInput, (), Error<&str>> {
    println!("tune_space");
    let StatefulInput { input, state } = input;
    let (input, _) = space1.parse(input)?;

    Ok((StatefulInput { input, state }, ()))
}

pub fn tune(input: &str) -> IResult<&str, Vec<Note>, Error<&str>> {
    let state = ParseState {
        fxcmd: None,
        fxmod: None,
        fxval: 0.0,
        len: 4,
        oct: 4,
        modifier: None,
        fx: Vec::new(),
        tone: '\0',
        note: None,
    };
    let mut input = StatefulInput { input, state };
    let mut tune = Vec::new();

    while let Ok((mut rest, _)) =
        alt((tune_note, tune_len, tune_up, tune_down, tune_space)).parse(input.clone())
    {
        println!("rest: {:?}", rest.input);
        if let Some(note) = rest.state.note.take() {
            dbg!(&note);
            tune.push(note);
        }

        input = rest;
    }

    Ok((input.input, tune))
}
