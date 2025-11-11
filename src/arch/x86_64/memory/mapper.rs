/// Virtual memory mapper for x86_64
/// 
/// This module implements the core virtual memory mapping functionality,
/// allowing virtual addresses to be mapped to physical frames through
/// the 4-level page table hierarchy.

use super::paging::{
    Page, PageTable, PageTableEntry, PageTableFlags, PageTableLevel,
    PhysAddr, PhysFrame, VirtAddr,
};
use super::frame_alloc::{FrameAllocator, FrameAllocError};

/// Result type for mapping operations
pub type MapResult<T> = Result<T, MapError>;

/// Errors that can occur during mapping operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapError {
    /// The page is already mapped
    PageAlreadyMapped,
    /// Frame allocation failed
    FrameAllocationFailed,
    /// The parent entry is a huge page
    ParentEntryHugePage,
    /// Invalid flags for the operation
    InvalidFlags,
}

impl From<FrameAllocError> for MapError {
    fn from(_: FrameAllocError) -> Self {
        MapError::FrameAllocationFailed
    }
}

/// A mapper for managing virtual memory mappings
/// 
/// This type provides methods to map and unmap virtual pages to physical frames
/// by traversing and modifying the page table hierarchy.
pub struct Mapper<'a, A: FrameAllocator> {
    pml4: &'a mut PageTable,
    allocator: A,
}

impl<'a, A: FrameAllocator> Mapper<'a, A> {
    /// Create a new mapper with the given PML4 table and frame allocator
    /// 
    /// # Safety
    /// The caller must ensure that:
    /// - The PML4 table is valid and properly initialized
    /// - The PML4 table is the active page table or will be loaded
    pub unsafe fn new(pml4: &'a mut PageTable, allocator: A) -> Self {
        Self { pml4, allocator }
    }

    /// Map a virtual page to a physical frame with the given flags
    /// 
    /// This function will:
    /// 1. Traverse the page table hierarchy (creating tables as needed)
    /// 2. Set the final page table entry to point to the physical frame
    /// 3. Apply the given flags
    /// 
    /// Returns an error if the page is already mapped or allocation fails.
    pub fn map_to(
        &mut self,
        page: Page,
        frame: PhysFrame,
        flags: PageTableFlags,
    ) -> MapResult<()> {
        // Ensure the PRESENT flag is set
        let flags = flags.union(PageTableFlags::PRESENT);
        
        // Get the page table entry for this page, creating tables as needed
        let pt_entry = self.create_page_table_entry(page)?;
        
        // Check if the page is already mapped
        if !pt_entry.is_unused() {
            return Err(MapError::PageAlreadyMapped);
        }
        
        // Map the page to the frame
        pt_entry.set_addr(frame.start_address(), flags);
        
        Ok(())
    }

    /// Map a virtual page to a physical frame, allocating a frame if needed
    /// 
    /// This is a convenience function that allocates a physical frame
    /// and then maps the page to it.
    pub fn map(
        &mut self,
        page: Page,
        flags: PageTableFlags,
    ) -> MapResult<PhysFrame> {
        // Allocate a physical frame
        let frame = self.allocator.allocate_frame()?;
        
        // Map the page to the frame
        match self.map_to(page, frame, flags) {
            Ok(()) => Ok(frame),
            Err(e) => {
                // Deallocate the frame on error
                unsafe { self.allocator.deallocate_frame(frame); }
                Err(e)
            }
        }
    }

    /// Unmap a virtual page
    /// 
    /// This function removes the mapping for the given page but does NOT
    /// deallocate the physical frame. The caller is responsible for
    /// deallocating the frame if needed.
    /// 
    /// Returns the physical frame that was mapped to the page.
    pub fn unmap(&mut self, page: Page) -> MapResult<PhysFrame> {
        // Get the page table entry for this page
        let pt_entry = self.page_table_entry(page)?;
        
        // Check if the page is mapped
        if pt_entry.is_unused() {
            return Err(MapError::PageAlreadyMapped);
        }
        
        // Get the frame before clearing the entry
        let frame = pt_entry.frame();
        
        // Clear the entry
        pt_entry.set_unused();
        
        // Flush the TLB for this page
        flush_page(page.start_address());
        
        Ok(frame)
    }

    /// Translate a virtual address to a physical address
    /// 
    /// Returns None if the virtual address is not mapped.
    pub fn translate(&self, addr: VirtAddr) -> Option<PhysAddr> {
        let page = Page::containing_address(addr);
        let offset = addr.page_offset();
        
        // Get the page table entry for this page
        match self.page_table_entry_readonly(page) {
            Ok(entry) => {
                if entry.is_unused() {
                    None
                } else {
                    let frame_addr = entry.addr().as_u64();
                    Some(PhysAddr::new(frame_addr + offset as u64))
                }
            }
            Err(_) => None,
        }
    }

    /// Get the page table entry for a virtual page (read-only)
    /// 
    /// This traverses the page table hierarchy without creating tables.
    fn page_table_entry_readonly(&self, page: Page) -> MapResult<&PageTableEntry> {
        let addr = page.start_address();
        
        // Start at PML4
        let mut table = self.pml4 as *const PageTable;
        
        // Traverse through levels 4, 3, and 2
        for level in [PageTableLevel::Four, PageTableLevel::Three, PageTableLevel::Two] {
            let index = addr.page_table_index(level);
            let table_ref = unsafe { &*table };
            let entry = &table_ref[index];
            
            if !entry.flags().contains(PageTableFlags::PRESENT) {
                return Err(MapError::PageAlreadyMapped);
            }
            
            if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
                return Err(MapError::ParentEntryHugePage);
            }
            
            table = entry.addr().as_u64() as *const PageTable;
        }
        
        // Get the entry from the final page table (level 1)
        let index = addr.page_table_index(PageTableLevel::One);
        let table_ref = unsafe { &*table };
        Ok(&table_ref[index])
    }

    /// Get a mutable reference to the page table entry for a virtual page
    /// 
    /// This traverses the page table hierarchy without creating tables.
    fn page_table_entry(&mut self, page: Page) -> MapResult<&mut PageTableEntry> {
        let addr = page.start_address();
        
        // Start at PML4
        let mut table = self.pml4 as *mut PageTable;
        
        // Traverse through levels 4, 3, and 2
        for level in [PageTableLevel::Four, PageTableLevel::Three, PageTableLevel::Two] {
            let index = addr.page_table_index(level);
            let table_ref = unsafe { &mut *table };
            let entry = &table_ref[index];
            
            if !entry.flags().contains(PageTableFlags::PRESENT) {
                return Err(MapError::PageAlreadyMapped);
            }
            
            if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
                return Err(MapError::ParentEntryHugePage);
            }
            
            table = entry.addr().as_u64() as *mut PageTable;
        }
        
        // Get the entry from the final page table (level 1)
        let index = addr.page_table_index(PageTableLevel::One);
        let table_ref = unsafe { &mut *table };
        Ok(&mut table_ref[index])
    }

    /// Get or create the page table entry for a virtual page
    /// 
    /// This traverses the page table hierarchy, creating new tables as needed.
    fn create_page_table_entry(&mut self, page: Page) -> MapResult<&mut PageTableEntry> {
        let addr = page.start_address();
        
        // Start at PML4
        let mut table = self.pml4 as *mut PageTable;
        
        // Traverse through levels 4, 3, and 2, creating tables as needed
        for level in [PageTableLevel::Four, PageTableLevel::Three, PageTableLevel::Two] {
            let index = addr.page_table_index(level);
            let table_ref = unsafe { &mut *table };
            let entry = &mut table_ref[index];
            
            if !entry.flags().contains(PageTableFlags::PRESENT) {
                // Allocate a new page table
                let frame = self.allocator.allocate_frame()?;
                let new_table = frame.start_address().as_u64() as *mut PageTable;
                
                // Zero out the new table
                unsafe {
                    (*new_table).zero();
                }
                
                // Set the entry to point to the new table
                let flags = PageTableFlags::PRESENT
                    .union(PageTableFlags::WRITABLE)
                    .union(PageTableFlags::USER_ACCESSIBLE);
                entry.set_addr(frame.start_address(), flags);
            }
            
            if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
                return Err(MapError::ParentEntryHugePage);
            }
            
            table = entry.addr().as_u64() as *mut PageTable;
        }
        
        // Get the entry from the final page table (level 1)
        let index = addr.page_table_index(PageTableLevel::One);
        let table_ref = unsafe { &mut *table };
        Ok(&mut table_ref[index])
    }

    /// Update the flags for an existing mapping
    /// 
    /// Returns an error if the page is not mapped.
    pub fn update_flags(&mut self, page: Page, flags: PageTableFlags) -> MapResult<()> {
        // Ensure the PRESENT flag is set
        let flags = flags.union(PageTableFlags::PRESENT);
        
        // Get the page table entry for this page
        let pt_entry = self.page_table_entry(page)?;
        
        // Check if the page is mapped
        if pt_entry.is_unused() {
            return Err(MapError::PageAlreadyMapped);
        }
        
        // Update the flags
        pt_entry.set_flags(flags);
        
        // Flush the TLB for this page
        flush_page(page.start_address());
        
        Ok(())
    }

    /// Identity map a physical frame (virtual address = physical address)
    /// 
    /// This is useful for memory-mapped I/O and during early boot.
    pub fn identity_map(
        &mut self,
        frame: PhysFrame,
        flags: PageTableFlags,
    ) -> MapResult<()> {
        let addr = frame.start_address().as_u64();
        let virt_addr = VirtAddr::new_unchecked(addr);
        let page = Page::containing_address(virt_addr);
        
        self.map_to(page, frame, flags)
    }
}

/// Flush the TLB entry for a single page
/// 
/// This function invalidates the TLB entry for the given virtual address,
/// ensuring that the next access will reload the page table entry.
pub fn flush_page(addr: VirtAddr) {
    unsafe {
        core::arch::asm!(
            "invlpg [{}]",
            in(reg) addr.as_u64(),
            options(nostack, preserves_flags)
        );
    }
}

/// Flush the entire TLB
/// 
/// This reloads CR3, which flushes all non-global TLB entries.
/// This is more expensive than flushing individual pages but simpler.
pub fn flush_all() {
    unsafe {
        core::arch::asm!(
            "mov {0}, cr3",
            "mov cr3, {0}",
            out(reg) _,
            options(nostack, preserves_flags)
        );
    }
}

/// Read the CR3 register to get the physical address of the active PML4 table
pub fn read_cr3() -> PhysAddr {
    let value: u64;
    unsafe {
        core::arch::asm!(
            "mov {}, cr3",
            out(reg) value,
            options(nostack, preserves_flags)
        );
    }
    PhysAddr::new(value & 0x000F_FFFF_FFFF_F000)
}

/// Write to the CR3 register to set the active PML4 table
/// 
/// # Safety
/// The caller must ensure that the physical address points to a valid
/// PML4 table that is properly initialized.
pub unsafe fn write_cr3(addr: PhysAddr) {
    core::arch::asm!(
        "mov cr3, {}",
        in(reg) addr.as_u64(),
        options(nostack, preserves_flags)
    );
}
