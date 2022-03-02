mod synth_tools;
use synth_tools::synth::{Synthetizer};

fn main() {
    let mut synth = Synthetizer::new( 44100, 128); 
    let fn_oscill = |x: f32| {
        if x.sin().signum() as i8 == 1 {
            return x / std::f32::consts::PI;
        }
        return (x / std::f32::consts::PI) - 2.0f32 ;

    };

    synth.add_oscillator(400.0, 440.0, |x| {return x.sin()}, 1.0);
    synth.add_oscillator(400.0, 440.0, fn_oscill, 1.0);
    synth.add_oscillator(700.0, 440.0, |x| {return x.sin()}, 1.0);
    let _result = synth.play();

    // use std::fs::File;
    // use std::io::BufReader;
    // use rodio::{Decoder, OutputStream, source::Source};

    // Get a output stream handle to the default physical sound device
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    // let file = BufReader::new(File::open("aja.mp3").unwrap());
    // let file2 = BufReader::new(File::open("mozzrt_buscar.mp3").unwrap());

    // Decode that sound file into a source
    // let source = Decoder::new(file).unwrap();
    // let source2 = Decoder::new(file2).unwrap();
    // let nuevo_mix = source.mix(source2);
    // Play the sound directly on the device
    // stream_handle.play_raw(nuevo_mix.convert_samples());

    // c1 = s1.clone()
    // c1 == s1

    // c1 = s1.to_owned()
    // c1 = &*s1
    // int* p1 = espacio
    // int* p2 = p1

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
std::thread::sleep(std::time::Duration::from_secs(5));
}
