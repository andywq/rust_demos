use std::env;
use std::thread;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

fn main() -> std::io::Result<()> {
    let port = env::args()
        .nth(1)
        .unwrap_or_else(|| "8080".to_string());
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr)?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            let _ = handler(stream);
        });
    }

    Ok(())
}


fn handler(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf)?;
        if buf.starts_with(b"EOF\n") {
            break;
        }

        stream.write_all(b"echo: ")?;
        stream.write_all(&buf[..n])?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}
