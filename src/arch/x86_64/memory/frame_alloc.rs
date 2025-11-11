/// Frame allocator trait for virtual memory mapping
/// 
/// This trait provides an interface between the virtual memory mapper
/// and the physical memory allocator, allowing the mapper to allocate
/// physical frames for new page tables.

use super::paging::{PhysFrame, PhysAddr};

/// Result type for frame allocation
pub type FrameAllocResult = Result<PhysFrame, FrameAllocError>;

/// Errors that can occur during frame allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameAllocError {
    /// No more physical memory available
    OutOfMemory,
    /// The allocator is not initialized
    NotInitialized,
}

/// Trait for allocating physical memory frames
/// 
/// This trait must be implemented by physical memory allocators
/// to enable virtual memory mapping.
pub trait FrameAllocator {
    /// Allocate a single 4KB physical frame
    /// 
    /// Returns the physical frame on success, or an error if allocation fails.
    fn allocate_frame(&mut self) -> FrameAllocResult;

    /// Deallocate a previously allocated frame
    /// 
    /// # Safety
    /// The frame must have been previously allocated by this allocator
    /// and must not be in use anymore.
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame);
}

/// Frame allocator implementation that wraps our bitmap allocator
pub struct BitmapFrameAllocator;

impl BitmapFrameAllocator {
    /// Create a new bitmap frame allocator
    pub const fn new() -> Self {
        Self
    }
}

impl FrameAllocator for BitmapFrameAllocator {
    fn allocate_frame(&mut self) -> FrameAllocResult {
        use super::physical;
        
        // Try to allocate a frame from the global allocator
        match physical::allocate_frame() {
            Some(addr) => {
                let phys_addr = PhysAddr::new(addr as u64);
                Ok(PhysFrame::containing_address(phys_addr))
            }
            None => Err(FrameAllocError::OutOfMemory),
        }
    }

    unsafe fn deallocate_frame(&mut self, frame: PhysFrame) {
        use super::physical;
        
        let addr = frame.start_address().as_u64() as usize;
        physical::free_frame(addr);
    }
}

/// Empty frame allocator for testing
/// 
/// This allocator always returns an error and is useful for
/// testing code paths that don't require actual allocation.
pub struct EmptyFrameAllocator;

impl FrameAllocator for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> FrameAllocResult {
        Err(FrameAllocError::OutOfMemory)
    }

    unsafe fn deallocate_frame(&mut self, _frame: PhysFrame) {
        // Do nothing
    }
}
