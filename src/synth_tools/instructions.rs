use std::sync::Arc;
pub mod notes{
pub const C1: f32 = 32.70;
pub const Db1: f32 = 34.65;
pub const D1: f32 = 36.71;
pub const Eb1: f32 = 38.89;
pub const E1: f32 = 41.20;
pub const F1: f32 = 43.65;
pub const Gb1: f32 = 46.25;
pub const G1: f32 = 49.00;
pub const Ab1: f32 = 51.91;
pub const A1: f32 = 55.00;
pub const B1: f32 = 61.73541;
}

pub mod atacks {
   pub fn simple(t: f32) -> f32 {
      let t = 4.0*t;
      if t > 1.0 {
         return 0.1;
      } else {
         return ( 3.0*t.powf(2.0)-2.0*t.powf(3.0) ) / 10.0
      }

   }
}
// pub const Db: f32 = 34.65;
// pub const Db: f32 = 34.65;
// pub const Db: f32 = 34.65;
// pub const Db: f32 = 34.65;
// pub const Db: f32 = 34.65;
// pub const Db: f32 = 34.65;
// pub const Db: f32 = 34.65;
// pub const Db: f32 = 34.65;
   // "Bb"=> 58.27,
   // "B"=> 61.74,
pub fn note_fq(note: &str, octave: u8) -> f32 {
   return ( 2^( (octave-1) as u8) as u8) as f32 * match note.to_owned().to_uppercase().as_str(){
   "C"=> 32.70,
   "Db" => 34.65,
   "D" => 36.71,
   "Eb" | "D#" => 38.89,
   "E" | "Fb"=> 41.20,
   "F"=> 43.65,
   "Gb"=> 46.25, 
   "G"=> 49.00,
   "Ab"=> 51.91,
   "A"=> 55.00,
   "Bb"=> 58.27,
   "B"=> 61.74,
   _ => panic!("Nota no reconocida!")
   };
}

pub enum VoiceFqPath {
   Function (Arc< dyn Fn(f32) -> f32 +Send+Sync+'static>), // varying frequency. "t" starts at 0.0 
   // Constant (f32) // the frequency of the voice will stay constant during the specified time
}
pub enum VoiceVolPath {
   Function (Arc< dyn Fn(f32) -> f32 +Send+Sync+'static>), // varying volume. "t" starts at 0.0 
   // Constant (f32) // the volume of the voice will stay constant during the specified time
}
pub struct VoiceInstruction {
   pub instructions: Vec<(f32, VoiceFqPath, VoiceVolPath)>,
   pub path: Vec<(f32,f32)>
}

impl VoiceInstruction {
   pub fn new(instructions: Vec<(f32, VoiceFqPath, VoiceVolPath)>) -> VoiceInstruction {
      return VoiceInstruction{
         instructions: instructions,
         path: vec!()
      }
   }
   pub fn build_path(&mut self, sample_rate: usize){

      let step = ( 1.0 as f64 / sample_rate as f64 ) as f32; 
      for (seconds, fq_mode, vol_mode) in &self.instructions{

         let mut acumulator = 0.0;
         let fq_fn = match fq_mode {
            // Destructure "f" from inside the enum 
            VoiceFqPath::Function(f) => f, // f : f(f32) -> f32
         };
         let vol_fn = match vol_mode {
            VoiceVolPath::Function(f) => f,

         };
         
         while acumulator <= *seconds {
            self.path.push( ( fq_fn(acumulator), vol_fn(acumulator)) ) ;
         
            acumulator += step;
         }

      }
   }
}