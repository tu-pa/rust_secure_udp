use std::{
    io::{Write},
    net::UdpSocket,
    thread,
    time::Duration,
};

use udp_dtls::{Certificate, DtlsConnector, SrtpProfile};
use udp_dtls::{UdpChannel};

fn main() {
    println!("entering main");
    let error_write = 1;

    // TODO: System Configuration
    let root_ca = include_bytes!("../../certs/root-ca.der");
    let root_ca = Certificate::from_der(root_ca).unwrap();

    //let acceptor = DtlsAcceptor::builder(identity).build().unwrap();
    let connector = DtlsConnector::builder()
        .add_srtp_profile(SrtpProfile::Aes128CmSha180)
        .add_srtp_profile(SrtpProfile::AeadAes256Gcm)
        .add_root_certificate(root_ca)
        .build()
        .unwrap();

    println!("udp bind");
    // TODO: Control Channel Configuration
    let client = UdpSocket::bind("127.0.0.1:10101").unwrap();
    let mut server_addr = client.local_addr().unwrap();
    server_addr.set_port(10102);
    println!("{}",server_addr);

    println!("udp channel");
    let client_channel = UdpChannel {
        socket: client,
        remote_addr: server_addr,
    };

    println!("connect");
    // TODO: Control Channel Configuration
    let mut dtls_client = connector.connect("foobar.com", client_channel).unwrap();

    println!("entering loop");
    loop {
        let _buf = [0; 5];

        let _buf = b"hello";
        let _result = dtls_client.write_all(_buf);


        if _result.is_err() {
            eprintln!("Error: write");
            std::process::exit(error_write);
        }

        thread::sleep(Duration::from_millis(30));
    }
}
