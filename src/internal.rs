const HI_OCTAVE: u8 = 8;
const MAX_TRACKS: usize = 64;
const MAX_CHANNELS: usize = 64;

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
    refcount: usize,
    nlen: i32,
    capa: i32,
    notes: Vec<Note>,
    params: Params,
}

pub(super) struct Voice {
    track: Track,
    params: Params,
    frames: i32,
    nextnote: [i32; 2],
    volume: f32,
    freq: f32,
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

pub(super) struct Rustaphone {
    refcount: usize,
    tempo: i32,
    volume: f32,
    voices: [Voice; MAX_TRACKS],
    state: State,
}

pub(super) struct Mix {
    channels: [Rustaphone; MAX_CHANNELS],
}
