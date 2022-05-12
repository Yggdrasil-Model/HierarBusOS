/*use isomorphic_drivers::provider;
use memory_manager::{frame_alloc_contiguous,frame_dealloc,PhysAddr};
pub const PAGE_SIZE: usize = 1 << 12;
pub struct Provider;

impl provider::Provider for Provider{
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_dma(size: usize) -> (usize, usize) {
        let paddr = virtio_dma_alloc((size -1 + PAGE_SIZE) / PAGE_SIZE).into();
        let vaddr = paddr;
        (vaddr, paddr)
    }

    fn dealloc_dma(vaddr: usize, size: usize) {
        let paddr = vaddr;
        for i in paddr..paddr+size {
            frame_dealloc(i.into());
        }
    }
}

#[no_mangle]
extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let paddr: PhysAddr = frame_alloc_contiguous(pages).unwrap().into();
    trace!("alloc DMA: paddr={:#x}, pages={}", paddr.0, pages);
    paddr
}*/


//use core::sync::atomic::*;
use lazy_static::lazy_static;
use memory_manager::{frame_alloc_contiguous,frame_dealloc,PhysAddr,VirtAddr};
//extern "C" {
   // fn ekernel();
////}

//lazy_static! {
    //static ref DMA_PADDR: AtomicUsize = AtomicUsize::new(ekernel as usize);
////}

#[no_mangle]
extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    //let paddr = DMA_PADDR.fetch_add(0x1000 * pages, Ordering::SeqCst);
    let paddr: PhysAddr = frame_alloc_contiguous(pages).unwrap().into();
    println!("alloc DMA: paddr={:#x}, pages={}", paddr.0, pages);
    paddr
}
pub const PAGE_SIZE: usize = 0x1000;
#[no_mangle]
extern "C" fn virtio_dma_dealloc(paddr: PhysAddr, pages: usize) -> i32 {
    let end = paddr.0+pages*PAGE_SIZE;
    for i in paddr.0..end{
        frame_dealloc(i.into());
    }
    println!("dealloc DMA: paddr={:#x}, pages={}", paddr.0, pages);
    0
}

#[no_mangle]
extern "C" fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    paddr.0.into()
}

#[no_mangle]
extern "C" fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    vaddr.0.into()
}

//type VirtAddr = usize;
//type PhysAddr = usize;

