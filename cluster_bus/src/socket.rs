#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_mut)]
#![allow(clippy::collapsible_if)]
#[allow(dead_code)]

use core::str;
use log::{debug, error, info};

use smoltcp::iface::{InterfaceBuilder, NeighborCache};
use smoltcp::phy::{Loopback, Medium};
use smoltcp::socket::{TcpSocket, TcpSocketBuffer};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};


mod mock {
    use core::cell::Cell;
    use smoltcp::time::{Duration, Instant};

    #[derive(Debug)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Clock(Cell<Instant>);

    impl Clock {
        pub fn new() -> Clock {
            Clock(Cell::new(Instant::from_millis(0)))
        }

        pub fn advance(&self, duration: Duration) {
            self.0.set(self.0.get() + duration)
        }

        pub fn elapsed(&self) -> Instant {
            self.0.get()
        }
    }
}

pub fn test() {
    let clock = mock::Clock::new();
    let device = Loopback::new(Medium::Ethernet);
    println!("enter cluster bus ");

    #[cfg(feature = "std")]
    let device = {
        let clock = clock.clone();
        utils::setup_logging_with_clock("", move || clock.elapsed());

        let (mut opts, mut free) = utils::create_options();
        utils::add_middleware_options(&mut opts, &mut free);

        let mut matches = utils::parse_options(&opts, free);
        utils::parse_middleware_options(&mut matches, device, /*loopback=*/ true)
    };

    let mut neighbor_cache_entries = [None; 8];
    let mut neighbor_cache = NeighborCache::new(&mut neighbor_cache_entries[..]);

    let mut ip_addrs = [IpCidr::new(IpAddress::v4(127, 0, 0, 1), 8)];
    let mut sockets: [_; 2] = Default::default();
    let mut iface = InterfaceBuilder::new(device, &mut sockets[..])
        .hardware_addr(EthernetAddress::default().into())
        .neighbor_cache(neighbor_cache)
        .ip_addrs(ip_addrs)
        .finalize();

    let server_socket = {
        // It is not strictly necessary to use a `static mut` and unsafe code here, but
        // on embedded systems that smoltcp targets it is far better to allocate the data
        // statically to verify that it fits into RAM rather than get undefined behavior
        // when stack overflows.
        static mut TCP_SERVER_RX_DATA: [u8; 1024] = [0; 1024];
        static mut TCP_SERVER_TX_DATA: [u8; 1024] = [0; 1024];
        let tcp_rx_buffer = TcpSocketBuffer::new(unsafe { &mut TCP_SERVER_RX_DATA[..] });
        let tcp_tx_buffer = TcpSocketBuffer::new(unsafe { &mut TCP_SERVER_TX_DATA[..] });
        TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer)
    };

    let client_socket = {
        static mut TCP_CLIENT_RX_DATA: [u8; 1024] = [0; 1024];
        static mut TCP_CLIENT_TX_DATA: [u8; 1024] = [0; 1024];
        let tcp_rx_buffer = TcpSocketBuffer::new(unsafe { &mut TCP_CLIENT_RX_DATA[..] });
        let tcp_tx_buffer = TcpSocketBuffer::new(unsafe { &mut TCP_CLIENT_TX_DATA[..] });
        TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer)
    };

    let server_handle = iface.add_socket(server_socket);
    let client_handle = iface.add_socket(client_socket);

    let mut did_listen = false;
    let mut did_connect = false;
    let mut done = false;
    while !done && clock.elapsed() < Instant::from_millis(10_000) {
        match iface.poll(clock.elapsed()) {
            Ok(_) => {}
            Err(e) => {
                println!("poll error: {}", e);
            }
        }

        let mut socket = iface.get_socket::<TcpSocket>(server_handle);
        if !socket.is_active() && !socket.is_listening() {
            if !did_listen {
                println!("listening");
                socket.listen(1234).unwrap();
                did_listen = true;
            }
        }

        if socket.can_recv() {
            println!(
                "got {:?}",
                socket.recv(|buffer| { (buffer.len(), str::from_utf8(buffer).unwrap()) })
            );
            socket.close();
            done = true;
        }

        let (mut socket, cx) = iface.get_socket_and_context::<TcpSocket>(client_handle);
        if !socket.is_open() {
            if !did_connect {
                println!("connecting");
                socket
                    .connect(
                        cx,
                        (IpAddress::v4(127, 0, 0, 1), 1234),
                        (IpAddress::Unspecified, 65000),
                    )
                    .unwrap();
                did_connect = true;
            }
        }

        if socket.can_send() {
            println!("sending");
            socket.send_slice(b"0123456789abcdef").unwrap();
            socket.close();
        }

        match iface.poll_delay(clock.elapsed()) {
            Some(Duration::ZERO) => debug!("resuming"),
            Some(delay) => {
                println!("sleeping for {} ms", delay);
                clock.advance(delay)
            }
            None => clock.advance(Duration::from_millis(1)),
        }
    }

    if done {
        println!("done")
    } else {
        println!("this is taking too long, bailing out")
    }
}