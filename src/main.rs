mod synth_tools;
use synth_tools::synth::{self, Synthetizer};

fn main() {
    let mut synth = Synthetizer::new( 44100, 128);
    synth.add_oscillator(400.0, 440.0, |x| {return x.sin()}, 1.0);
    synth.add_oscillator(700.0, 440.0, |x| {return x.sin()}, 1.0);
    let _result = synth.play();
}
