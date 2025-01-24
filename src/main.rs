#![allow(unused)]
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

mod config; use config::CONFIG;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind(&CONFIG.bind_addr).await?;

    if CONFIG.verbose {
        eprintln!("Listening on {}", &CONFIG.bind_addr);
    }

    loop {
        let (mut socket, addr) = listener.accept().await?;
        if CONFIG.verbose {
            eprintln!("Connection from {}", &addr);
        }

        // Start a new task to handle the client
        tokio::task::spawn(async move  {
            match handle_client(&mut socket, &addr).await {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("{}: Error: {}", &addr, &e);
                }
            }
            if CONFIG.verbose {
                eprintln!("Connection from {} completed", &addr);
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}

pub async fn handle_client(socket: &mut TcpStream, addr: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {

    let filename: String = addr.ip().to_string();
    if std::fs::exists(&filename)? {
        // Serve the content back to the client, then unlink
        let mut file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&filename)
            .await?;
        let bytes = tokio::io::copy(&mut file, socket).await?;
        eprintln!("{} byte(s) to from file \"{}\" -> TCP client {}", bytes, &filename, &addr.ip());
        tokio::fs::remove_file(&filename).await?;
    } else {
        // Store the content from the client
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filename)
            .await?;
        let bytes = tokio::io::copy(socket, &mut file).await?;
        eprintln!("{} byte(s) from TCP client {} -> file \"{}\"", bytes, &addr, &filename);
    }

    Ok(())
}