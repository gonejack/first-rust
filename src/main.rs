macro_rules! switch {
    ($($a:expr => $b:expr;)* _ => $e:expr $(,)?) => {
        match () {
            $(_ if $a => $b)*
            _ => $e,
        }
    };
}

use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::{Read, Error, Write};
use std::str;

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    println!("Connection received! {:?} is sending data.", addr);
    // let mut buf = Vec::new();
    // let _ = tcp_stream.read_to_end(&mut buf)?;

    let mut buf = vec![0; 10];
    loop {
        let n = tcp_stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let result = str::from_utf8(&buf[..n]);
        if let Ok(s) = result {
            println!("{:?} says {}", addr, s);
        }
    }
    let _ = tcp_stream.write(b"abc");

    Ok(())
}
