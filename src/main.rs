//use std::net::{UdpSocket, IpAddr, Ipv4Addr};
use std::process::Command;
use std::str;
use std::time::Instant;


fn main() {
    let now = Instant::now();

    let nmap = Command::new("nmap")
        .arg("-sP")
        .arg("192.168.10.0/24")
        .output()
        .expect("failed to execute process");

    let text = str::from_utf8(&nmap.stdout)
        .unwrap()
        .split("\n")
        .filter(|&x| x.starts_with("Nmap scan"));

    let mut addrs = Vec::new();
    for line in text {
        let start = line.find("192").unwrap_or(0);
        let end;
        if line[line.len()-1..line.len()].to_string() == ")" {
            end = line.find(")").unwrap_or(0);
        } else {
            end = line.len();
        }
        addrs.push(line[start..end].to_string());
    }

    for addr in addrs {
        let ping = Command::new("ping")
            .arg("-c 1")
            .arg(addr)
            .output()
            .expect("failed to execute ping");
        let ping_out = str::from_utf8( &ping.stdout ).unwrap();
        println!("{}\n", ping_out);
    }

    println!("elapsed time: {} ms", now.elapsed().as_millis());


}

/*
fn main() {
    let mut addr_array: [std::net::IpAddr; 100] = [IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)); 100];
    let mut lsb : u8 = 51;
    for entry in addr_array.iter_mut() {
        *entry = IpAddr::V4(Ipv4Addr::new(192, 168, 2, lsb));
        lsb += 1;
    }

    for entry in addr_array.iter() {
        println!("{}", entry.to_string());
    }

    let socket1 = UdpSocket::bind("145.137.154.92:35565").expect("Couldn't bind to address");
    let socket2 = UdpSocket::bind("145.137.154.92:35566").expect("Couldn't bind to address");

    let mut buf = [1; 32];
    socket2.send_to(&[255; 32], "145.137.154.92:35565").expect("Couldn't send data");
    let (number_of_bytes, src_addr) = socket1.recv_from(&mut buf).expect("Didn't receive any data");

    let filled_buf = &mut buf[..number_of_bytes];
    for entry in filled_buf {
        print!("{}, ", entry);
    }
    println!("; {}", src_addr);
}*/