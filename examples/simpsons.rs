use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustaphone::{Instrument, Mixer, Rustaphone, Waveform};

fn main() {
    let melodious = Instrument::builder()
        .with_waveform(Waveform::Square)
        .with_punch(0.5)
        .with_sustain(0.4)
        .with_decay(0.2)
        .with_arp(0.4)
        .with_aspeed(0.6)
        .with_repeat(0.6)
        .with_phase(0.2)
        .with_psweep(0.2)
        .build();

    let mut simpsons = Rustaphone::new();
    simpsons.add_track(melodious, "32 + C E F# 8:A G E C - 8:A 8:F# 8:F# 8:F# 2:G");

    let mut mixer = Mixer::new();
    mixer.play(simpsons);

    println!("Playing Simpsons theme...");

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = supported_config.into();
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                mixer.synth(config.sample_rate.0, data);
            },
            move |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )
        .expect("error while building stream");
    stream.play().expect("error while playing stream");
}
