mod synth_tools;
use synth_tools::instructions;
use synth_tools::instructions::*;
use synth_tools::instructions::notes;
use synth_tools::synth::{Synthetizer};
use std::sync::Arc;


fn main() {

    let mut semifusa = 0.2;
    let mut synth = Synthetizer::new( 44100, 128); 
    let fn_oscill_saw = |x: f32| {

        let x = x % std::f32::consts::TAU;
        if x.sin().signum() as i8 == 1 {
            return x / std::f32::consts::PI;
        }
        return (x / std::f32::consts::PI) - 2.0f32 ;
    };

    //synthetizing an oscillator
    const MAIN_TONE: f32 = 100.0;
    synth.set_main_fq(MAIN_TONE);
    synth.add_oscillator(MAIN_TONE, 440.0, |x| {return x.sin()}, 0.9);
    //Overtones
    synth.add_oscillator(MAIN_TONE*2.0, 440.0, |x| {return x.sin()}, 0.5);
    synth.add_oscillator(MAIN_TONE*3.0, 440.0, |x| {return x.sin()}, 0.3);
    synth.add_oscillator(MAIN_TONE*4.0, 440.0, |x| {return x.sin()}, 0.1);
    synth.add_oscillator(MAIN_TONE*5.0, 440.0, |x| {return x.sin()}, 0.9);
    synth.add_oscillator(MAIN_TONE*6.0, 440.0, |x| {return x.sin()}, 0.1);
    synth.add_oscillator(MAIN_TONE*7.0, 440.0, |x| {return x.sin()}, 0.1);
    synth.add_oscillator(MAIN_TONE*8.0, 440.0, |x| {return x.sin()}, 0.1);
    synth.add_oscillator(100.0, 440.0, fn_oscill_saw, 0.8);

    
    //Writing voice instructions for the oscillator
    let mut controles  = vec!( (3.0, instructions::VoiceFqPath::Function( Arc::new( move |_x:f32| 200.0+_x.sin().abs()*300.0*semifusa ) ), instructions::VoiceVolPath::Function( Arc::new(|_x| 0.08)) ) );
    

    //attaching instructions 
    let mut voice_instructions = instructions::VoiceInstruction::new(controles);
    voice_instructions.build_path(44100);
    //generating .syv file and vector frequency table
    synth.gen_playback_table(&mut voice_instructions);
    //playing 
    synth.play_self(10.0);

}
