/*use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use smoltcp::iface::*;
use smoltcp::phy::{self, DeviceCapabilities};
use smoltcp::time::Instant;
use smoltcp::wire::*;
use smoltcp::Result;

use isomorphic_drivers::net::ethernet::intel::e1000::E1000;
use isomorphic_drivers::net::ethernet::structs::EthernetAddress as DriverEthernetAddress;

use super::*;
use super::Lock::{Lock,LockGuard};
#[derive(Clone)]
pub struct E1000Driver(Arc<Lock<E1000<Provider>>>);

pub struct E1000Interface {
    iface: Lock<EthernetInterface<'static, 'static, 'static, E1000Driver>>,
    driver: E1000Driver,
    name: String,
    irq: Option<usize>,
}

impl Driver for E1000Interface {
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool {
        if irq.is_some() && self.irq.is_some() && irq != self.irq {
            // not ours, skip it
            return false;
        }

        let data = self.driver.0.lock().handle_interrupt();

        if data {
            let timestamp = Instant::from_millis(crate::trap::uptime_msec() as i64);
            let mut sockets = SOCKETS.lock();
            match self.iface.lock().poll(&mut sockets, timestamp) {
                Ok(_) => {
                    SOCKET_ACTIVITY.notify_all();
                }
                Err(err) => {
                    debug!("poll got err {}", err);
                }
            }
        }
        return data;
    }
    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }

    fn get_id(&self) -> String {
        String::from("e1000")
    }

    fn as_net(&self) -> Option<&dyn NetDriver> {
        Some(self)
    }  
}

impl NetDriver for E1000Interface {
    fn get_mac(&self) -> EthernetAddress {
        self.iface.lock().ethernet_addr()
    }

    fn get_ifname(&self) -> String {
        self.name.clone()
    }

    // get ip addresses
    fn get_ip_addresses(&self) -> Vec<IpCidr> {
        Vec::from(self.iface.lock().ip_addrs())
    }

    fn ipv4_address(&self) -> Option<Ipv4Address> {
        self.iface.lock().ipv4_address()
    }

    fn poll(&self) {
        let timestamp = Instant::from_millis(crate::trap::uptime_msec() as i64);
        let mut sockets = SOCKETS.lock();
        match self.iface.lock().poll(&mut sockets, timestamp) {
            Ok(_) => {
                SOCKET_ACTIVITY.notify_all();
            }
            Err(err) => {
                debug!("poll got err {}", err);
            }
        }
    }

    fn send(&self, data: &[u8]) -> Option<usize> {
        use smoltcp::phy::TxToken;
        let token = E1000TxToken(self.driver.clone());
        if token
            .consume(Instant::from_millis(0), data.len(), |buffer| {
                buffer.copy_from_slice(&data);
                Ok(())
            })
            .is_ok()
        {
            Some(data.len())
        } else {
            None
        }
    }

    fn get_arp(&self, ip: IpAddress) -> Option<EthernetAddress> {
        let iface = self.iface.lock();
        let cache = iface.neighbor_cache();
        cache.lookup_pure(&ip, Instant::from_millis(0))
    }
}


pub struct E1000RxToken(Vec<u8>);
pub struct E1000TxToken(E1000Driver);

impl phy::Device<'_> for E1000Driver {
    type RxToken = E1000RxToken;
    type TxToken = E1000TxToken;

    fn receive(&mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        self.0
            .lock()
            .receive()
            .map(|vec| (E1000RxToken(vec), E1000TxToken(self.clone())))
    }

    fn transmit(&mut self) -> Option<Self::TxToken> {
        if self.0.lock().can_send() {
            Some(E1000TxToken(self.clone()))
        } else {
            None
        }
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1536;
        caps.max_burst_size = Some(64);
        caps
    }
}

impl phy::RxToken for E1000RxToken {
    fn consume<R, F>(mut self, _timestamp: Instant, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        f(&mut self.0)
    }
}
pub const PAGE_SIZE: usize = 1 << 12;
impl phy::TxToken for E1000TxToken {
    fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        let mut buffer = [0u8; PAGE_SIZE];
        let result = f(&mut buffer[..len]);

        let mut driver = (self.0).0.lock();
        driver.send(&buffer);

        result
    }
}


pub fn init(name: String, irq: Option<usize>, header: usize, size: usize, index: usize) {
    info!("Probing e1000 {}", name);

    // randomly generated
    let mac: [u8; 6] = [0x54, 0x51, 0x9F, 0x71, 0xC0, index as u8];

    let e1000 = E1000::new(header, size, DriverEthernetAddress::from_bytes(&mac));

    let net_driver = E1000Driver(Arc::new(Lock::new(e1000)));

    let ethernet_addr = EthernetAddress::from_bytes(&mac);
    let ip_addrs = [IpCidr::new(IpAddress::v4(10, 0, index as u8, 2), 24)];
    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    let iface = EthernetInterfaceBuilder::new(net_driver.clone())
        .ethernet_addr(ethernet_addr)
        .ip_addrs(ip_addrs)
        .neighbor_cache(neighbor_cache)
        .finalize();

    info!("e1000 interface {} up with addr 10.0.{}.2/24", name, index);
    let e1000_iface = E1000Interface {
        iface: Lock::new(iface),
        driver: net_driver.clone(),
        name,
        irq,
    };

    let driver = Arc::new(e1000_iface);
    DRIVERS.write().push(driver.clone());
    IRQ_MANAGER.write().register_opt(irq, driver.clone());
    NET_DRIVERS.write().push(driver);
}*/