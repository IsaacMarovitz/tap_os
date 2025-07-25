use crate::memory;
use good_memory_allocator::SpinLockedAllocator;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::VirtAddr;

pub const HEAP_START: u64 = 0x4444_4444_0000;

// Referenced from https://github.com/vinc/moros/blob/trunk/src/sys/allocator.rs

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // Expand this later
    let total_memory = memory::memory_size();
    let heap_size = (total_memory / 16).min(16 * 1024 * 1024); // Cap at 16MB
    let heap_start = VirtAddr::new(HEAP_START);

    log::info!("[ALLOC]: Heap initialization:");
    log::info!("            Total memory: {} MB", total_memory / (1024 * 1024));
    log::info!("            Requested heap size: {} MB", heap_size / (1024 * 1024));
    log::info!("            Heap pages needed: {}", heap_size / 4096);

    let pages = {
        let heap_end = heap_start + heap_size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    for page in pages {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    unsafe {
        ALLOCATOR.init(heap_start.as_u64() as usize, heap_size as usize);
    }

    Ok(())
}
