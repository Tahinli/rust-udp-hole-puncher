use std::{io, net::UdpSocket};

fn main() {
    println!("Hello, world!");
    println!("Addr");
    let mut addr = String::default();
    io::stdin().read_line(&mut addr).unwrap();
    let addr = addr.trim_end();
    println!("Server | Client");
    let mut input = String::default();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end();
    match input {
        "server" | "Server" | "SERVER" | "s" => server(addr),
        "client" | "Client" | "CLIENT" | "c" => client(addr),
        _ => {}
    }
}

fn server(addr: &str) {
    let udp_socket = match UdpSocket::bind(addr) {
        Ok(udp_socket) => udp_socket,
        Err(err_val) => {
            eprintln!("{}", err_val);
            return;
        }
    };
    let mut buffer = [0_u8; 100];
    let mut peers = vec![];
    for _ in 0..2 {
        let (data_size, peer) = match udp_socket.recv_from(&mut buffer) {
            Ok((data_size, peer)) => (data_size, peer),
            Err(err_val) => {
                eprintln!("{}", err_val);
                return;
            }
        };
        peers.push(format!("{}:{}", peer.ip(), peer.port()));
        let data = buffer.split_at(data_size).0;
        let message = String::from_utf8_lossy(data).into_owned();
        println!("{}:{} = {}", peer.ip(), peer.port(), message);
    }

    match udp_socket.send_to(peers[1].as_bytes(), &peers[0]) {
        Ok(_) => println!("Sent First"),
        Err(err_val) => {
            eprintln!("{}", err_val);
            return;
        }
    };
    match udp_socket.send_to(peers[0].as_bytes(), &peers[1]) {
        Ok(_) => println!("Sent Second"),
        Err(err_val) => eprintln!("{}", err_val),
    };
}

fn client(addr: &str) {
    let udp_socket = match UdpSocket::bind(addr) {
        Ok(udp_socket) => udp_socket,
        Err(err_val) => {
            eprintln!("{}", err_val);
            return;
        }
    };
    match udp_socket.send_to(
        "Hello Sir".as_bytes(),
        "0.0.0.0", /* I change it with public address */
    ) {
        Ok(_) => {}
        Err(err_val) => {
            eprintln!("{}", err_val);
        }
    };
    let mut buffer = [0_u8; 100];
    let (data_size, peer) = match udp_socket.recv_from(&mut buffer) {
        Ok((data_size, peer)) => (data_size, peer),
        Err(err_val) => {
            eprintln!("{}", err_val);
            return;
        }
    };
    let data = buffer.split_at(data_size).0;
    let message = String::from_utf8_lossy(data).into_owned();
    println!("{}:{} = {}", peer.ip(), peer.port(), message);

    match udp_socket.send_to("Hello Dear".as_bytes(), message) {
        Ok(_) => {}
        Err(err_val) => {
            eprintln!("{}", err_val);
            return;
        }
    };

    let mut buffer = [0_u8; 100];
    let (data_size, peer) = match udp_socket.recv_from(&mut buffer) {
        Ok((data_size, peer)) => (data_size, peer),
        Err(err_val) => {
            eprintln!("{}", err_val);
            return;
        },
    };
    let data = buffer.split_at(data_size).0;
    let message = String::from_utf8_lossy(data).into_owned();
    println!("{}:{} = {}", peer.ip(), peer.port(), message);
}
