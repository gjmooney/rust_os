use x86_64::{structures::paging::PageTable, VirtAddr};

// Get a mutable reference to the active level 4 table
// Unsafe because caller must guarantee complete physical memory is mapped
// at the passed offset
// must only call function once to avoid aliasing &mut references
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
