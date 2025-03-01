mod internal;

const MAX_CHANNELS: usize = 64;
const UNUSED_CHANNEL: Option<Rustaphone> = None;

pub struct Rustaphone {
    internal: internal::Rustaphone,
}

impl Rustaphone {
    pub fn new() -> Rustaphone {
        Rustaphone {
            internal: internal::Rustaphone::new(120, 0.10),
        }
    }

    pub fn add_track(&mut self, instrument: Instrument, tune: &str) {
        let track = internal::Track::new(instrument, tune);
        self.internal.add_track(track);
    }
}

impl Default for Rustaphone {
    fn default() -> Self {
        Rustaphone::new()
    }
}

pub struct StopHandle {
    channel: usize,
}

pub struct Mixer {
    channels: [Option<Rustaphone>; MAX_CHANNELS],
}

impl Mixer {
    pub fn new() -> Self {
        Mixer {
            channels: [UNUSED_CHANNEL; MAX_CHANNELS],
        }
    }

    pub fn play(&mut self, mut rustaphone: Rustaphone) -> Option<StopHandle> {
        rustaphone.internal.play();

        for i in 0..MAX_CHANNELS {
            match &self.channels[i] {
                Some(old) if old.internal.is_done() => continue,
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
            channel.internal.stop();
        }
        self.is_done()
    }

    pub fn is_done(&self) -> bool {
        self.channels.iter().all(|channel| {
            if let Some(channel) = channel {
                channel.internal.is_done()
            } else {
                true
            }
        })
    }

    pub fn synth(&mut self, sample_rate: usize, buffer: &mut [f32]) {
        for sample in buffer {
            let mut allsample = 0.0;

            for c in 0..MAX_CHANNELS {
                let Some(channel) = &mut self.channels[c] else {
                    continue;
                };

                channel.internal.synth(sample_rate, &mut allsample);
            }

            *sample = allsample;
        }
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Mixer::new()
    }
}

#[derive(Clone)]
pub struct Instrument {
    params: internal::Params,
}

impl Instrument {
    pub fn builder() -> InstrumentBuilder {
        InstrumentBuilder {
            params: internal::Params::default(),
        }
    }

    pub fn square() -> Self {
        let params = internal::Params {
            r#type: Waveform::Square,
            volume: 0.5,
            sustain: 0.3,
            decay: 0.4,
            freq: 0.3,
            lpf: 1.0,
            ..Default::default()
        };

        Instrument { params }
    }
}

pub struct InstrumentBuilder {
    params: internal::Params,
}

impl InstrumentBuilder {
    pub fn build(self) -> Instrument {
        Instrument {
            params: self.params,
        }
    }

    pub fn with_waveform(mut self, waveform: Waveform) -> InstrumentBuilder {
        self.params.r#type = waveform;
        self
    }

    pub fn with_pan(mut self, pan: u8) -> InstrumentBuilder {
        self.params.pan = pan;
        self
    }

    pub fn with_volume(mut self, volume: f32) -> InstrumentBuilder {
        self.params.volume = volume;
        self
    }

    pub fn with_punch(mut self, punch: f32) -> InstrumentBuilder {
        self.params.punch = punch;
        self
    }

    pub fn with_attack(mut self, attack: f32) -> InstrumentBuilder {
        self.params.attack = attack;
        self
    }

    pub fn with_sustain(mut self, sustain: f32) -> InstrumentBuilder {
        self.params.sustain = sustain;
        self
    }

    pub fn with_decay(mut self, decay: f32) -> InstrumentBuilder {
        self.params.decay = decay;
        self
    }

    pub fn with_freq(mut self, freq: f32) -> InstrumentBuilder {
        self.params.freq = freq;
        self
    }

    pub fn with_limit(mut self, limit: f32) -> InstrumentBuilder {
        self.params.limit = limit;
        self
    }

    pub fn with_slide(mut self, slide: f32) -> InstrumentBuilder {
        self.params.slide = slide;
        self
    }

    pub fn with_dslide(mut self, dslide: f32) -> InstrumentBuilder {
        self.params.dslide = dslide;
        self
    }

    pub fn with_square(mut self, square: f32) -> InstrumentBuilder {
        self.params.square = square;
        self
    }

    pub fn with_sweep(mut self, sweep: f32) -> InstrumentBuilder {
        self.params.sweep = sweep;
        self
    }

    pub fn with_vibe(mut self, vibe: f32) -> InstrumentBuilder {
        self.params.vibe = vibe;
        self
    }

    pub fn with_vspeed(mut self, vspeed: f32) -> InstrumentBuilder {
        self.params.vspeed = vspeed;
        self
    }

    pub fn with_vdelay(mut self, vdelay: f32) -> InstrumentBuilder {
        self.params.vdelay = vdelay;
        self
    }

    pub fn with_lpf(mut self, lpf: f32) -> InstrumentBuilder {
        self.params.lpf = lpf;
        self
    }

    pub fn with_lsweep(mut self, lsweep: f32) -> InstrumentBuilder {
        self.params.lsweep = lsweep;
        self
    }

    pub fn with_resonance(mut self, resonance: f32) -> InstrumentBuilder {
        self.params.resonance = resonance;
        self
    }

    pub fn with_hpf(mut self, hpf: f32) -> InstrumentBuilder {
        self.params.hpf = hpf;
        self
    }

    pub fn with_hsweep(mut self, hsweep: f32) -> InstrumentBuilder {
        self.params.hsweep = hsweep;
        self
    }

    pub fn with_arp(mut self, arp: f32) -> InstrumentBuilder {
        self.params.arp = arp;
        self
    }

    pub fn with_aspeed(mut self, aspeed: f32) -> InstrumentBuilder {
        self.params.aspeed = aspeed;
        self
    }

    pub fn with_phase(mut self, phase: f32) -> InstrumentBuilder {
        self.params.phase = phase;
        self
    }

    pub fn with_psweep(mut self, psweep: f32) -> InstrumentBuilder {
        self.params.psweep = psweep;
        self
    }

    pub fn with_repeat(mut self, repeat: f32) -> InstrumentBuilder {
        self.params.repeat = repeat;
        self
    }
}

#[derive(Clone, PartialEq)]
pub enum Waveform {
    Square,
    Sawtooth,
    Sine,
    Noise,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
