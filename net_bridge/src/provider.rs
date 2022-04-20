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

