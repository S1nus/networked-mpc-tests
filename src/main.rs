use std::env;
use std::net::SocketAddrV4;
use async_std::prelude::*;

use async_std::{task,
    io::{BufReader},
    net::{TcpListener, TcpStream},
    stream::{Stream, StreamExt},
};

use serde_json::de::Deserializer;

async fn run_server(addr: SocketAddrV4) {
    let listener = TcpListener::bind(addr)
        .await
        .expect(format!("Failed to bind to {}", addr).as_str());

    println!("Listening on {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Got a stream from {:?}!", addr);
        task::spawn(connection_loop(stream));
    }
}

async fn connection_loop(stream: TcpStream) {
    //let mut lines = reader.lines();
    let deserializer = Deserializer::from_reader(stream);
    
    /*while let Some(line) = lines.next().await {
        println!("Got line: {:?}", line);
    }*/
    println!("done");
}

async fn connect_to_player(addr: SocketAddrV4) -> TcpStream {
    TcpStream::connect(addr)
        .await
        .expect(format!("Failed to connect to {}", addr).as_str())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let player_number : u8 = args[1]
        .parse()
        .expect("unable to parse supplied player number arg");

    match player_number {
        0 => {
            task::block_on(run_server(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 5001)));
        },
        1 => {
            task::spawn(run_server(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 5002)));
            let mut p1_stream = task::block_on(connect_to_player(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 5001)));
            task::block_on(p1_stream.write_all("Poopy poop\n".as_bytes()));
            loop {}
        },
        2 => {
            let p0_stream = task::block_on(connect_to_player(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 5001)));
            let p1_stream = task::block_on(connect_to_player(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 5002)));
            loop {}
        },
        _ => {
            panic!("Invalid player number supplied");
        }
    }

}
