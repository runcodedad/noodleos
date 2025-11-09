/// Physical memory allocator using a bitmap approach
/// 
/// This allocator tracks physical memory frames using a bitmap where each bit
/// represents one 4KB frame. This is a simple and efficient approach that can
/// be extended or replaced with more sophisticated allocators later.

use super::constants::PAGE_SIZE;
use crate::arch::boot::multiboot2::{BootInfo, MemoryType};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Maximum physical memory we can manage (16 GB)
/// This limits bitmap size to a reasonable amount (512 KB for 16 GB)
const MAX_PHYSICAL_MEMORY: usize = 16 * 1024 * 1024 * 1024;

/// Number of frames we can track
const MAX_FRAMES: usize = MAX_PHYSICAL_MEMORY / PAGE_SIZE;

/// Size of the bitmap in bytes
const BITMAP_SIZE: usize = MAX_FRAMES / 8;

/// Bitmap allocator for physical memory frames
/// 
/// Each bit in the bitmap represents one 4KB physical frame:
/// - 0 = frame is free
/// - 1 = frame is allocated/reserved
pub struct BitmapAllocator {
    bitmap: &'static mut [u8],
    total_frames: usize,
    free_frames: AtomicUsize,
    start_frame: usize,
}

impl BitmapAllocator {
    /// Create a new uninitialized allocator
    /// 
    /// The allocator must be initialized with `init()` before use.
    pub const fn new() -> Self {
        Self {
            bitmap: &mut [],
            total_frames: 0,
            free_frames: AtomicUsize::new(0),
            start_frame: 0,
        }
    }
    
    /// Initialize the allocator using multiboot memory map
    /// 
    /// This function:
    /// 1. Finds available memory to store the bitmap
    /// 2. Marks all memory as reserved by default
    /// 3. Marks available regions from the memory map as free
    /// 4. Protects kernel memory and bitmap itself
    /// 
    /// # Safety
    /// Must be called exactly once during kernel initialization
    pub unsafe fn init(&mut self, boot_info: &BootInfo, kernel_start: usize, kernel_end: usize) {
        // Find the highest available memory address to determine total frames
        let mut highest_addr = 0u64;
        
        if let Some(mmap) = boot_info.memory_map() {
            for entry in mmap {
                let end_addr = entry.base_addr + entry.length;
                if end_addr > highest_addr {
                    highest_addr = end_addr;
                }
            }
        }
        
        // Cap at MAX_PHYSICAL_MEMORY
        if highest_addr > MAX_PHYSICAL_MEMORY as u64 {
            highest_addr = MAX_PHYSICAL_MEMORY as u64;
        }
        
        self.total_frames = (highest_addr as usize) / PAGE_SIZE;
        let bitmap_bytes = (self.total_frames + 7) / 8;
        
        // Find a suitable location for the bitmap
        // We'll place it right after the kernel
        let bitmap_start = align_up(kernel_end, PAGE_SIZE);
        let bitmap_end = bitmap_start + bitmap_bytes;
        
        // Create the bitmap slice
        self.bitmap = core::slice::from_raw_parts_mut(
            bitmap_start as *mut u8,
            bitmap_bytes
        );
        
        // Initially mark all memory as reserved (set all bits to 1)
        for byte in self.bitmap.iter_mut() {
            *byte = 0xFF;
        }
        
        // Now mark available regions as free based on memory map
        let mut free_count = 0;
        
        if let Some(mmap) = boot_info.memory_map() {
            for entry in mmap {
                if MemoryType::from_u32(entry.mem_type) == Some(MemoryType::Available) {
                    let start = entry.base_addr as usize;
                    let end = (entry.base_addr + entry.length) as usize;
                    free_count += self.mark_region_free(start, end);
                }
            }
        }
        
        // Mark kernel memory as reserved
        self.mark_region_reserved(kernel_start, kernel_end);
        
        // Mark bitmap memory as reserved
        self.mark_region_reserved(bitmap_start, bitmap_end);
        
        // Update free frame counter
        self.free_frames.store(self.count_free_frames(), Ordering::Relaxed);
    }
    
    /// Mark a memory region as free (available for allocation)
    /// Returns the number of frames marked as free
    fn mark_region_free(&mut self, start: usize, end: usize) -> usize {
        let start_frame = start / PAGE_SIZE;
        let end_frame = (end + PAGE_SIZE - 1) / PAGE_SIZE;
        let mut count = 0;
        
        for frame in start_frame..end_frame.min(self.total_frames) {
            if self.mark_frame_free(frame) {
                count += 1;
            }
        }
        
        count
    }
    
    /// Mark a memory region as reserved (not available for allocation)
    fn mark_region_reserved(&mut self, start: usize, end: usize) {
        let start_frame = start / PAGE_SIZE;
        let end_frame = (end + PAGE_SIZE - 1) / PAGE_SIZE;
        
        for frame in start_frame..end_frame.min(self.total_frames) {
            self.mark_frame_allocated(frame);
        }
    }
    
    /// Mark a single frame as free
    /// Returns true if the frame was previously allocated
    fn mark_frame_free(&mut self, frame: usize) -> bool {
        if frame >= self.total_frames {
            return false;
        }
        
        let byte_index = frame / 8;
        let bit_index = frame % 8;
        let mask = 1u8 << bit_index;
        
        let was_allocated = (self.bitmap[byte_index] & mask) != 0;
        self.bitmap[byte_index] &= !mask;
        was_allocated
    }
    
    /// Mark a single frame as allocated
    fn mark_frame_allocated(&mut self, frame: usize) {
        if frame >= self.total_frames {
            return;
        }
        
        let byte_index = frame / 8;
        let bit_index = frame % 8;
        let mask = 1u8 << bit_index;
        
        self.bitmap[byte_index] |= mask;
    }
    
    /// Check if a frame is free
    fn is_frame_free(&self, frame: usize) -> bool {
        if frame >= self.total_frames {
            return false;
        }
        
        let byte_index = frame / 8;
        let bit_index = frame % 8;
        let mask = 1u8 << bit_index;
        
        (self.bitmap[byte_index] & mask) == 0
    }
    
    /// Count total free frames (used during initialization)
    fn count_free_frames(&self) -> usize {
        let mut count = 0;
        
        for frame in 0..self.total_frames {
            if self.is_frame_free(frame) {
                count += 1;
            }
        }
        
        count
    }
    
    /// Allocate a single physical frame
    /// 
    /// Returns the physical address of the allocated frame, or None if
    /// no free frames are available.
    pub fn allocate_frame(&mut self) -> Option<usize> {
        // Simple first-fit algorithm: scan for the first free frame
        for frame in self.start_frame..self.total_frames {
            if self.is_frame_free(frame) {
                self.mark_frame_allocated(frame);
                self.free_frames.fetch_sub(1, Ordering::Relaxed);
                self.start_frame = frame + 1; // Hint for next allocation
                return Some(frame * PAGE_SIZE);
            }
        }
        
        // If we didn't find anything, wrap around and search from the beginning
        for frame in 0..self.start_frame {
            if self.is_frame_free(frame) {
                self.mark_frame_allocated(frame);
                self.free_frames.fetch_sub(1, Ordering::Relaxed);
                self.start_frame = frame + 1;
                return Some(frame * PAGE_SIZE);
            }
        }
        
        // No free frames available
        None
    }
    
    /// Allocate multiple contiguous physical frames
    /// 
    /// Returns the physical address of the first frame, or None if
    /// the requested number of contiguous frames cannot be allocated.
    pub fn allocate_frames(&mut self, count: usize) -> Option<usize> {
        if count == 0 {
            return None;
        }
        
        if count == 1 {
            return self.allocate_frame();
        }
        
        // Search for contiguous free frames
        let mut consecutive = 0;
        let mut start_frame = 0;
        
        for frame in 0..self.total_frames {
            if self.is_frame_free(frame) {
                if consecutive == 0 {
                    start_frame = frame;
                }
                consecutive += 1;
                
                if consecutive == count {
                    // Found enough contiguous frames
                    for f in start_frame..(start_frame + count) {
                        self.mark_frame_allocated(f);
                    }
                    self.free_frames.fetch_sub(count, Ordering::Relaxed);
                    self.start_frame = start_frame + count;
                    return Some(start_frame * PAGE_SIZE);
                }
            } else {
                consecutive = 0;
            }
        }
        
        None
    }
    
    /// Free a previously allocated frame
    /// 
    /// # Safety
    /// The caller must ensure that the frame is no longer in use.
    pub unsafe fn free_frame(&mut self, phys_addr: usize) {
        let frame = phys_addr / PAGE_SIZE;
        
        if frame < self.total_frames && !self.is_frame_free(frame) {
            self.mark_frame_free(frame);
            self.free_frames.fetch_add(1, Ordering::Relaxed);
            
            // Update hint for next allocation
            if frame < self.start_frame {
                self.start_frame = frame;
            }
        }
    }
    
    /// Free multiple contiguous frames
    /// 
    /// # Safety
    /// The caller must ensure that all frames are no longer in use.
    pub unsafe fn free_frames(&mut self, phys_addr: usize, count: usize) {
        let start_frame = phys_addr / PAGE_SIZE;
        
        for i in 0..count {
            let frame = start_frame + i;
            if frame < self.total_frames && !self.is_frame_free(frame) {
                self.mark_frame_free(frame);
                self.free_frames.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        if start_frame < self.start_frame {
            self.start_frame = start_frame;
        }
    }
    
    /// Get the number of free frames
    pub fn get_free_frames(&self) -> usize {
        self.free_frames.load(Ordering::Relaxed)
    }
    
    /// Get the total number of frames
    pub fn total_frames(&self) -> usize {
        self.total_frames
    }
    
    /// Get the number of allocated frames
    pub fn allocated_frames(&self) -> usize {
        self.total_frames.saturating_sub(self.get_free_frames())
    }
}

/// Round up to the nearest multiple of alignment
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

// Global physical memory allocator
static mut PHYSICAL_ALLOCATOR: BitmapAllocator = BitmapAllocator::new();

/// Initialize the physical memory allocator
/// 
/// # Safety
/// Must be called exactly once during kernel initialization
pub unsafe fn init_physical_allocator(
    boot_info: &BootInfo,
    kernel_start: usize,
    kernel_end: usize
) {
    PHYSICAL_ALLOCATOR.init(boot_info, kernel_start, kernel_end);
}

/// Allocate a single physical frame
pub fn allocate_frame() -> Option<usize> {
    unsafe { PHYSICAL_ALLOCATOR.allocate_frame() }
}

/// Allocate multiple contiguous physical frames
pub fn allocate_frames(count: usize) -> Option<usize> {
    unsafe { PHYSICAL_ALLOCATOR.allocate_frames(count) }
}

/// Free a physical frame
/// 
/// # Safety
/// The frame must have been allocated and must no longer be in use
pub unsafe fn free_frame(phys_addr: usize) {
    PHYSICAL_ALLOCATOR.free_frame(phys_addr);
}

/// Free multiple contiguous physical frames
/// 
/// # Safety
/// The frames must have been allocated and must no longer be in use
pub unsafe fn free_frames(phys_addr: usize, count: usize) {
    PHYSICAL_ALLOCATOR.free_frames(phys_addr, count);
}

/// Get statistics about physical memory
pub fn memory_stats() -> (usize, usize, usize) {
    unsafe {
        (
            PHYSICAL_ALLOCATOR.total_frames(),
            PHYSICAL_ALLOCATOR.get_free_frames(),
            PHYSICAL_ALLOCATOR.allocated_frames(),
        )
    }
}
