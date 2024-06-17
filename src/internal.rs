use std::{cell::RefCell, rc::Rc};

const HI_OCTAVE: u8 = 8;
const MAX_TRACKS: usize = 64;
const MAX_CHANNELS: usize = 64;

const UNUSED_VOICE: Option<Voice> = None;
const UNUSED_CHANNEL: Option<Rc<RefCell<Rustaphone>>> = None;

pub(super) enum State {
    Stop,
    Play,
}

pub(super) enum Waveform {
    Square,
    Sawtooth,
    Sine,
    Noise,
}

pub(super) enum FxCommand {
    Volume,
    Punch,
    Attack,
    Sustain,
    Decay,
    Sweep,
    Square,
    Vibe,
    VSpeed,
    VDelay,
    Lpf,
    LSweep,
    Resonance,
    Hpf,
    HSweep,
    Arp,
    ASpeed,
    Phase,
    PSweep,
    Repeat,
}

pub(super) struct Params {
    r#type: Waveform,

    pan: u8,
    volume: f32,
    punch: f32,
    attack: f32,
    sustain: f32,
    decay: f32,

    // pitch
    freq: f32,
    limit: f32,
    slide: f32,
    dslide: f32,

    // square wave
    square: f32,
    sweep: f32,

    // vibrato
    vibe: f32,
    vspeed: f32,
    vdelay: f32,

    // hi-pass, lo-pass
    lpf: f32,
    lsweep: f32,
    resonance: f32,
    hpf: f32,
    hsweep: f32,

    // arpeggiator
    arp: f32,
    aspeed: f32,

    // phaser
    phase: f32,
    psweep: f32,

    // repeats?
    repeat: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            r#type: Waveform::Square,
            pan: Default::default(),
            volume: 0.5,
            punch: Default::default(),
            attack: Default::default(),
            sustain: 0.3,
            decay: 0.4,
            freq: 0.3,
            limit: Default::default(),
            slide: Default::default(),
            dslide: Default::default(),
            square: Default::default(),
            sweep: Default::default(),
            vibe: Default::default(),
            vspeed: Default::default(),
            vdelay: Default::default(),
            lpf: 1.0,
            lsweep: Default::default(),
            resonance: Default::default(),
            hpf: Default::default(),
            hsweep: Default::default(),
            arp: Default::default(),
            aspeed: Default::default(),
            phase: Default::default(),
            psweep: Default::default(),
            repeat: Default::default(),
        }
    }
}

pub(super) struct Sound {
    refcount: usize,
    params: Params,
}

pub(super) struct Fx {
    command: FxCommand,
    val: f64,
    r#mod: u8,
}

pub(super) struct Note {
    tone: u8,
    octave: u8,
    duration: u8,
    fx: Vec<Fx>,
}

pub(super) struct Track {
    nlen: i32,
    capa: i32,
    notes: Vec<Note>,
    params: Params,
}

pub(super) struct Voice {
    track: Option<Rc<RefCell<Track>>>,
    params: Params,
    frames: i32,
    nextnote: [i32; 2],
    volume: f32,
    // freq: f32, // unused
    state: State,
    stage: i32,
    time: i32,
    length: [i32; 3],
    period: f64,
    maxperiod: f64,
    slide: f64,
    dslide: f64,
    square: f32,
    sweep: f32,
    phase: i32,
    iphase: i32,
    phasex: i32,
    fphase: f32,
    dphase: f32,
    phaser: [f32; 1024],
    noise: [f32; 32],
    filter: [f32; 8],
    vibe: f32,
    vspeed: f32,
    vdelay: f32,
    repeat: i32,
    limit: i32,
    arp: f64,
    atime: i32,
    alimit: i32,
}

impl Voice {
    fn reset() {}
}

impl Default for Voice {
    fn default() -> Self {
        let params = Params::default();
        let filter2 = f32::powf(params.lpf, 3.0) * 0.1;
        let filter4 = 5.0 / (1.0 + f32::powf(params.resonance, 2.0) * 20.0) * (0.01 + filter2);
        let fphase = f32::powf(params.phase, 2.0) * 1020.0;
        let dphase = f32::powf(params.psweep, 2.0) * 1.0;
        let limit = (f32::powf(1.0 - params.repeat, 2.0) * 20000.0 + 32.0) as i32;

        Self {
            track: None,
            params: Params::default(),
            frames: 0,
            nextnote: [0, 0],
            volume: 0.0,
            state: State::Play,
            stage: 0,
            time: 0,
            length: [
                (params.attack * params.attack * 100000.0) as i32,
                (params.sustain * params.sustain * 100000.0) as i32,
                (params.decay * params.decay * 100000.0) as i32,
            ],
            period: 100.0 / (params.freq as f64 * params.freq as f64 + 0.001),
            maxperiod: 100.0 / (params.limit as f64 * params.limit as f64 + 0.001),
            slide: 1.0 - f64::powf(params.slide as f64, 3.0) * 0.01,
            dslide: -f64::powf(params.dslide as f64, 3.0) * 0.000001,
            square: 0.5 - params.square * 0.5,
            sweep: -params.sweep * 0.00005,
            phase: 0,
            iphase: i32::abs(fphase as i32),
            phasex: 0,
            fphase: if params.phase >= 0.0 { fphase } else { -fphase },
            dphase: if params.psweep >= 0.0 {
                dphase
            } else {
                -dphase
            },
            phaser: [0.0; 1024],
            noise: rand::random::<[f32; 32]>().map(|r| 2.0 * r - 1.0), // array of random f32s
            filter: [
                0.0,
                0.0,
                filter2,
                1.0 + params.lsweep * 0.0001,
                f32::min(filter4, 0.8),
                0.0,
                f32::powf(params.hpf, 2.0) * 0.1,
                1.0 + params.hsweep * 0.0003,
            ],
            vibe: 0.0,
            vspeed: f32::powf(params.vspeed, 2.0) * 0.01,
            vdelay: params.vibe * 0.5,
            repeat: 0,
            limit: if params.repeat == 0.0 { 0 } else { limit },
            arp: if params.arp >= 0.0 {
                1.0 - f64::powf(params.arp as f64, 2.0) * 0.9
            } else {
                1.0 + f64::powf(params.arp as f64, 2.0) * 10.0
            },
            atime: 0,
            alimit: if params.aspeed == 1.0 {
                0
            } else {
                (f32::powf(1.0 - params.aspeed, 2.0) * 20000.0 + 32.0) as i32
            },
        }
    }
}

pub(super) struct Rustaphone {
    tempo: i32,
    volume: f32,
    voices: [Option<Voice>; MAX_TRACKS],
    state: State,
}

impl Rustaphone {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Rustaphone {
            tempo: 120,
            volume: 0.10,
            voices: [UNUSED_VOICE; MAX_TRACKS],
            state: State::Stop,
        }))
    }

    pub fn clear(&mut self) {
        for i in 0..MAX_TRACKS {
            self.set_track_at(None, i);
        }
    }

    fn set_track_at(&mut self, track: Option<Rc<RefCell<Track>>>, index: usize) {
        self.voices[index] = match self.voices[index].take() {
            Some(old_voice) => Some(Voice { track, ..old_voice }),
            None => Some(Voice {
                track,
                ..Default::default()
            }),
        };
    }
}

pub(super) struct Mix {
    channels: [Option<Rc<RefCell<Rustaphone>>>; MAX_CHANNELS],
}

impl Mix {
    pub fn new() -> Self {
        Mix {
            channels: [UNUSED_CHANNEL; MAX_CHANNELS],
        }
    }

    pub fn remove(&mut self, rustaphone: Rc<RefCell<Rustaphone>>) {
        for i in 0..MAX_CHANNELS {
            if let Some(channel) = &self.channels[i] {
                if Rc::ptr_eq(channel, &rustaphone) {
                    self.channels[i] = None;
                }
            }
        }
    }
}
