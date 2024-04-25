mod internal;

pub struct Rustaphone {
    tempo: u32,
}

impl Rustaphone {
    pub fn new(tempo: u32) -> Rustaphone {
        Rustaphone { tempo }
    }

    pub fn play(&self) {
        println!("Playing at {} bpm", self.tempo);
    }
}

impl Sound {
    pub fn builder(waveform: Waveform) -> Sound {
        let pan = 0;

        let volume = 0.5;

        let punch = 0.0;
        let attack = 0.0;

        let sustain = 0.3;
        let decay = 0.4;
        let freq = 0.3;

        let limit = 0.0;
        let slide = 0.0;
        let dslide = 0.0;
        let square = 0.0;
        let sweep = 0.0;
        let vibe = 0.0;
        let vspeed = 0.0;
        let vdelay = 0.0;

        let lpf = 1.0;

        let lsweep = 0.0;
        let resonance = 0.0;
        let hpf = 0.0;
        let hsweep = 0.0;
        let arp = 0.0;
        let aspeed = 0.0;
        let phase = 0.0;
        let psweep = 0.0;
        let repeat = 0.0;

        Sound {
            waveform,
            pan,
            volume,
            punch,
            attack,
            sustain,
            decay,
            freq,
            limit,
            slide,
            dslide,
            square,
            sweep,
            vibe,
            vspeed,
            vdelay,
            lpf,
            lsweep,
            resonance,
            hpf,
            hsweep,
            arp,
            aspeed,
            phase,
            psweep,
            repeat,
        }
    }

    pub fn with_pan(mut self, pan: u8) -> Sound {
        self.pan = pan;
        self
    }

    pub fn with_volume(mut self, volume: f64) -> Sound {
        self.volume = volume;
        self
    }

    pub fn with_punch(mut self, punch: f64) -> Sound {
        self.punch = punch;
        self
    }

    pub fn with_attack(mut self, attack: f64) -> Sound {
        self.attack = attack;
        self
    }

    pub fn with_sustain(mut self, sustain: f64) -> Sound {
        self.sustain = sustain;
        self
    }

    pub fn with_decay(mut self, decay: f64) -> Sound {
        self.decay = decay;
        self
    }

    pub fn with_freq(mut self, freq: f64) -> Sound {
        self.freq = freq;
        self
    }

    pub fn with_limit(mut self, limit: f64) -> Sound {
        self.limit = limit;
        self
    }

    pub fn with_slide(mut self, slide: f64) -> Sound {
        self.slide = slide;
        self
    }

    pub fn with_dslide(mut self, dslide: f64) -> Sound {
        self.dslide = dslide;
        self
    }

    pub fn with_square(mut self, square: f64) -> Sound {
        self.square = square;
        self
    }

    pub fn with_sweep(mut self, sweep: f64) -> Sound {
        self.sweep = sweep;
        self
    }

    pub fn with_vibe(mut self, vibe: f64) -> Sound {
        self.vibe = vibe;
        self
    }

    pub fn with_vspeed(mut self, vspeed: f64) -> Sound {
        self.vspeed = vspeed;
        self
    }

    pub fn with_vdelay(mut self, vdelay: f64) -> Sound {
        self.vdelay = vdelay;
        self
    }

    pub fn with_lpf(mut self, lpf: f64) -> Sound {
        self.lpf = lpf;
        self
    }

    pub fn with_lsweep(mut self, lsweep: f64) -> Sound {
        self.lsweep = lsweep;
        self
    }

    pub fn with_resonance(mut self, resonance: f64) -> Sound {
        self.resonance = resonance;
        self
    }

    pub fn with_hpf(mut self, hpf: f64) -> Sound {
        self.hpf = hpf;
        self
    }

    pub fn with_hsweep(mut self, hsweep: f64) -> Sound {
        self.hsweep = hsweep;
        self
    }

    pub fn with_arp(mut self, arp: f64) -> Sound {
        self.arp = arp;
        self
    }

    pub fn with_aspeed(mut self, aspeed: f64) -> Sound {
        self.aspeed = aspeed;
        self
    }

    pub fn with_phase(mut self, phase: f64) -> Sound {
        self.phase = phase;
        self
    }

    pub fn with_psweep(mut self, psweep: f64) -> Sound {
        self.psweep = psweep;
        self
    }

    pub fn with_repeat(mut self, repeat: f64) -> Sound {
        self.repeat = repeat;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
