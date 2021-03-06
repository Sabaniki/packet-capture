use crate::packet::{GettableEndPoints, print_packet_info};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

// TCPパケットを構築する。
//引数のパケットにはIPパケットが入り、それの皮を剥いたものを次に渡す。
pub fn tcp_handler(packet: &dyn GettableEndPoints) {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}

//UDPパケットを構築する
//引数のパケットにはIPパケットが入り、それの皮を剥いたものを次に渡す。
pub fn udp_handler(packet: &dyn GettableEndPoints) {
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "UDP");
    }
}