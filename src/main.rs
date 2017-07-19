extern crate syslog_rfc5424;

use std::net::UdpSocket;
use std::str;
use std::io;
use std::error::Error;
use syslog_rfc5424::message::SyslogMessage;
use syslog_rfc5424::parser::ParseErr;

fn recv(s: &UdpSocket, mut buf: &mut[u8]) -> Result<String, ParseErr> {
    let (sz, addr) = s.recv_from(&mut buf).unwrap();
    let ustr = str::from_utf8(&buf[0..sz]).map_err(ParseErr::BaseUnicodeError)?;
    Ok(String::from(ustr))
}
fn parse(utf_data: String) -> Result<SyslogMessage, ParseErr> {
    Ok(syslog_rfc5424::parse_message(&utf_data)?)
}
fn print_msg(msg: SyslogMessage) -> Result<SyslogMessage, ParseErr> {
    println!("{:?} {:?} {:?} {:?}", msg.facility, msg.severity, msg.hostname, msg.msg);
    Ok(msg)
}

fn server() {
    let mut buf = [0u8; 2048];
    let mut socket = UdpSocket::bind("[::]:10514").expect("couldn't bind");
    println!("listening");
    let mut f = || {
        let r = recv(&socket, &mut buf).and_then(parse).and_then(print_msg);
        if r.is_err() {
            let err = r.unwrap_err();
            println!("{:?}", err);
        }
    };
    loop {
        f()
    }

}

fn main() {
    server()
}
