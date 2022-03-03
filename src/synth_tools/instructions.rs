pub enum VoiceFqPath {
   Function (fn(t: f32) -> f32), // varying frequency. "t" starts at 0.0 
   // Constant (f32) // the frequency of the voice will stay constant during the specified time
}
pub enum VoiceVolPath {
   Function (fn(t: f32) -> f32), // varying volume. "t" starts at 0.0 
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
         let fq_fn = &match *fq_mode {
            VoiceFqPath::Function(f) => f,
            // VoiceFqPath::Constant(fq) =>,
            // _ => |x| x ,
         };
         let vol_fn = &match *vol_mode {
            VoiceVolPath::Function(f) => f,
            // VoiceVolPath::Constant => |x| x,
            // _ => |x| x ,
         };
         while acumulator <= *seconds {
            self.path.push( (fq_fn(acumulator), vol_fn(acumulator)) );
            acumulator += step;
         }

      }
   }
}