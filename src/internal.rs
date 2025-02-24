use std::sync::{Arc, Mutex};

const HI_OCTAVE: u8 = 8;
const MAX_TRACKS: usize = 64;
const MAX_CHANNELS: usize = 64;

const UNUSED_VOICE: Option<Voice> = None;
const UNUSED_CHANNEL: Option<Rustaphone> = None;

#[derive(Clone, Default, PartialEq)]
pub(super) enum State {
    #[default]
    Stop,
    Play,
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
struct Phaser([f32; 1024]);

impl Default for Phaser {
    fn default() -> Self {
        Self([0.0; 1024])
    }
}

#[derive(Clone, Default)]
pub(super) struct Voice {
    track: Option<Arc<Mutex<Track>>>,
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
        self.noise = rand::random::<[f32; 32]>().map(|r| 2.0 * r - 1.0); // array of random f32s

        self.repeat = 0;
        let limit = (f32::powf(1.0 - self.params.repeat, 2.0) * 20000.0 + 32.0) as i32;
        self.limit = if self.params.repeat == 0.0 { 0 } else { limit };
        self.state = State::Play;
    }
}

#[derive(Clone)]
pub(super) struct Rustaphone {
    tempo: i32,
    volume: f32,
    voices: [Option<Voice>; MAX_TRACKS],
    state: State,
}

impl Rustaphone {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Rustaphone {
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

    pub fn tempo(&mut self, tempo: i32) {
        self.tempo = tempo;
    }

    pub fn play(&mut self) {
        for i in 0..MAX_TRACKS {
            if let Some(voice) = &mut self.voices[i] {
                if let Some(track) = &voice.track {
                    voice.params = track.lock().unwrap().params.clone();
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

    fn set_track_at(&mut self, track: Option<Arc<Mutex<Track>>>, index: usize) {
        self.voices[index] = match self.voices[index].take() {
            Some(old_voice) => {
                let params = match &track {
                    Some(track) => track.lock().unwrap().params.clone(),
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

pub struct StopHandle {
    channel: usize,
}

pub(super) struct Mix {
    channels: [Option<Rustaphone>; MAX_CHANNELS],
}

impl Mix {
    pub fn new() -> Self {
        Mix {
            channels: [UNUSED_CHANNEL; MAX_CHANNELS],
        }
    }

    pub fn play(&mut self, mut rustaphone: Rustaphone) -> Option<StopHandle> {
        rustaphone.play();

        for i in 0..MAX_CHANNELS {
            match &self.channels[i] {
                Some(old) if old.state != State::Stop => continue,
                _ => {
                    self.channels[i] = Some(rustaphone);
                    return Some(StopHandle { channel: i });
                }
            }
        }

        None
    }

    pub fn stop(&mut self, handle: StopHandle) -> bool {
        if let Some(channel) = &mut self.channels[handle.channel] {
            channel.stop();
        }
        self.is_done()
    }

    fn is_done(&self) -> bool {
        self.channels.iter().all(|channel| {
            if let Some(channel) = channel {
                channel.is_done()
            } else {
                true
            }
        })
    }
}
