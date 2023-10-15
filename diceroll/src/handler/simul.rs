use rand::Rng;

pub fn dice_roll() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6).to_string()
}