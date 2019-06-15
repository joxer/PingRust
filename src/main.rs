extern crate pnet;
extern crate rand;

use rand::Rng;
use std::env;
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer3};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::{IcmpPacket, IcmpTypes, IcmpCode, checksum};
use pnet::packet::{Packet, MutablePacket};
use std::net;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use pnet::util;
const IPV4_HEADER_LEN: usize = 21;
const ICMP_HEADER_LEN: usize = 8;
const ICMP_PAYLOAD_LEN: usize = 32;

fn run_app() -> std::result::Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            return ping(&args);
        },
        _ => {
            return Err("error in arguments".to_string());
        }
    }
}

fn ping(args: &Vec<String>) -> std::result::Result<(), String>{
    let (ping_type, addr) = (&args[1], &args[2]);
    match ping_type.as_str() {

        "icmp" => {
            ping_icmp(addr);
        },
        "tcp" =>{

        }
        "udp" => {

        }
        _ => {

        }

    }

    Ok(())
}

/*fn ping_tcp<'a>(buffer: &'a mut[u8]) -> Result<MutableEchoRequestPacket<'a>, &'a str>{

    

}
fn ping_udp<'a>(buffer: &'a mut[u8]) -> Result<MutableEchoRequestPacket<'a>, &'a str>{

    

}
 */

fn ping_icmp(addr: &String) -> std::result::Result<(), String> {
    let mut rng = rand::thread_rng();
    
    let protocol = Layer3(IpNextHeaderProtocols::Icmp);
    let (mut tx, mut rx) = transport_channel(1024, protocol)
        .map_err(|err| format!("Error opening the channel: {}", err)).unwrap();
    let mut rx = icmp_packet_iter(&mut rx);
        
    let to = net::Ipv4Addr::from_str(addr).map_err(|_| return "Invalid address").unwrap();

    let mut seq: u16 = 1u16;
    let id: u16 = rng.gen_range(0, 32768) as u16;
    let mut buffer = [0u8; 80];
    let mut ibuffer = [0u8; 40];

    loop {
        let packet = icmp_packet(&mut buffer[..], &mut ibuffer[..], to, seq, id).unwrap();
        println!("send to {}",  IpAddr::V4(to));
        tx.send_to(packet, IpAddr::V4(to));
        if let Ok((_, addr)) = rx.next() {
            println!("TTL: {}, {}", seq, addr);
        }
        seq+=1;
    }
    Ok(())
}

fn icmp_packet<'a>(buffer: &'a mut[u8], icmp_buffer: &'a mut[u8], destination: net::Ipv4Addr, sequence_number: u16, identifier: u16) -> std::result::Result<pnet::packet::ipv4::MutableIpv4Packet<'a>, &'a str>{

    let mut ipv4_packet = pnet::packet::ipv4::MutableIpv4Packet::new(buffer).unwrap();
    ipv4_packet.set_version(4);
    ipv4_packet.set_ttl(32);
    ipv4_packet.set_header_length(IPV4_HEADER_LEN as u8);
    ipv4_packet.set_total_length((IPV4_HEADER_LEN + ICMP_HEADER_LEN + ICMP_PAYLOAD_LEN) as u16);   
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    ipv4_packet.set_destination(destination);

    let mut ipacket = MutableEchoRequestPacket::new(icmp_buffer).unwrap();
    ipacket.set_sequence_number(sequence_number);
    ipacket.set_identifier(identifier);
    ipacket.set_icmp_type(IcmpTypes::EchoRequest);
    ipacket.set_icmp_code(IcmpCode::new(0));
    ipacket.set_payload("PINGPING".as_bytes());
    let echo_checksum = pnet::packet::icmp::checksum(&IcmpPacket::new(ipacket.packet()).unwrap());
    ipacket.set_checksum(echo_checksum);

    ipv4_packet.set_payload(&ipacket.packet_mut());
    Ok(ipv4_packet)
}



fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
