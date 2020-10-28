use serialport::TTYPort;
use std::{
    io::{Read, Write},
    thread,
    time::Duration,
};

fn main() {

    let (mut master, mut slave) = TTYPort::pair().unwrap();

    thread::spawn(move || {
        master.write_all(&[5, 6, 7, 8])
        .expect("Failed to write to master");
    thread::sleep(Duration::from_millis(1000));    
    });

    // Read back four bytes from the master
    let mut buf: [u8; 1] = [0; 1];
    loop {
        match slave.read(&mut buf) {
            Ok(bytes) => {
                if bytes == 1 {
                    println!("received: {:?}", buf);
                }
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::TimedOut => (),
            Err(ref err) if err.kind() == std::io::ErrorKind::BrokenPipe => break,
            Err(e) => {
                eprintln!("{:?}", e);
            },
        }
    }
}
