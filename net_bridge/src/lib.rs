#![no_std]
extern crate alloc;


use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use lazy_static::lazy_static;
use spin::RwLock;
use device_tree::{DeviceTree, Node};
use device_tree::util::SliceRead;
use core::mem::size_of;
use memory_manager::{KERNEL_SPACE,MapType,MapPermission,MapArea};

//pub mod e1000;
pub mod virtio_net;
//pub mod provider;
pub mod Lock;
pub mod provider;
pub mod head;
pub mod net;
pub mod queue;
extern crate dynamic_malloc_support;
pub use head::*;
pub use net::*;
pub use queue::*;
pub mod hal;
pub use hal::*;
#[macro_use]
extern crate console_support;

/*#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Invalid = 0,
    Net,
    Gpu,
    Input,
    Block,
    Rtc,
    Serial,
    Intc,
}*/
pub trait Driver: Send + Sync {
    // if interrupt belongs to this driver, handle it and return true
    // return false otherwise
    // irq number is provided when available
    // driver should skip handling when irq number is mismatched
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool;

    // return the correspondent device type, see DeviceType
    fn device_type(&self) -> DeviceType;

    // get unique identifier for this device
    // should be different for each instance
    fn get_id(&self) -> String;

    // trait casting
    fn as_net(&self) -> Option<&dyn NetDriver> {
        None
    }
}

pub trait NetDriver: Driver {
    // get mac address for this device
    fn get_mac(&self) -> EthernetAddress {
        unimplemented!("not a net driver")
    }

    // get interface name for this device
    fn get_ifname(&self) -> String {
        unimplemented!("not a net driver")
    }

    // get ip addresses
    fn get_ip_addresses(&self) -> Vec<IpCidr> {
        unimplemented!("not a net driver")
    }

    // get ipv4 address
    fn ipv4_address(&self) -> Option<Ipv4Address> {
        unimplemented!("not a net driver")
    }

    // manually trigger a poll, use it after sending packets
    fn poll(&self) {
        unimplemented!("not a net driver")
    }

    // send an ethernet frame, only use it when necessary
    fn send(&self, _data: &[u8]) -> Option<usize> {
        unimplemented!("not a net driver")
    }

    // get mac address from ip address in arp table
    fn get_arp(&self, _ip: IpAddress) -> Option<EthernetAddress> {
        unimplemented!("not a net driver")
    }
}
lazy_static! {
    // NOTE: RwLock only write when initializing drivers
    pub static ref DRIVERS: RwLock<Vec<Arc<dyn Driver>>> = RwLock::new(Vec::new());
    pub static ref NET_DRIVERS: RwLock<Vec<Arc<dyn NetDriver>>> = RwLock::new(Vec::new());
}

struct DtbHeader {
    magic: u32,
    size: u32,
}

pub fn init(dtb: usize) {
    let header = unsafe { &*(dtb as *const DtbHeader) };
    let magic = u32::from_be(header.magic);
    const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;
    if magic == DEVICE_TREE_MAGIC {
        println!("init device!");
        let size = u32::from_be(header.size);
        let dtb_data = unsafe { core::slice::from_raw_parts(dtb as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(dtb_data) {
            // find interrupt controller first
            walk_dt_node(&dt.root);
        }
    }
    println!("finish device init");
}

fn walk_dt_node(dt: &Node) {
    if let Ok(compatible) = dt.prop_str("compatible") {
        if compatible == "virtio,mmio" {
            virtio_probe(dt);
        }
    }
    for child in dt.children.iter() {
        walk_dt_node(child);
    }
}

fn virtio_probe(node: &Node) {
    if let Some(reg) = node.prop_raw("reg") {
        let paddr = reg.as_slice().read_be_u64(0).unwrap();
        let size = reg.as_slice().read_be_u64(8).unwrap();
        let vaddr = paddr;
        println!("walk dt addr={:#x}, size={:#x}", paddr, size);
        KERNEL_SPACE.lock().push(MapArea::new(
            (paddr as usize).into(),
            ((paddr+size) as usize).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ), None);
        let header = unsafe { &mut *(vaddr as *mut VirtIOHeader) };
        println!(
            "Detected virtio device with vendor id {:#X}",
            header.vendor_id()
        );
        println!("Device tree node {:?}", node);
        match header.device_type(){
            DeviceType::Network => {virtio_net::initnet(header)},
            t => println!(" virtio device: {:?}",t),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The buffer is too small.
    BufferTooSmall,
    /// The device is not ready.
    NotReady,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
}

unsafe trait AsBuf: Sized {
    fn as_buf(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, size_of::<Self>()) }
    }
    fn as_buf_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, size_of::<Self>()) }
    }
}

pub type Result<T = ()> = core::result::Result<T, Error>;
pub const PAGE_SIZE: usize = 0x1000;

pub fn align_up(size: usize) -> usize {
    (size + PAGE_SIZE) & !(PAGE_SIZE - 1)
}