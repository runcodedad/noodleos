/// Multiboot2 parsing and memory map scanning
/// 
/// This module provides structures and functions to parse the Multiboot2
/// boot information structure provided by GRUB.

pub const MULTIBOOT2_MAGIC: u32 = 0x36d76289;

/// Multiboot2 tag types
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    End = 0,
    BootCommandLine = 1,
    BootLoaderName = 2,
    Module = 3,
    BasicMemInfo = 4,
    BiosBootDevice = 5,
    MemoryMap = 6,
    VbeInfo = 7,
    FramebufferInfo = 8,
    ElfSymbols = 9,
    ApmTable = 10,
    Efi32BitSystemTable = 11,
    Efi64BitSystemTable = 12,
    SmbiosTables = 13,
    AcpiOldRsdp = 14,
    AcpiNewRsdp = 15,
    NetworkingInfo = 16,
    EfiMemoryMap = 17,
    EfiBootServicesNotTerminated = 18,
    Efi32BitImageHandle = 19,
    Efi64BitImageHandle = 20,
    ImageLoadBasePhysicalAddress = 21,
}

/// Memory map entry type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Available = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    Nvs = 4,
    BadRam = 5,
}

impl MemoryType {
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            1 => Some(MemoryType::Available),
            2 => Some(MemoryType::Reserved),
            3 => Some(MemoryType::AcpiReclaimable),
            4 => Some(MemoryType::Nvs),
            5 => Some(MemoryType::BadRam),
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            MemoryType::Available => "Available",
            MemoryType::Reserved => "Reserved",
            MemoryType::AcpiReclaimable => "ACPI Reclaimable",
            MemoryType::Nvs => "NVS",
            MemoryType::BadRam => "Bad RAM",
        }
    }
}

/// Memory map entry from Multiboot2
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub mem_type: u32,
    _reserved: u32,
}

/// Generic Multiboot2 tag header
#[repr(C, packed)]
struct TagHeader {
    tag_type: u32,
    size: u32,
}

/// Memory map tag structure
#[repr(C, packed)]
struct MemoryMapTag {
    tag_type: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    // Followed by memory map entries
}

/// Boot info structure header
#[repr(C, packed)]
struct BootInfoHeader {
    total_size: u32,
    _reserved: u32,
    // Followed by tags
}

/// Iterator over memory map entries
/// This implements the standard Rust Iterator trait so we can use for loops
pub struct MemoryMapIter {
    current: *const u8,     // Points to the current entry we're reading
    end: *const u8,         // Points past the last entry
    entry_size: u32,        // How many bytes each entry takes (from Multiboot2)
}

impl Iterator for MemoryMapIter {
    type Item = MemoryMapEntry;  // Each call to next() returns a MemoryMapEntry
    
    fn next(&mut self) -> Option<Self::Item> {
        // Check if we've reached the end
        if self.current >= self.end {
            return None;
        }
        
        unsafe {
            // Cast the raw pointer to MemoryMapEntry and dereference it
            // This works because Multiboot2 guarantees the memory layout matches our struct
            let entry = *(self.current as *const MemoryMapEntry);
            // Move pointer forward by entry_size bytes to the next entry
            self.current = self.current.add(self.entry_size as usize);
            Some(entry)
        }
    }
}

/// Scans and parses Multiboot2 boot information
pub struct BootInfo {
    addr: usize,
}

impl BootInfo {
    /// Creates a new BootInfo from the address provided by the bootloader
    /// 
    /// # Safety
    /// The caller must ensure that `addr` points to a valid Multiboot2 structure
    pub unsafe fn new(addr: usize) -> Option<Self> {
        if addr == 0 {
            return None;
        }
        
        Some(BootInfo { addr })
    }
    
    /// Returns the total size of the boot information structure
    pub fn total_size(&self) -> u32 {
        unsafe {
            let header = self.addr as *const BootInfoHeader;
            (*header).total_size
        }
    }
    
    /// Finds and returns an iterator over memory map entries
    pub fn memory_map(&self) -> Option<MemoryMapIter> {
        unsafe {
            // Skip the first 8 bytes (BootInfoHeader: total_size + _reserved)
            // The Multiboot2 spec defines the structure as:
            //   u32 total_size
            //   u32 reserved (must be 0)
            //   followed by tags
            let mut current = (self.addr + 8) as *const TagHeader;
            let end = (self.addr + self.total_size() as usize) as *const TagHeader;
            
            while (current as usize) < (end as usize) {
                // We can cast to TagHeader because the Multiboot2 spec guarantees
                // that every tag starts with: u32 type, u32 size
                // Our TagHeader struct matches this exact layout (#[repr(C, packed)])
                let tag = &*current;
                
                if tag.tag_type == TagType::End as u32 {
                    break;
                }
                
                if tag.tag_type == TagType::MemoryMap as u32 {
                    // Cast to MemoryMapTag because we know this is a memory map tag
                    // The Multiboot2 spec defines memory map tags as:
                    //   u32 type, u32 size, u32 entry_size, u32 entry_version
                    //   followed by the actual memory entries
                    let mmap_tag = current as *const MemoryMapTag;
                    let entry_size = (*mmap_tag).entry_size;
                    // Skip the 16 bytes of MemoryMapTag header to get to entries
                    let entries_start = (mmap_tag as *const u8).add(16);
                    let entries_end = (current as *const u8).add((*mmap_tag).size as usize);
                    
                    return Some(MemoryMapIter {
                        current: entries_start,
                        end: entries_end,
                        entry_size,
                    });
                }
                
                // Move to next tag (align to 8-byte boundary)
                let next_addr = (current as usize + tag.size as usize + 7) & !7;
                current = next_addr as *const TagHeader;
            }
        }
        
        None
    }
    
    /// Prints the memory map to the console
    pub fn print_memory_map(&self) {
        use crate::arch::println;
        
        println("");
        println("=== Memory Map ===");
        
        if let Some(mmap) = self.memory_map() {
            let mut total_available = 0u64;
            let mut total_reserved = 0u64;
            
            for entry in mmap {
                let mem_type = MemoryType::from_u32(entry.mem_type)
                    .unwrap_or(MemoryType::Reserved);
                
                // Track totals
                match mem_type {
                    MemoryType::Available => total_available += entry.length,
                    _ => total_reserved += entry.length,
                }
                
                // Print entry
                crate::arch::print("  Base: 0x");
                print_hex(entry.base_addr);
                crate::arch::print("  Length: 0x");
                print_hex(entry.length);
                crate::arch::print("  (");
                print_size(entry.length);
                crate::arch::print(")  Type: ");
                println(mem_type.as_str());
            }
            
            println("");
            crate::arch::print("Total Available: ");
            print_size(total_available);
            println("");
            crate::arch::print("Total Reserved:  ");
            print_size(total_reserved);
            println("");
        } else {
            println("No memory map found!");
        }
        
        println("==================");
        println("");
    }
}

/// Helper function to print a hex value
fn print_hex(value: u64) {
    use crate::arch::print;
    
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
    let mut buffer = [0u8; 16];
    
    for i in 0..16 {
        let nibble = ((value >> (60 - i * 4)) & 0xF) as usize;
        buffer[i] = HEX_CHARS[nibble];
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
    print(s);
}

/// Helper function to print size in human-readable format
fn print_size(bytes: u64) {
    use crate::arch::print;
    
    let kb = bytes / 1024;
    let mb = kb / 1024;
    
    if mb > 0 {
        print_decimal(mb);
        print(" MB");
    } else if kb > 0 {
        print_decimal(kb);
        print(" KB");
    } else {
        print_decimal(bytes);
        print(" bytes");
    }
}

/// Helper function to print a decimal number
fn print_decimal(mut value: u64) {
    use crate::arch::print;
    
    if value == 0 {
        print("0");
        return;
    }
    
    let mut buffer = [0u8; 20];
    let mut i = 0;
    
    while value > 0 {
        buffer[i] = b'0' + (value % 10) as u8;
        value /= 10;
        i += 1;
    }
    
    // Reverse the buffer
    for j in 0..i/2 {
        buffer.swap(j, i - 1 - j);
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer[..i]) };
    print(s);
}
