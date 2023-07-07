#![warn(rust_2018_idioms)]

use std::time::Duration;

use common::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const LISTEN: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Fallible<()> {

    println!("Iterations: {ITERATIONS},  Samples: {SAMPLES}");

    for (gen, value) in (GEN_MIN ..= GEN_MAX).enumerate() {

        let chunk_size = 2usize.pow(value);
        let server_chunk_size = chunk_size;
        let client_chunk_size = chunk_size;

        let server = tokio::spawn(server(server_chunk_size));

        // Wait for listener
        tokio::time::sleep(Duration::from_millis(500)).await;

        let client = tokio::spawn(client(client_chunk_size));

        let (
            (recv_bytes, recv_time),
            (sent_bytes, sent_time)) = tokio::try_join!(server, client)?;

        show_result(gen ,chunk_size, sent_bytes, recv_bytes, sent_time, recv_time);

        // Wait for shutdown
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}


async fn server(chunk_size: usize) -> (usize, Duration) {
    let mut recv_bytes = 0;
    let mut recv_time  = Duration::ZERO;

    let listener = TcpListener::bind(LISTEN).await.unwrap();

    let (mut stream, _) = listener.accept().await.unwrap();

    let mut buf = vec![0; chunk_size];


    loop {
        let timer = Instant::now();

        let n = stream
            .read(&mut buf)
            .await
            .expect("failed to read data from stream");

        if n == 0 {
            break;
        }

        recv_time  += timer.elapsed();
        recv_bytes += chunk_size;
    }

    (recv_bytes, recv_time)
}

async fn client(chunk_size: usize) -> (usize, Duration) {
    let mut stream = TcpStream::connect(LISTEN).await.unwrap();

    let mut sent_bytes = 0;
    let mut sent_time  = Duration::ZERO;

    let samples = gen_data_samples(chunk_size);
    let mut samples = samples.iter().cycle();

    // let mut buf = vec![0; server_chunk_size];

    for _ in 0 .. ITERATIONS {

        let sample = samples.next().expect("Failed to get next sample");

        let timer = Instant::now();

        stream
            .write_all(&sample[..])
            .await
            .expect("failed to write data to stream");

        sent_time  += timer.elapsed();
        sent_bytes += chunk_size;
    }

    (sent_bytes, sent_time)
}