use std::sync::mpsc::Sender;

use log::error;
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::{self, NetworkInterface};

pub fn sniff(interface_name: String, app_tx: Sender<String>) -> ! {
    let interface_name_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = pnet_datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_name_match)
        .next()
        .expect(format!("Interface not found: {}", interface_name).as_str());

    let (_, mut rx) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                if packet.len() < 14 {
                    continue;
                }
                app_tx
                    .send(format!("{:?} -> {:?}", &packet[..6], &packet[6..12]))
                    .unwrap();
            }
            Err(e) => error!("Error occurred while catching packets {}", e),
        }
    }
}
