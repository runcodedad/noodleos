/// Page table management for x86_64 virtual memory
/// 
/// x86_64 uses 4-level paging with:
/// - PML4 (Page Map Level 4) - top level
/// - PDPT (Page Directory Pointer Table)
/// - PD (Page Directory)
/// - PT (Page Table)
/// 
/// Each table has 512 entries, and each entry is 8 bytes.
/// Virtual addresses are translated through all 4 levels to reach physical frames.

use super::constants::PAGE_SIZE;
use core::ops::{Index, IndexMut};

/// Number of entries in each page table
pub const ENTRY_COUNT: usize = 512;

/// Page table entry flags
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableFlags(u64);

impl PageTableFlags {
    /// Page is present in memory
    pub const PRESENT: Self = Self(1 << 0);
    /// Page is writable (otherwise read-only)
    pub const WRITABLE: Self = Self(1 << 1);
    /// Page is accessible from user mode
    pub const USER_ACCESSIBLE: Self = Self(1 << 2);
    /// Write-through caching
    pub const WRITE_THROUGH: Self = Self(1 << 3);
    /// Disable cache for this page
    pub const NO_CACHE: Self = Self(1 << 4);
    /// Page has been accessed
    pub const ACCESSED: Self = Self(1 << 5);
    /// Page has been written to (dirty)
    pub const DIRTY: Self = Self(1 << 6);
    /// Huge page (2MB or 1GB depending on level)
    pub const HUGE_PAGE: Self = Self(1 << 7);
    /// Page won't be flushed from cache on address space switch
    pub const GLOBAL: Self = Self(1 << 8);
    /// Disable execution (NX bit, requires EFER.NXE)
    pub const NO_EXECUTE: Self = Self(1 << 63);

    /// Create empty flags
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Check if a flag is set
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Set a flag
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Clear a flag
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    /// Combine two flag sets
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Get the raw flags value
    pub const fn bits(self) -> u64 {
        self.0
    }
}

/// A 64-bit page table entry
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    /// Create a new unused entry
    pub const fn new() -> Self {
        Self { entry: 0 }
    }

    /// Check if the entry is unused (not present)
    pub const fn is_unused(&self) -> bool {
        self.entry == 0
    }

    /// Set this entry to zero (mark as unused)
    pub fn set_unused(&mut self) {
        self.entry = 0;
    }

    /// Get the flags for this entry
    pub const fn flags(&self) -> PageTableFlags {
        PageTableFlags(self.entry & 0xFFF)
    }

    /// Get the physical address this entry points to
    /// Returns the 4KB-aligned physical frame address (bits 12-51)
    pub const fn addr(&self) -> PhysAddr {
        PhysAddr(self.entry & 0x000F_FFFF_FFFF_F000)
    }

    /// Get the physical frame this entry points to
    pub const fn frame(&self) -> PhysFrame {
        PhysFrame::containing_address(self.addr())
    }

    /// Set the physical address and flags for this entry
    pub fn set_addr(&mut self, addr: PhysAddr, flags: PageTableFlags) {
        assert!(addr.is_aligned(PAGE_SIZE), "Address must be page-aligned");
        self.entry = addr.0 | flags.bits();
    }

    /// Set the flags for this entry without changing the address
    pub fn set_flags(&mut self, flags: PageTableFlags) {
        let addr = self.addr();
        self.entry = addr.0 | flags.bits();
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PageTableEntry")
            .field("addr", &format_args!("{:#x}", self.addr().0))
            .field("flags", &self.flags())
            .finish()
    }
}

/// A physical memory address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(pub u64);

impl PhysAddr {
    /// Create a new physical address
    pub const fn new(addr: u64) -> Self {
        Self(addr)
    }

    /// Check if the address is aligned to the given alignment
    pub const fn is_aligned(&self, align: usize) -> bool {
        self.0 % align as u64 == 0
    }

    /// Align down to the given alignment
    pub const fn align_down(&self, align: usize) -> Self {
        Self(self.0 & !((align as u64) - 1))
    }

    /// Align up to the given alignment
    pub const fn align_up(&self, align: usize) -> Self {
        Self((self.0 + (align as u64) - 1) & !((align as u64) - 1))
    }

    /// Get the raw address value
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

/// A virtual memory address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    /// Create a new virtual address
    /// Panics if the address is not canonical (bits 48-63 must match bit 47)
    pub const fn new(addr: u64) -> Self {
        // Check if address is canonical
        let bits_48_63 = addr >> 48;
        
        // In canonical form, bits 48-63 must be all 0s or all 1s
        assert!(
            bits_48_63 == 0 || bits_48_63 == 0xFFFF,
            "Address is not canonical"
        );
        
        Self(addr)
    }

    /// Create a new virtual address without checking if it's canonical
    pub const fn new_unchecked(addr: u64) -> Self {
        Self(addr)
    }

    /// Check if the address is aligned to the given alignment
    pub const fn is_aligned(&self, align: usize) -> bool {
        self.0 % align as u64 == 0
    }

    /// Align down to the given alignment
    pub const fn align_down(&self, align: usize) -> Self {
        Self(self.0 & !((align as u64) - 1))
    }

    /// Align up to the given alignment
    pub const fn align_up(&self, align: usize) -> Self {
        Self((self.0 + (align as u64) - 1) & !((align as u64) - 1))
    }

    /// Get the raw address value
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    /// Get the page table index for the given level
    /// Level 4 = PML4, Level 3 = PDPT, Level 2 = PD, Level 1 = PT
    pub const fn page_table_index(&self, level: PageTableLevel) -> usize {
        let shift = 12 + (level as usize - 1) * 9;
        ((self.0 >> shift) & 0x1FF) as usize
    }

    /// Get the offset within the page
    pub const fn page_offset(&self) -> usize {
        (self.0 & 0xFFF) as usize
    }
}

/// A physical 4KB frame
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysFrame {
    start_address: PhysAddr,
}

impl PhysFrame {
    /// Create a frame containing the given address
    pub const fn containing_address(addr: PhysAddr) -> Self {
        Self {
            start_address: addr.align_down(PAGE_SIZE),
        }
    }

    /// Get the starting physical address of this frame
    pub const fn start_address(&self) -> PhysAddr {
        self.start_address
    }

    /// Get the frame number
    pub const fn number(&self) -> u64 {
        self.start_address.0 / PAGE_SIZE as u64
    }
}

/// A virtual 4KB page
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    start_address: VirtAddr,
}

impl Page {
    /// Create a page containing the given address
    pub const fn containing_address(addr: VirtAddr) -> Self {
        Self {
            start_address: addr.align_down(PAGE_SIZE),
        }
    }

    /// Get the starting virtual address of this page
    pub const fn start_address(&self) -> VirtAddr {
        self.start_address
    }

    /// Get the page number
    pub const fn number(&self) -> u64 {
        self.start_address.0 / PAGE_SIZE as u64
    }
}

/// Page table levels in the 4-level paging hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PageTableLevel {
    /// Page Table (PT) - Level 1, maps to 4KB pages
    One = 1,
    /// Page Directory (PD) - Level 2, can map 2MB huge pages
    Two = 2,
    /// Page Directory Pointer Table (PDPT) - Level 3, can map 1GB huge pages
    Three = 3,
    /// Page Map Level 4 (PML4) - Level 4, top level
    Four = 4,
}

/// A page table with 512 entries
#[repr(align(4096))]
#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    /// Create a new empty page table
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry::new(); ENTRY_COUNT],
        }
    }

    /// Clear all entries in this table
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }

    /// Get an iterator over the entries
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.entries.iter()
    }

    /// Get a mutable iterator over the entries
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        self.entries.iter_mut()
    }

    /// Get the physical address of the next level page table for the given entry
    /// Returns None if the entry is not present or is a huge page
    pub fn next_table(&self, index: usize) -> Option<&PageTable> {
        let entry = &self.entries[index];
        
        if !entry.flags().contains(PageTableFlags::PRESENT) {
            return None;
        }
        
        if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            return None;
        }
        
        let table_addr = entry.addr().as_u64() as *const PageTable;
        Some(unsafe { &*table_addr })
    }

    /// Get a mutable reference to the next level page table for the given entry
    /// Returns None if the entry is not present or is a huge page
    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut PageTable> {
        let entry = &self.entries[index];
        
        if !entry.flags().contains(PageTableFlags::PRESENT) {
            return None;
        }
        
        if entry.flags().contains(PageTableFlags::HUGE_PAGE) {
            return None;
        }
        
        let table_addr = entry.addr().as_u64() as *mut PageTable;
        Some(unsafe { &mut *table_addr })
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl core::fmt::Debug for PageTable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PageTable")
            .field("entries", &format_args!("[... {} entries ...]", ENTRY_COUNT))
            .finish()
    }
}
