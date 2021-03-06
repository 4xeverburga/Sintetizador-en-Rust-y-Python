
use super::instructions;
use core::time::Duration;
use rodio::{OutputStream, source::{Source}};
use std::fs::File;
use std::sync::Arc;
use std::io::Write;

#[derive(Clone)]
struct WavetableOscillator{
   sample_rate: usize,
   pub original_frequency: f32,
   pub oscillator_frequency: f32, // original frequency multiplier 
   wave_table: Vec<f32>,   // table for sound of the oscillator
   wavetable_size: usize,
   pub oscillator_fn: Arc< dyn Fn(f32) -> f32 +Send+Sync+'static>, // Function with period of 2PI. Output must be a normalized float value
   pub volume: f32, // value ranging from 0 to 1 
   index: f32, // index position in the wavetable
   index_increment: f32,   // used to fix the frequency at wich the sound will be played or saved
   contador_callbacks: usize,

}

impl WavetableOscillator {
   fn new(sample_rate: usize, wavetable_size: usize) -> WavetableOscillator {

      return WavetableOscillator {
         sample_rate: sample_rate,
         oscillator_frequency: 440.0,
         original_frequency : 1.0,
         wave_table: vec!(),   // table for sound of the oscillator
         wavetable_size: wavetable_size,
         oscillator_fn: Arc::new(|x: f32| {return x.sin()}), 
         volume: 1.0, // value ranging from 0 to 1 
         index: 0.0, // index position in the wavetable
         index_increment: 0.0, 
         contador_callbacks: 0,
      }
   }

   fn restart(&mut self){
      // Reset the index value to 0.0
      // Must do before getting new samples
      self.index = 0.0;
   }

   fn set_oscillator(&mut self, set_frequency: f32, oscillator_fn: impl Fn(f32) -> f32 +Send+Sync+'static, volume: f32, original_fq: f32 ){

      // Volume
      self.volume = volume;
      // Setting the index increment and frequency for the write/read stream
      self.oscillator_frequency = set_frequency;
      self.original_frequency = original_fq;
      self.index_increment = set_frequency * self.wavetable_size as f32 / self.sample_rate as f32;
      
      // Filling the wavetable
      for n in 0..self.wavetable_size {
         let res = oscillator_fn( 2.0 * std::f32::consts::PI * n as f32 / self.wavetable_size as f32 );
         self.wave_table.push(res);
      }
      self.oscillator_fn = Arc::new(oscillator_fn);
   }

   fn get_sample(&mut self) -> f32 {
      self.contador_callbacks+= 1;
      let sample = self.lerp();
      self.index += self.index_increment;
      self.index %= self.wave_table.len() as f32;
      return sample*self.volume;
   }

   fn lerp(&self) -> f32 {

      let truncated_index = self.index as usize;
      let next_index = (truncated_index + 1) % self.wave_table.len();
      
      let next_index_weight = self.index - truncated_index as f32;
      let truncated_index_weight = 1.0 - next_index_weight;

      return truncated_index_weight * self.wave_table[truncated_index] 
            + next_index_weight * self.wave_table[next_index];
   }
}
   
impl Iterator for WavetableOscillator {
   type Item = f32;
   
   fn next(&mut self) -> Option<Self::Item> {
         return Some(self.get_sample());
   }
}

impl Source for WavetableOscillator {
   fn channels(&self) -> u16 {
       return 1;
   }

   fn sample_rate(&self) -> u32 {
       return self.sample_rate as u32;
   }   

   fn current_frame_len(&self) -> Option<usize> {
       return None;
   }

   fn total_duration(&self) -> Option<Duration> {
       return None;
   }
}

#[derive(Clone)]
pub struct Synthetizer {
   sample_rate: usize,
   main_frequency: f32,
   wave_table: Vec<f32>,   // table for sound of the synth. Mix of the different oscillator voices
   wavetable_size: usize,
   playback_table: Vec<f32>,  // table used to write/read a voice line 
   oscillators: Vec<WavetableOscillator>, // oscillators voices conforming the synth voice 
   pub index_playback: usize,
   index: f32, // index position in the wavetable
   index_increment: f32,   // set the frequency at wich the sound will be played or saved
}

impl Synthetizer{

   pub fn new(sample_rate: usize, wavetable_size: usize) -> Synthetizer {
      return Synthetizer{
         sample_rate: sample_rate,
         main_frequency: 0.0,
         wave_table: vec!(),
         wavetable_size: wavetable_size,
         playback_table: vec!(),
         oscillators: vec!(),
         index_playback: 0,
         index: 0.0,
         index_increment: 0.0,
      }
   }
   pub fn set_main_fq(&mut self, frequency: f32){
      self.main_frequency = frequency;
   }

   pub fn add_oscillator(&mut self, set_frequency: f32, original_fq: f32, oscillator_fn: impl Fn(f32) -> f32 +Send+Sync+'static, volume: f32){
      let mut osc = WavetableOscillator::new(self.sample_rate, self.wavetable_size);
      osc.set_oscillator(set_frequency, oscillator_fn, volume, original_fq);
      self.oscillators.push(osc);
   }

   pub fn show_oscillators(&self){
      if self.oscillators.len() == 0 {
         println!("No hay osciladores a??adidos al sintetizador");
         return ();
      }

      for i in 0..self.oscillators.len() {
         println!("> Oscilador n??mero {}:", i);
         println!("Oscillator original fq: {} ", self.oscillators[i].original_frequency);
         println!("Oscillator volume: {}", self.oscillators[i].volume);
      }
   }

   pub fn play(&self){
      use rodio::Sink;
      use rodio::dynamic_mixer::mixer;
     
      let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
      let (mixer_controller, dynamic_mixer) 
      = mixer(1, self.sample_rate as u32);
      for i in 0..self.oscillators.len(){
         eprintln!("Mixing oscillators");
         // mix_source = oscillator.to_owned().mix(self.oscillators[i].clone());
         mixer_controller.add(self.oscillators[i].clone());
      }
      
      let sink = Sink::try_new(&stream_handle).unwrap();
      sink.set_volume(0.01);
      sink.append(dynamic_mixer);

      std::thread::sleep(std::time::Duration::from_secs(5));

   }

   pub fn play_self(&self, seconds: f32){
      use rodio::Sink;
      let (_stream, stream_handle) = OutputStream::try_default().unwrap();
      let sink = Sink::try_new(&stream_handle).unwrap();
      sink.set_volume(0.1);
      println!("Playing the playback_vector!");
      sink.append(self.to_owned());
      std::thread::sleep(std::time::Duration::from_millis( ((seconds)*1000 as f32) as u64));
   }

   pub fn gen_playback_table(&mut self, voice_instructions: &mut instructions::VoiceInstruction ){
      //Extensive allocation of memory containing the information of the sound being generated
      //The table must be converted from the binary file created
      // Asumes all the paths are already build
      // [=, &, ](parametros){fucnion}
      let mut playback_data = File::create("playback_table.syv").unwrap(); // synth voice
      // The samples will be taken at the same sample velocity of the synth
      let num_oscillators = self.oscillators.len();
      let mut function_osc_x = vec!(0.0f32; num_oscillators);

      for &(fq,vol) in &voice_instructions.path {
         
         let mut sample = 0.0;
         let mut i = 0;
         for oscillator in &self.oscillators {
            // Calculing the relative pitch change for every oscillator
            function_osc_x[i] += (std::f32::consts::TAU*fq*oscillator.oscillator_frequency) / (self.main_frequency*self.sample_rate as f32) ;

            let fq_func = &oscillator.oscillator_fn;
            sample += fq_func(function_osc_x[i])*oscillator.volume;
            i+=1;
         }

         let res = (sample*vol)/num_oscillators as f32;
         writeln!(playback_data, "{}", res);
         self.playback_table.push(res);

      }
   }
   pub fn gen_playback_test(&mut self){
      let mut playback_data = File::create("playback_table_test.txt").unwrap(); // synth voice
      let step = 1.0 / self.sample_rate as f32;
      let mut acum = 0.0;
      let counter = self.oscillators.len();
      while acum < 5.0 {
         let mut sample = 0.0;
         for i in 0..counter {
            sample += self.oscillators[i].get_sample();
         }
         writeln!(playback_data,"{}",sample/counter as f32);
         self.playback_table.push(sample/counter as f32);
         acum += step;
      }
   }
   pub fn get_sample(&mut self) -> f32{

      let index = self.index_playback % self.playback_table.len();
      self.index_playback += 1;
      return self.playback_table[index]

   }
}

impl Iterator for Synthetizer {
   type Item = f32;
    
   fn next(&mut self) -> Option<Self::Item> {
      return Some(self.get_sample());
   }  
}

impl Source for Synthetizer {
   fn channels(&self) -> u16 {
       return 1;
   }

   fn sample_rate(&self) -> u32 {
       return self.sample_rate as u32;
   }   

   fn current_frame_len(&self) -> Option<usize> {
       return None;
   }

   fn total_duration(&self) -> Option<Duration> {
      None
   }
}
