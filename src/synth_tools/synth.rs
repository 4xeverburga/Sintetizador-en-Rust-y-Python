
pub fn nothing_synth(){
   print!("nothing -from synth");
}

use core::time::Duration;
use rodio::{OutputStream, source::{Source, Mix}};

#[derive(Clone)]
struct WavetableOscillator{
   sample_rate: usize,
   original_frequency: f32,
   oscillator_frequency: f32, // original frequency multiplier 
   wave_table: Vec<f32>,   // table for sound of the oscillator
   wavetable_size: usize,
   oscillator_fn: fn(x: f32) -> f32, // Function with period of 2PI. Output must be a normalized float value
   volume: f32, // value ranging from 0 to 1 
   index: f32, // index position in the wavetable
   index_increment: f32,   // used to fix the frequency at wich the sound will be played or saved
}

impl WavetableOscillator {
   fn new(sample_rate: usize, wavetable_size: usize) -> WavetableOscillator {

      return WavetableOscillator {
         sample_rate: sample_rate,
         oscillator_frequency: 440.0,
         original_frequency : 1.0,
         wave_table: vec!(),   // table for sound of the oscillator
         wavetable_size: wavetable_size,
         oscillator_fn: |x| {return x.sin()}, 
         volume: 1.0, // value ranging from 0 to 1 
         index: 0.0, // index position in the wavetable
         index_increment: 0.0, 
      }
   }

   fn restart(&mut self){
      // Reset the index value to 0.0
      // Must do before getting new samples
      self.index = 0.0;
   }

   fn set_oscillator(&mut self, frequency: f32, oscillator_fn: fn(x: f32) -> f32, volume: f32, original_fq: f32 ){

      // Volume
      self.volume = volume;
      // Setting the index increment and frequency for the write/read stream
      self.oscillator_frequency = frequency;
      self.original_frequency = original_fq;
      self.index_increment = frequency * self.wavetable_size as f32 / self.sample_rate as f32;
      
      // Filling the wavetable
      for n in 0..self.wavetable_size {
         let res = oscillator_fn( 2.0 * std::f32::consts::PI * n as f32 / self.wavetable_size as f32 );
         self.wave_table.push(res);
      }
   }

   fn get_sample(&mut self) -> f32 {
      let sample = self.lerp();
      self.index += self.index_increment;
      self.index %= self.wave_table.len() as f32;
      // if self.index > self.wave_table.len() as f32 {
      //    self.index = 0.0;
      // }
      return sample;
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
   wave_table: Vec<f32>,   // table for sound of the synth. Mix of the different oscillator voices
   wavetable_size: usize,
   playback_table: Vec<f32>,  // table used to write/read a voice line 
   oscillators: Vec<WavetableOscillator>, // oscillators voices conforming the synth voice 
   index: f32, // index position in the wavetable
   index_increment: f32,   // set the frequency at wich the sound will be played or saved
}

impl Synthetizer{

   pub fn new(sample_rate: usize, wavetable_size: usize) -> Synthetizer {
      return Synthetizer{
         sample_rate: sample_rate,
         wave_table: vec!(),
         wavetable_size: wavetable_size,
         playback_table: vec!(),
         oscillators: vec!(),
         index: 0.0,
         index_increment: 0.0
      }
   }

   pub fn add_oscillator( &mut self, frequency: f32, original_fq: f32, oscillator_fn: fn(x: f32) -> f32, volume: f32){
      let mut osc = WavetableOscillator::new(self.sample_rate, self.wavetable_size);
      osc.set_oscillator(frequency, oscillator_fn, volume, original_fq);
      self.oscillators.push(osc);
   }

   pub fn show_oscillators(&self){
      if self.oscillators.len() == 0 {
         println!("No hay osciladores añadidos al sintetizador");
         return ();
      }

      for i in 0..self.oscillators.len() {
         println!("> Oscilador número {}:", i);
         println!("Oscillator original fq: {} ", self.oscillators[i].original_frequency);
         println!("Oscillator volume: {}", self.oscillators[i].volume);
      }
   }

   pub fn play(&self){

      // pub fn copy_shuffle<T: Clone>(vec: &[T]) -> Vec<T> {
      //    let mut vec = vec.to_vec();
      //    shuffle(&mut vec);
      //    vec
   // }
     
      let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
      let oscillator = self.oscillators[0].clone();
      // algo.iter() -> &T
      // algo.into_iter() -> T



      // let mut oscillator = WavetableOscillator::new( self.sample_rate, 128);
      // oscillator.set_oscillator(440.0, |x|{return x.sin()}, 1.0, 440.0);
      let mix_source =  oscillator.to_owned().mix(self.oscillators[1].clone());
      // for i in 1..self.oscillators.len(){
      //    eprintln!("Mixing oscillators");
      //    mix_source = oscillator.to_owned().mix(self.oscillators[i].clone());
      // }
      // mix<Item> -> Mix<self, item>
      // pub struct Mix<I1, I2> where
      //     I1: Source,
      //     I1::Item: Sample,
      //     I2: Source,
      //     I2::Item: Sample,  { /* private fields */ }

      // let stream = self.clone().to_owned();
      let _result = stream_handle.play_raw(mix_source.convert_samples());
      // self.convert_samples
      std::thread::sleep(std::time::Duration::from_secs(5));

   }

   // recorrer
   // for val in vect.iter() {
      ///// mas codifo 
   // }
 
// impl Series {
//     pub fn iter(&self) -> std::iter::Copied<std::slice::Iter<Float>> {
//         self.data.iter().copied()
//     }
// }

   // fn todo( &self, valor_prestado: WavetableOscillator ){
   //    for i in 1..self.oscillators.len(){
   //       valor_prestado.to_owned().mix(self.oscillators[i].clone());
   //    };

   // }

   pub fn set_wavetable(&self){
      //Must be at least 1 oscillator
      // let mut mIndex: usize = 0;
      // let mut minor = self.oscillators[0].oscillator_frequency;

      //calculate optimal wavetable size algorithm 
      //still working
      // for i in 0..self.oscillators.len(){
      //    if self.oscillators[i].oscillator_frequency < minor {
      //       mIndex = i;
      //       minor = self.oscillators[i].oscillator_frequency;
      //    }
      // }

   }

   pub fn get_sample(&mut self) -> f32{

      let mut sample = 0.0;
      let counter = self.oscillators.len();
      for i in 0..self.oscillators.len() {
         sample += self.oscillators[i].get_sample();
         eprint!("{}",self.oscillators[i].get_sample());
      }
      // for val in self.oscillators.iter() {
      //    sample += val.to_owned().clone().get_sample();
      // }
      return sample;
   }
}

impl Iterator for Synthetizer {
   type Item = f32;
    
   fn next(&mut self) -> Option<Self::Item> {
      // return Some(self.get_sample());
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
       return None;
   }
}
