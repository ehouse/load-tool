extern crate num_cpus;

fn main() {

    // count logical cores this process could try to use
    let num = num_cpus::get();
    println!("There are {} Logical CPU's", num);
}
