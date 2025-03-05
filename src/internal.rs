mod notation;

const HI_OCTAVE: u8 = 8;
const MAX_TRACKS: usize = 4;

const UNUSED_VOICE: Option<Voice> = None;

#[derive(Clone, Default, PartialEq)]
pub(super) enum State {
    #[default]
    Stop,
    Play,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub(super) struct Params {
    pub r#type: super::Waveform,

    pub pan: u8,
    pub volume: f32,
    pub punch: f32,
    pub attack: f32,
    pub sustain: f32,
    pub decay: f32,

    // pitch
    pub freq: f32,
    pub limit: f32,
    pub slide: f32,
    pub dslide: f32,

    // square wave
    pub square: f32,
    pub sweep: f32,

    // vibrato
    pub vibe: f32,
    pub vspeed: f32,
    pub vdelay: f32,

    // hi-pass, lo-pass
    pub lpf: f32,
    pub lsweep: f32,
    pub resonance: f32,
    pub hpf: f32,
    pub hsweep: f32,

    // arpeggiator
    pub arp: f32,
    pub aspeed: f32,

    // phaser
    pub phase: f32,
    pub psweep: f32,

    // repeats?
    pub repeat: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            r#type: super::Waveform::Square,
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

macro_rules! fx {
    ($f:ident, $a:ident, $v:ident) => {{
        if $f.r#mod == '+' {
            $a.params.$v += $f.val;
        } else if $f.r#mod == '-' {
            $a.params.$v -= $f.val;
        } else {
            $a.params.$v = $f.val;
        }

        if $a.params.$v > 1.0 {
            $a.params.$v = 1.0;
        } else if $a.params.$v < 0.0 {
            $a.params.$v = 0.0;
        }
    }};
}

#[derive(Clone)]
pub(super) struct Fx {
    command: FxCommand,
    val: f32,
    r#mod: char,
}

#[derive(Clone)]
pub(super) struct Note {
    tone: char,
    octave: u8,
    duration: u8,
    fx: Vec<Fx>,
}

impl Note {
    fn freq(&self) -> f32 {
        match self.tone {
            'A' =>
            // A
            {
                match self.octave {
                    1 => 0.121,
                    2 => 0.175,
                    3 => 0.248,
                    4 => 0.353,
                    5 => 0.500,
                    _ => 0.0,
                }
            }

            'b' =>
            // A# or Bb
            {
                match self.octave {
                    1 => 0.125,
                    2 => 0.181,
                    3 => 0.255,
                    4 => 0.364,
                    5 => 0.516,
                    _ => 0.0,
                }
            }
            'B' =>
            // B
            {
                match self.octave {
                    1 => 0.129,
                    2 => 0.187,
                    3 => 0.263,
                    4 => 0.374,
                    5 => 0.528,
                    _ => 0.0,
                }
            }

            'C' =>
            // C
            {
                match self.octave {
                    2 => 0.133,
                    3 => 0.192,
                    4 => 0.271,
                    5 => 0.385,
                    6 => 0.544,
                    _ => 0.0,
                }
            }
            'd' =>
            // C# or Db
            {
                match self.octave {
                    2 => 0.138,
                    3 => 0.198,
                    4 => 0.279,
                    5 => 0.395,
                    6 => 0.559,
                    _ => 0.0,
                }
            }
            'D' =>
            // D
            {
                match self.octave {
                    2 => 0.143,
                    3 => 0.202,
                    4 => 0.287,
                    5 => 0.406,
                    6 => 0.575,
                    _ => 0.0,
                }
            }
            'e' =>
            // D# or Eb
            {
                match self.octave {
                    2 => 0.148,
                    3 => 0.208,
                    4 => 0.296,
                    5 => 0.418,
                    6 => 0.593,
                    _ => 0.0,
                }
            }
            'E' =>
            // E
            {
                match self.octave {
                    2 => 0.152,
                    3 => 0.214,
                    4 => 0.305,
                    5 => 0.429,
                    6 => 0.608,
                    _ => 0.0,
                }
            }
            'F' =>
            // F
            {
                match self.octave {
                    2 => 0.155,
                    3 => 0.220,
                    4 => 0.314,
                    5 => 0.441,
                    _ => 0.0,
                }
            }
            'g' =>
            // F# or Gb
            {
                match self.octave {
                    2 => 0.160,
                    3 => 0.227,
                    4 => 0.323,
                    5 => 0.454,
                    _ => 0.0,
                }
            }
            'G' =>
            // G
            {
                match self.octave {
                    2 => 0.164,
                    3 => 0.234,
                    4 => 0.332,
                    5 => 0.468,
                    _ => 0.0,
                }
            }
            'a' =>
            // G# or Ab
            {
                match self.octave {
                    1 => 0.117,
                    2 => 0.170,
                    3 => 0.242,
                    4 => 0.343,
                    5 => 0.485,
                    _ => 0.0,
                }
            }
            _ => 0.0,
        }
    }
}

#[derive(Clone)]
pub(super) struct Track {
    notes: Vec<Note>,
    params: Params,
}

impl Track {
    pub fn new(instrument: super::Instrument, tune: &str) -> Self {
        Track {
            notes: notation::tune(tune).unwrap().1,
            params: instrument.params,
        }
    }
}

#[derive(Clone)]
struct Phaser([f32; 1024]);

impl Default for Phaser {
    fn default() -> Self {
        Self([0.0; 1024])
    }
}

#[derive(Clone, Default)]
pub(super) struct Voice {
    track: Option<Track>,
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
    phaser: Phaser,
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
    fn reset(&mut self) {
        self.period = 100.0 / (self.params.freq as f64 * self.params.freq as f64 + 0.001);
        self.maxperiod = 100.0 / (self.params.limit as f64 * self.params.limit as f64 + 0.001);
        self.slide = 1.0 - f64::powf(self.params.slide as f64, 3.0) * 0.01;
        self.dslide = -f64::powf(self.params.dslide as f64, 3.0) * 0.000001;
        self.square = 0.5 - self.params.square * 0.5;
        self.sweep = -self.params.sweep * 0.00005;
        self.arp = if self.params.arp >= 0.0 {
            1.0 - f64::powf(self.params.arp as f64, 2.0) * 0.9
        } else {
            1.0 + f64::powf(self.params.arp as f64, 2.0) * 10.0
        };
        self.atime = 0;
        self.alimit = if self.params.aspeed == 1.0 {
            0
        } else {
            (f32::powf(1.0 - self.params.aspeed, 2.0) * 20000.0 + 32.0) as i32
        };
    }

    fn start(&mut self) {
        self.phase = 0;
        let filter2 = f32::powf(self.params.lpf, 3.0) * 0.1;
        let filter4 = 5.0 / (1.0 + f32::powf(self.params.resonance, 2.0) * 20.0) * (0.01 + filter2);
        self.filter = [
            0.0,
            0.0,
            filter2,
            1.0 + self.params.lsweep * 0.0001,
            f32::min(filter4, 0.8),
            0.0,
            f32::powf(self.params.hpf, 2.0) * 0.1,
            1.0 + self.params.hsweep * 0.0003,
        ];

        self.vibe = 0.0;
        self.vspeed = f32::powf(self.params.vspeed, 2.0) * 0.01;
        self.vdelay = self.params.vibe * 0.5;

        self.volume = 0.0;
        self.stage = 0;
        self.time = 0;
        self.length = [
            (self.params.attack * self.params.attack * 100000.0) as i32,
            (self.params.sustain * self.params.sustain * 100000.0) as i32,
            (self.params.decay * self.params.decay * 100000.0) as i32,
        ];

        let fphase = f32::powf(self.params.phase, 2.0) * 1020.0;
        self.fphase = if self.params.phase >= 0.0 {
            fphase
        } else {
            -fphase
        };
        let dphase = f32::powf(self.params.psweep, 2.0) * 1.0;
        self.dphase = if self.params.psweep >= 0.0 {
            dphase
        } else {
            -dphase
        };
        self.iphase = i32::abs(self.fphase as i32);
        self.phasex = 0;

        self.phaser = Default::default();
        self.noise = rand::random::<[f32; 32]>().map(|r| 2.0 * r - 1.0); // array of random f32s // TODO: verify this is actually the same as the C version

        self.repeat = 0;
        let limit = (f32::powf(1.0 - self.params.repeat, 2.0) * 20000.0 + 32.0) as i32;
        self.limit = if self.params.repeat == 0.0 { 0 } else { limit };
        self.state = State::Play;
    }
}

#[derive(Clone)]
pub struct Rustaphone {
    tempo: i32,
    volume: f32,
    voices: [Option<Voice>; MAX_TRACKS],
    state: State,
}

impl Rustaphone {
    pub fn new(tempo: i32, volume: f32) -> Self {
        Rustaphone {
            tempo,
            volume,
            voices: [UNUSED_VOICE; MAX_TRACKS],
            state: State::Stop,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..MAX_TRACKS {
            self.set_track_at(None, i);
        }
    }

    pub fn tempo(&mut self, tempo: i32) {
        self.tempo = tempo;
    }

    pub fn play(&mut self) {
        for i in 0..MAX_TRACKS {
            if let Some(voice) = &mut self.voices[i] {
                if let Some(track) = &voice.track {
                    voice.params = track.params.clone();
                    voice.reset();
                    voice.start();
                    voice.frames = 0;
                    voice.nextnote = [0; 2];
                }
            }
        }

        self.state = State::Play;
    }

    pub fn stop(&mut self) {
        self.state = State::Stop;
    }

    pub fn is_done(&self) -> bool {
        self.state == State::Stop
    }

    pub fn add_track(&mut self, track: Track) {
        for i in 0..MAX_TRACKS {
            if self.voices[i].is_none() {
                self.set_track_at(Some(track), i);
                return;
            }
        }
    }

    pub fn synth(&mut self, sample_rate: u32, allsample: &mut f32) {
        let mut moreframes = 0;

        for t in 0..MAX_TRACKS {
            let Some(a) = &mut self.voices[t] else {
                continue;
            };

            let Some(track) = a.track.clone() else {
                continue;
            };

            if !track.notes.is_empty() {
                if a.frames == a.nextnote[0] {
                    if a.nextnote[1] < track.notes.len() as i32 {
                        let note = &track.notes[a.nextnote[1] as usize];
                        let mut freq = a.params.freq;
                        if note.tone != 'n' {
                            freq = note.freq();
                        }
                        if freq == 0.0 {
                            a.period = 0.0;
                            a.state = State::Stop;
                        } else {
                            let note = &track.notes[a.nextnote[1] as usize];
                            for fx in &note.fx {
                                match fx.command {
                                    FxCommand::Volume => fx!(fx, a, volume),
                                    FxCommand::Punch => fx!(fx, a, punch),
                                    FxCommand::Attack => fx!(fx, a, attack),
                                    FxCommand::Sustain => fx!(fx, a, sustain),
                                    FxCommand::Decay => fx!(fx, a, decay),
                                    FxCommand::Square => fx!(fx, a, square),
                                    FxCommand::Sweep => fx!(fx, a, sweep),
                                    FxCommand::Vibe => fx!(fx, a, vibe),
                                    FxCommand::VSpeed => fx!(fx, a, vspeed),
                                    FxCommand::VDelay => fx!(fx, a, vdelay),
                                    FxCommand::Lpf => fx!(fx, a, lpf),
                                    FxCommand::LSweep => fx!(fx, a, lsweep),
                                    FxCommand::Resonance => fx!(fx, a, resonance),
                                    FxCommand::Hpf => fx!(fx, a, hpf),
                                    FxCommand::HSweep => fx!(fx, a, hsweep),
                                    FxCommand::Arp => fx!(fx, a, arp),
                                    FxCommand::ASpeed => fx!(fx, a, aspeed),
                                    FxCommand::Phase => fx!(fx, a, phase),
                                    FxCommand::PSweep => fx!(fx, a, psweep),
                                    FxCommand::Repeat => fx!(fx, a, repeat),
                                }
                            }

                            a.reset();
                            a.start();
                            a.period = 100.0 / (freq * freq + 0.001) as f64;
                        }

                        a.nextnote[0] += (sample_rate as f32 / (self.tempo as f32 / 60.0)
                            * (4.0 / note.duration as f32))
                            as i32;
                    }

                    a.nextnote[1] += 1;
                }
                if a.nextnote[1] <= track.notes.len() as i32 {
                    moreframes += 1;
                }
            } else {
                moreframes += 1;
            }

            a.frames += 1;

            if a.state == State::Stop {
                continue;
            }

            a.repeat += 1;
            if a.limit != 0 && a.repeat >= a.limit {
                a.repeat = 0;
                a.reset();
            }

            a.atime += 1;
            if a.alimit != 0 && a.atime >= a.alimit {
                a.alimit = 0;
                a.period *= a.arp;
            }

            a.slide += a.dslide;
            a.period *= a.slide;
            if a.period > a.maxperiod {
                a.period = a.maxperiod;
                if a.params.limit > 0.0 {
                    a.state = State::Stop;
                }
            }

            let mut rfperiod = a.period as f32;
            if a.vdelay > 0.0 {
                a.vibe += a.vspeed;
                rfperiod = a.period as f32 * (1.0 + f32::sin(a.vibe) * a.vdelay);
            }

            let mut period = rfperiod as i32;
            if period < 8 {
                period = 8;
            }
            a.square += a.sweep;
            a.square = a.square.clamp(0.0, 0.5);

            a.time += 1;
            while a.time >= a.length[a.stage as usize] {
                a.time = 0;
                a.stage += 1;
                if a.stage == 3 {
                    a.state = State::Stop;
                    break; // TODO: is this correct?
                }
            }

            match a.stage {
                0 => {
                    a.volume = a.time as f32 / a.length[0] as f32;
                }
                1 => {
                    a.volume =
                        1.0 + (1.0 - a.time as f32 / a.length[1] as f32) * 2.0 * a.params.punch;
                }
                2 => {
                    a.volume = 1.0 - a.time as f32 / a.length[2] as f32;
                }
                _ => {}
            }

            a.fphase += a.dphase;
            a.iphase = (a.fphase as i32).abs();
            if a.iphase > 1023 {
                a.iphase = 1023;
            }

            if a.filter[7] != 0.0 {
                a.filter[6] *= a.filter[7];
                a.filter[6] = a.filter[6].clamp(0.00001, 0.1);
            }

            let mut ssample = 0.0;
            for _ in 0..8 {
                a.phase += 1;
                if a.phase >= period {
                    a.phase %= period;
                    if a.params.r#type == super::Waveform::Noise {
                        for i in 0..32 {
                            a.noise[i] = rand::random::<f32>() * 2.0 - 1.0;
                            // TODO: verify this is actually the same as the C version.
                            // TODO: Init array without loop as above.
                        }
                    }
                }

                let fp = a.phase as f32 / period as f32;
                let mut sample = match a.params.r#type {
                    super::Waveform::Square => {
                        if fp < a.square {
                            0.5
                        } else {
                            -0.5
                        }
                    }
                    super::Waveform::Sawtooth => 1.0 - fp * 2.0,
                    super::Waveform::Sine => f32::sin(fp * 2.0 * core::f32::consts::PI),
                    super::Waveform::Noise => a.noise[(a.phase * 32 / period) as usize],
                };

                let pp = a.filter[0];
                a.filter[2] *= a.filter[3];
                a.filter[2] = a.filter[2].clamp(0.0, 0.1);
                if a.params.lpf != 1.0 {
                    a.filter[1] += (sample - a.filter[0]) * a.filter[2];
                    a.filter[1] -= a.filter[1] * a.filter[4];
                } else {
                    a.filter[0] = sample;
                    a.filter[1] = 0.0;
                }
                a.filter[0] += a.filter[1];

                a.filter[5] += a.filter[0] - pp;
                a.filter[5] -= a.filter[5] * a.filter[6];
                sample = a.filter[5];

                a.phaser.0[(a.phasex & 1023) as usize] = sample;
                sample += a.phaser.0[((a.phasex - a.iphase + 1024) & 1023) as usize];
                a.phasex = (a.phasex + 1) & 1023;

                ssample += sample * a.volume;
            }
            ssample = ssample / 8.0 * self.volume;
            ssample *= 2.0 * a.params.volume;

            ssample = ssample.clamp(-1.0, 1.0);
            *allsample += ssample;
        }

        if moreframes == 0 {
            self.state = State::Stop;
        }
    }

    fn set_track_at(&mut self, track: Option<Track>, index: usize) {
        self.voices[index] = match self.voices[index].take() {
            Some(old_voice) => {
                let params = match &track {
                    Some(track) => track.params.clone(),
                    None => old_voice.params,
                };

                Some(Voice {
                    track,
                    params,
                    state: State::Stop,
                    frames: 0,
                    nextnote: [0; 2],
                    ..old_voice
                })
            }
            None => Some(Voice {
                track,
                ..Default::default()
            }),
        };
    }
}
