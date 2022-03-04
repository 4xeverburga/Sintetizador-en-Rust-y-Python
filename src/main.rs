mod synth_tools;
use synth_tools::instructions;
use synth_tools::synth::{Synthetizer};

fn main() {

    let mut synth = Synthetizer::new( 44100, 128); 
    let fn_oscill = |x: f32| {

        let x = x % std::f32::consts::TAU;
        if x.sin().signum() as i8 == 1 {
            return x / std::f32::consts::PI;
        }
        return (x / std::f32::consts::PI) - 2.0f32 ;

    };
    synth.set_main_fq(400.0);
    synth.add_oscillator(200.0, 440.0, |x| {return x.sin()}, 0.8);
    synth.add_oscillator(400.0, 440.0, fn_oscill, 0.5);
    // synth.add_oscillator(100.0, 440.0, |x| {return x.sin()}, 0.0);
    
    let controles = vec!( (5.0, instructions::VoiceFqPath::Function(|_x:f32| 200.0+_x.sin().abs()*300.0), instructions::VoiceVolPath::Function( |_x| 0.08)) );
    let mut voice_instructions = instructions::VoiceInstruction::new(controles);
    voice_instructions.build_path(44100);
    synth.gen_playback_table(&mut voice_instructions);
    // synth.gen_playback_test();
    synth.play_self(10.0);

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
}
