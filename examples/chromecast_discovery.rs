extern crate mdns;

use mdns::{Record, RecordKind};
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const SERVICE_NAME: &'static str = "_googlecast._tcp.local";

fn main() {
    let should_stop = Arc::new(AtomicBool::new(false));
    should_stop.store(true, Ordering::SeqCst);
    
    for response in mdns::discover::all(SERVICE_NAME, should_stop.clone()).unwrap() {
        let response = response.unwrap();

        let addr = response.records()
                           .filter_map(self::to_ip_addr)
                           .next();

        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

