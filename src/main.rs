use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt};
use rand::{Rng, distributions::Uniform};
use std::time::Duration;

#[tokio::main]
async fn main() {
    const BUFFER_SIZE: usize = 4;
    let mut socket = UnixStream::connect("/tmp/socket1.sock").await.unwrap();
    println!("Connected to server");

    let mut rnd = rand::thread_rng();
    let range = Uniform::new_inclusive(1, 255);
    loop {
        let mut buffer = vec![0; BUFFER_SIZE];
        for  i  in 0..BUFFER_SIZE {
            let random_number: u8 = rnd.sample(range);
            buffer[i] = random_number;
        }
        println!("buffer: {:?}", buffer);
        match socket.write_all(&buffer).await {
            Ok(socket) => socket,
            Err(e) => eprintln!("Error sending data: {}", e),
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
