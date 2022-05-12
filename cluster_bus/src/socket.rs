
use alloc::boxed::Box;
use alloc::fmt::Debug;
use alloc::sync::Arc;

use alloc::vec::*;
use bitflags::*;
use core::cmp::min;
use core::mem::size_of;
use core::slice;
use net_bridge::Lock::{Lock};
use net_bridge::NET_DRIVERS;
use smoltcp::socket::*;
use smoltcp::wire::*;
use super::*;
use lazy_static::*;
use smoltcp::socket::*;
use core::fmt::Write;
#[derive(Clone, Debug)]
pub struct LinkLevelEndpoint {
    pub interface_index: usize,
}

impl LinkLevelEndpoint {
    pub fn new(ifindex: usize) -> Self {
        LinkLevelEndpoint {
            interface_index: ifindex,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetlinkEndpoint {
    pub port_id: u32,
    pub multicast_groups_mask: u32,
}

impl NetlinkEndpoint {
    pub fn new(port_id: u32, multicast_groups_mask: u32) -> Self {
        NetlinkEndpoint {
            port_id,
            multicast_groups_mask,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Endpoint {
    Ip(IpEndpoint),
    LinkLevel(LinkLevelEndpoint),
    Netlink(NetlinkEndpoint),
}
pub type SysResult = Result<usize, SysError>;
pub trait Socket: Send + Sync + Debug {
    fn read(&self, data: &mut [u8]) -> (SysResult, Endpoint);
    fn write(&self, data: &[u8], sendto_endpoint: Option<Endpoint>) -> SysResult;
    fn poll(&self) -> (bool, bool, bool); // (in, out, err)
    fn connect(&mut self, endpoint: Endpoint) -> SysResult;
    fn bind(&mut self, _endpoint: Endpoint) -> SysResult {
        Err(SysError::EINVAL)
    }
    fn listen(&mut self) -> SysResult {
        Err(SysError::EINVAL)
    }
    fn shutdown(&self) -> SysResult {
        Err(SysError::EINVAL)
    }
    fn accept(&mut self) -> Result<(Box<dyn Socket>, Endpoint), SysError> {
        Err(SysError::EINVAL)
    }
    fn endpoint(&self) -> Option<Endpoint> {
        None
    }
    fn remote_endpoint(&self) -> Option<Endpoint> {
        None
    }
    fn setsockopt(&mut self, _level: usize, _opt: usize, _data: &[u8]) -> SysResult {
        warn!("setsockopt is unimplemented");
        Ok(0)
    }
    fn ioctl(&mut self, _request: usize, _arg1: usize, _arg2: usize, _arg3: usize) -> SysResult {
        warn!("ioctl is unimplemented for this socket");
        Ok(0)
    }
    fn box_clone(&self) -> Box<dyn Socket>;
}
impl Clone for Box<dyn Socket> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

lazy_static! {
    /// Global SocketSet in smoltcp.
    ///
    /// Because smoltcp is a single thread network stack,
    /// every socket operation needs to lock this.
    pub static ref SOCKETS: Lock<SocketSet<'static, 'static, 'static>> =
    Lock::new(SocketSet::new(vec![]));
}


#[derive(Debug, Clone)]
pub struct RawSocketState {
    handle: GlobalSocketHandle,
    header_included: bool,
}

/*#[derive(Debug, Clone)]
pub struct PacketSocketState {
    // no state, only ethernet egress
}

#[derive(Debug, Clone)]
pub struct NetlinkSocketState {
    data: Arc<Lock<Vec<Vec<u8>>>>,
}*/



#[derive(Debug)]
struct GlobalSocketHandle(SocketHandle);

impl Clone for GlobalSocketHandle {
    fn clone(&self) -> Self {
        SOCKETS.lock().retain(self.0);
        Self(self.0)
    }
}
impl Drop for GlobalSocketHandle {
    fn drop(&mut self) {
        let mut sockets = SOCKETS.lock();
        sockets.release(self.0);
        sockets.prune();

        // send FIN immediately when applicable
        drop(sockets);
        poll_ifaces();
    }
}

fn poll_ifaces() {
    for iface in NET_DRIVERS.read().iter() {
        iface.poll();
    }
}

const RAW_METADATA_BUF: usize = 1024;
const RAW_SENDBUF: usize = 64 * 1024; // 64K
const RAW_RECVBUF: usize = 64 * 1024; // 64K

impl RawSocketState {
    pub fn new(protocol: u8) -> Self {
        let rx_buffer = RawSocketBuffer::new(
            vec![RawPacketMetadata::EMPTY; RAW_METADATA_BUF],
            vec![0; RAW_RECVBUF],
        );
        let tx_buffer = RawSocketBuffer::new(
            vec![RawPacketMetadata::EMPTY; RAW_METADATA_BUF],
            vec![0; RAW_SENDBUF],
        );
        let socket = RawSocket::new(
            IpVersion::Ipv4,
            IpProtocol::from(protocol),
            rx_buffer,
            tx_buffer,
        );
        let handle = GlobalSocketHandle(SOCKETS.lock().add(socket));

        RawSocketState {
            handle,
            header_included: false,
        }
    }
}

impl Socket for RawSocketState {
    fn read(&self, data: &mut [u8]) -> (SysResult, Endpoint) {
        loop {
            let mut sockets = SOCKETS.lock();
            let mut socket = sockets.get::<RawSocket>(self.handle.0);

            if let Ok(size) = socket.recv_slice(data) {
                let packet = Ipv4Packet::new_unchecked(data);

                return (
                    Ok(size),
                    Endpoint::Ip(IpEndpoint {
                        addr: IpAddress::Ipv4(packet.src_addr()),
                        port: 0,
                    }),
                );
            }

            drop(socket);
            //SOCKET_ACTIVITY.wait(sockets);
        }
    }

    fn write(&self, data: &[u8], sendto_endpoint: Option<Endpoint>) -> SysResult {
        if self.header_included {
            let mut sockets = SOCKETS.lock();
            let mut socket = sockets.get::<RawSocket>(self.handle.0);

            match socket.send_slice(&data) {
                Ok(()) => Ok(data.len()),
                Err(_) => Err(SysError::ENOBUFS),
            }
        } else {
            if let Some(Endpoint::Ip(endpoint)) = sendto_endpoint {
                // temporary solution
                let iface = &*(NET_DRIVERS.read()[0]);
                let v4_src = iface.ipv4_address().unwrap();
                let mut sockets = SOCKETS.lock();
                let mut socket = sockets.get::<RawSocket>(self.handle.0);

                if let IpAddress::Ipv4(v4_dst) = endpoint.addr {
                    let len = data.len();
                    // using 20-byte IPv4 header
                    let mut buffer = vec![0u8; len + 20];
                    let mut packet = Ipv4Packet::new_unchecked(&mut buffer);
                    packet.set_version(4);
                    packet.set_header_len(20);
                    packet.set_total_len((20 + len) as u16);
                    packet.set_protocol(socket.ip_protocol().into());
                    packet.set_src_addr(v4_src);
                    packet.set_dst_addr(v4_dst);
                    let payload = packet.payload_mut();
                    payload.copy_from_slice(data);
                    packet.fill_checksum();

                    socket.send_slice(&buffer).unwrap();

                    // avoid deadlock
                    drop(socket);
                    drop(sockets);
                    iface.poll();

                    Ok(len)
                } else {
                    unimplemented!("ip type")
                }
            } else {
                Err(SysError::ENOTCONN)
            }
        }
    }

    fn poll(&self) -> (bool, bool, bool) {
        unimplemented!()
    }

    fn connect(&mut self, _endpoint: Endpoint) -> SysResult {
        unimplemented!()
    }

    fn box_clone(&self) -> Box<dyn Socket> {
        Box::new(self.clone())
    }

    fn setsockopt(&mut self, level: usize, opt: usize, data: &[u8]) -> SysResult {
        match (level, opt) {
            (IPPROTO_IP, IP_HDRINCL) => {
                if let Some(arg) = data.first() {
                    self.header_included = *arg > 0;
                    debug!("hdrincl set to {}", self.header_included);
                }
            }
            _ => {}
        }
        Ok(0)
    }
}


pub fn test(){
    let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 64]);
    let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 128]);
    let udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);

    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

    let tcp2_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp2_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp2_socket = TcpSocket::new(tcp2_rx_buffer, tcp2_tx_buffer);

    let mut sockets = SOCKETS.lock();
    let udp_handle = sockets.add(udp_socket);
    let tcp_handle = sockets.add(tcp_socket);
    let tcp2_handle = sockets.add(tcp2_socket);
    drop(sockets);

    loop {
        {
            let mut sockets = SOCKETS.lock();

        


            // simple tcp server that just eats everything
            {
                let mut socket = sockets.get::<TcpSocket>(tcp2_handle);
                if !socket.is_open() {
                    socket.listen(2222).unwrap();
                }

                if socket.can_recv() {
                    let mut data = [0u8; 2048];
                    let _size = socket.recv_slice(&mut data).unwrap();
                    println!("recv: {:?}", &data[.._size]);
                }
            }
        }

        //thread::yield_now();
    }
}

/*impl PacketSocketState {
    pub fn new() -> Self {
        PacketSocketState {}
    }
}

impl Socket for PacketSocketState {
    fn read(&self, _data: &mut [u8]) -> (SysResult, Endpoint) {
        unimplemented!()
    }

    fn write(&self, data: &[u8], sendto_endpoint: Option<Endpoint>) -> SysResult {
        if let Some(Endpoint::LinkLevel(endpoint)) = sendto_endpoint {
            let ifaces = NET_DRIVERS.read();
            match ifaces[endpoint.interface_index].send(data) {
                Some(len) => Ok(len),
                None => Err(SysError::ENOBUFS),
            }
        } else {
            Err(SysError::ENOTCONN)
        }
    }

    fn poll(&self) -> (bool, bool, bool) {
        unimplemented!()
    }

    fn connect(&mut self, _endpoint: Endpoint) -> SysResult {
        unimplemented!()
    }

    fn box_clone(&self) -> Box<dyn Socket> {
        Box::new(self.clone())
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct NetlinkMessageHeader {
    nlmsg_len: u32,                   // length of message including header
    nlmsg_type: u16,                  // message content
    nlmsg_flags: NetlinkMessageFlags, // additional flags
    nlmsg_seq: u32,                   // sequence number
    nlmsg_pid: u32,                   // sending process port id
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct IfaceInfoMsg {
    ifi_family: u16,
    ifi_type: u16,
    ifi_index: u32,
    ifi_flags: u32,
    ifi_change: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct IfaceAddrMsg {
    ifa_family: u8,
    ifa_prefixlen: u8,
    ifa_flags: u8,
    ifa_scope: u8,
    ifa_index: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct RouteAttr {
    rta_len: u16,
    rta_type: u16,
}

bitflags! {
    struct NetlinkMessageFlags : u16 {
        const REQUEST = 0x01;
        const MULTI = 0x02;
        const ACK = 0x04;
        const ECHO = 0x08;
        const DUMP_INTR = 0x10;
        const DUMP_FILTERED = 0x20;
        // GET request
        const ROOT = 0x100;
        const MATCH = 0x200;
        const ATOMIC = 0x400;
        const DUMP = 0x100 | 0x200;
        // NEW request
        const REPLACE = 0x100;
        const EXCL = 0x200;
        const CREATE = 0x400;
        const APPEND = 0x800;
        // DELETE request
        const NONREC = 0x100;
        // ACK message
        const CAPPED = 0x100;
        const ACK_TLVS = 0x200;
    }
}

enum_with_unknown! {
    /// Netlink message types
    pub doc enum NetlinkMessageType(u16) {
        /// Nothing
        Noop = 1,
        /// Error
        Error = 2,
        /// End of a dump
        Done = 3,
        /// Data lost
        Overrun = 4,
        /// New link
        NewLink = 16,
        /// Delete link
        DelLink = 17,
        /// Get link
        GetLink = 18,
        /// Set link
        SetLink = 19,
        /// New addr
        NewAddr = 20,
        /// Delete addr
        DelAddr = 21,
        /// Get addr
        GetAddr = 22,
    }
}

enum_with_unknown! {
    /// Route Attr Types
    pub doc enum RouteAttrTypes(u16) {
        /// Unspecified
        Unspecified = 0,
        /// MAC Address
        Address = 1,
        /// Broadcast
        Broadcast = 2,
        /// Interface name
        Ifname = 3,
        /// MTU
        MTU = 4,
        /// Link
        Link = 5,
    }
}

enum_with_unknown! {
    /// Address families
    pub doc enum AddressFamily(u16) {
        /// Unspecified
        Unspecified = 0,
        /// Unix domain sockets
        Unix = 1,
        /// Internet IP Protocol
        Internet = 2,
        /// Netlink
        Netlink = 16,
        /// Packet family
        Packet = 17,
    }
}

impl NetlinkSocketState {
    pub fn new() -> Self {
        NetlinkSocketState {
            data: Arc::new(Lock::new(Vec::new())),
        }
    }
}

trait VecExt {
    fn align4(&mut self);
    fn push_ext<T: Sized>(&mut self, data: T);
    fn set_ext<T: Sized>(&mut self, offset: usize, data: T);
}

impl VecExt for Vec<u8> {
    fn align4(&mut self) {
        let len = (self.len() + 3) & !3;
        if len > self.len() {
            self.resize(len, 0);
        }
    }

    fn push_ext<T: Sized>(&mut self, data: T) {
        let bytes =
            unsafe { slice::from_raw_parts(&data as *const T as *const u8, size_of::<T>()) };
        for byte in bytes {
            self.push(*byte);
        }
    }

    fn set_ext<T: Sized>(&mut self, offset: usize, data: T) {
        if self.len() < offset + size_of::<T>() {
            self.resize(offset + size_of::<T>(), 0);
        }
        let bytes =
            unsafe { slice::from_raw_parts(&data as *const T as *const u8, size_of::<T>()) };
        for i in 0..bytes.len() {
            self[offset + i] = bytes[i];
        }
    }
}

impl Socket for NetlinkSocketState {
    fn read(&self, data: &mut [u8]) -> (SysResult, Endpoint) {
        let mut buffer = self.data.lock();
        if buffer.len() > 0 {
            let msg = buffer.remove(0);
            let len = min(msg.len(), data.len());
            data[..len].copy_from_slice(&msg[..len]);
            (
                Ok(len),
                Endpoint::Netlink(NetlinkEndpoint {
                    port_id: 0,
                    multicast_groups_mask: 0,
                }),
            )
        } else {
            (
                Ok(0),
                Endpoint::Netlink(NetlinkEndpoint {
                    port_id: 0,
                    multicast_groups_mask: 0,
                }),
            )
        }
    }

    fn write(&self, data: &[u8], _sendto_endpoint: Option<Endpoint>) -> SysResult {
        if data.len() < size_of::<NetlinkMessageHeader>() {
            return Err(SysError::EINVAL);
        }
        let header = unsafe { &*(data.as_ptr() as *const NetlinkMessageHeader) };
        if header.nlmsg_len as usize > data.len() {
            return Err(SysError::EINVAL);
        }
        let message_type = NetlinkMessageType::from(header.nlmsg_type);
        debug!("type: {:?}", message_type);
        let mut buffer = self.data.lock();
        buffer.clear();
        match message_type {
            NetlinkMessageType::GetLink => {
                let ifaces = NET_DRIVERS.read();
                for i in 0..ifaces.len() {
                    let mut msg = Vec::new();
                    let new_header = NetlinkMessageHeader {
                        nlmsg_len: 0, // to be determined later
                        nlmsg_type: NetlinkMessageType::NewLink.into(),
                        nlmsg_flags: NetlinkMessageFlags::MULTI,
                        nlmsg_seq: header.nlmsg_seq,
                        nlmsg_pid: header.nlmsg_pid,
                    };
                    msg.push_ext(new_header);

                    let if_info = IfaceInfoMsg {
                        ifi_family: AddressFamily::Unspecified.into(),
                        ifi_type: 0,
                        ifi_index: i as u32,
                        ifi_flags: 0,
                        ifi_change: 0,
                    };
                    msg.align4();
                    msg.push_ext(if_info);

                    let mut attrs = Vec::new();

                    let mac_addr = ifaces[i].get_mac();
                    let attr = RouteAttr {
                        rta_len: (mac_addr.as_bytes().len() + size_of::<RouteAttr>()) as u16,
                        rta_type: RouteAttrTypes::Address.into(),
                    };
                    attrs.align4();
                    attrs.push_ext(attr);
                    for byte in mac_addr.as_bytes() {
                        attrs.push(*byte);
                    }

                    let ifname = ifaces[i].get_ifname();
                    let attr = RouteAttr {
                        rta_len: (ifname.as_bytes().len() + size_of::<RouteAttr>()) as u16,
                        rta_type: RouteAttrTypes::Ifname.into(),
                    };
                    attrs.align4();
                    attrs.push_ext(attr);
                    for byte in ifname.as_bytes() {
                        attrs.push(*byte);
                    }

                    msg.align4();
                    msg.append(&mut attrs);

                    msg.align4();
                    msg.set_ext(0, msg.len() as u32);

                    buffer.push(msg);
                }
            }
            NetlinkMessageType::GetAddr => {
                let ifaces = NET_DRIVERS.read();
                for i in 0..ifaces.len() {
                    let ip_addrs = ifaces[i].get_ip_addresses();
                    for j in 0..ip_addrs.len() {
                        let mut msg = Vec::new();
                        let new_header = NetlinkMessageHeader {
                            nlmsg_len: 0, // to be determined later
                            nlmsg_type: NetlinkMessageType::NewAddr.into(),
                            nlmsg_flags: NetlinkMessageFlags::MULTI,
                            nlmsg_seq: header.nlmsg_seq,
                            nlmsg_pid: header.nlmsg_pid,
                        };
                        msg.push_ext(new_header);

                        let family: u16 = AddressFamily::Internet.into();
                        let if_addr = IfaceAddrMsg {
                            ifa_family: family as u8,
                            ifa_prefixlen: ip_addrs[j].prefix_len(),
                            ifa_flags: 0,
                            ifa_scope: 0,
                            ifa_index: i as u32,
                        };
                        msg.align4();
                        msg.push_ext(if_addr);

                        let mut attrs = Vec::new();

                        let ip_addr = ip_addrs[j].address();
                        let attr = RouteAttr {
                            rta_len: (ip_addr.as_bytes().len() + size_of::<RouteAttr>()) as u16,
                            rta_type: RouteAttrTypes::Address.into(),
                        };
                        attrs.align4();
                        attrs.push_ext(attr);
                        for byte in ip_addr.as_bytes() {
                            attrs.push(*byte);
                        }

                        msg.align4();
                        msg.append(&mut attrs);

                        msg.align4();
                        msg.set_ext(0, msg.len() as u32);

                        buffer.push(msg);
                    }
                }
            }
            _ => {}
        }
        let mut msg = Vec::new();
        let new_header = NetlinkMessageHeader {
            nlmsg_len: 0, // to be determined later
            nlmsg_type: NetlinkMessageType::Done.into(),
            nlmsg_flags: NetlinkMessageFlags::MULTI,
            nlmsg_seq: header.nlmsg_seq,
            nlmsg_pid: header.nlmsg_pid,
        };
        msg.push_ext(new_header);
        msg.align4();
        msg.set_ext(0, msg.len() as u32);
        buffer.push(msg);
        Ok(data.len())
    }

    fn poll(&self) -> (bool, bool, bool) {
        unimplemented!()
    }

    fn connect(&mut self, _endpoint: Endpoint) -> SysResult {
        unimplemented!()
    }

    fn bind(&mut self, _endpoint: Endpoint) -> SysResult {
        Ok(0)
    }

    fn box_clone(&self) -> Box<dyn Socket> {
        Box::new(self.clone())
    }
}

fn get_ephemeral_port() -> u16 {
    // TODO selects non-conflict high port
    static mut EPHEMERAL_PORT: u16 = 0;
    unsafe {
        if EPHEMERAL_PORT == 0 {
            EPHEMERAL_PORT = (49152 + rand() % (65536 - 49152)) as u16;
        }
        if EPHEMERAL_PORT == 65535 {
            EPHEMERAL_PORT = 49152;
        } else {
            EPHEMERAL_PORT = EPHEMERAL_PORT + 1;
        }
        EPHEMERAL_PORT
    }
}



pub fn rand() -> u64 {
    return 0;
}
*/
