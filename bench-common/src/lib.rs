
pub use std::{
    error::Error,
    time::{
        Duration,
        Instant,
    },
    future::Future,
};

use rand::Rng;
use humansize::{FormatSizeOptions, format_size};

pub const GEN_MIN: u32 = 2;
pub const GEN_MAX: u32 = 12;
pub const SAMPLES: usize  = 10;
pub const ITERATIONS: u32 = 100_000;



pub type Fallible<T> = Result<T, Box<dyn Error>>;

pub fn gen_data_samples(size: usize) -> Vec<Vec<u8>> {


    let mut samples = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0 .. SAMPLES {

        let mut data = vec![0; size];

        rng.fill(&mut data[..]);

        samples.push(data);

    }

    samples
}

pub fn show_result(
    gen: usize,
    size: usize,
    sent_bytes: usize,
    recv_bytes: usize,
    sent_time: Duration,
    recv_time: Duration,
) {

    let custom_options = FormatSizeOptions::from(humansize::BINARY).decimal_places(2);

    let gen = gen + 1;

    let send_speed = format_size((sent_bytes as u128 / sent_time.as_millis()) as u64, custom_options);
    let recv_speed = format_size((recv_bytes as u128 / recv_time.as_millis()) as u64, custom_options);

    println!("Generation:{gen:<2} Chunk size:{size:<4} Bytes: {size:>4}x{ITERATIONS} | Sent: {send_speed:>10}/ms | Recv: {recv_speed:>10}/ms");
}
