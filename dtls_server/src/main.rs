use std::{
    io::{Read},
    net::UdpSocket,
    thread,
    time::Duration,
};

use udp_dtls::{DtlsAcceptor, Identity};
use udp_dtls::{UdpChannel};

fn main() {
    println!("entering main");
    let error_write = 1;
    let buffer = include_bytes!("../../certs/identity.p12");
    let identity = Identity::from_pkcs12(buffer, "mypass").unwrap();

    let acceptor = DtlsAcceptor::builder(identity).build().unwrap();

    println!("udp bind");
    let server = UdpSocket::bind("127.0.0.1:10102").unwrap();
    let mut client_addr = server.local_addr().unwrap();
    client_addr.set_port(10101);
    println!("{}",client_addr);

    println!("udp channel");
    let server_channel = UdpChannel {
        socket: server,
        remote_addr: client_addr,
    };

    println!("accept");
    let mut dtls_server = acceptor.accept(server_channel).unwrap();

    let mut count = 0;

    println!("entering loop");
    loop {
        let mut received = [0; 5];

        let _result = dtls_server.read_exact(&mut received);

        if _result.is_err() {
            eprintln!("Error: read");
            std::process::exit(error_write);
        }

        println!(
            "{:?} {:?}",
            count,
            String::from_utf8_lossy(received.as_ref())
        );

        count = count + 1;
        thread::sleep(Duration::from_millis(2));
    }

}
