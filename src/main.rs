mod synth_tools;
use synth_tools::synth;

fn main() {
    synth::nothing_synth();
    let v = vec!(1,1);
    for _i in 0..v.len() {
        print!("aver");
    }
}
