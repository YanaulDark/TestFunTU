use rand::Rng;

pub fn generate() -> u32 {
    let rng = rand::thread_rng();
    let mut rand_number: u32 = rng.gen_range(0..=100);
    println!("{}", rand_number);
}


