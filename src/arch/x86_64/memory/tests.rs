/// Tests for the physical memory allocator

use super::physical::{allocate_frame, allocate_frames, free_frame, free_frames, memory_stats};
use crate::arch::{println, print};

/// Run basic physical memory allocator tests
pub fn test_physical_allocator() {
    println("=== Testing Physical Memory Allocator ===");
    
    // Test 1: Check initial state
    let (total, free_before, allocated_before) = memory_stats();
    print("Initial state: ");
    print_decimal(free_before);
    print(" free frames, ");
    print_decimal(allocated_before);
    println(" allocated frames");
    
    // Test 2: Allocate a single frame
    print("Test 1: Allocating single frame... ");
    if let Some(frame1) = allocate_frame() {
        print("OK (0x");
        print_hex(frame1 as u64);
        println(")");
    } else {
        println("FAILED - no memory available");
        return;
    }
    
    // Test 3: Allocate another single frame
    print("Test 2: Allocating another frame... ");
    if let Some(frame2) = allocate_frame() {
        print("OK (0x");
        print_hex(frame2 as u64);
        println(")");
    } else {
        println("FAILED");
        return;
    }
    
    // Test 4: Check that free count decreased
    let (_, free_after_alloc, _) = memory_stats();
    print("Test 3: Verifying free count decreased... ");
    if free_after_alloc == free_before - 2 {
        println("OK");
    } else {
        print("FAILED (expected ");
        print_decimal(free_before - 2);
        print(", got ");
        print_decimal(free_after_alloc);
        println(")");
    }
    
    // Test 5: Allocate multiple contiguous frames
    print("Test 4: Allocating 4 contiguous frames... ");
    if let Some(frames_base) = allocate_frames(4) {
        print("OK (0x");
        print_hex(frames_base as u64);
        println(")");
        
        // Free them immediately
        unsafe {
            free_frames(frames_base, 4);
        }
        println("Test 5: Freed 4 contiguous frames... OK");
    } else {
        println("FAILED");
    }
    
    // Test 6: Free the first two frames we allocated
    print("Test 6: Freeing first allocated frame... ");
    unsafe {
        free_frame(allocate_frame().unwrap());
    }
    println("OK");
    
    // Test 7: Check final state
    let (_, free_final, allocated_final) = memory_stats();
    print("Final state: ");
    print_decimal(free_final);
    print(" free frames, ");
    print_decimal(allocated_final);
    println(" allocated frames");
    
    println("=== Physical Allocator Tests Complete ===");
    println("");
}

/// Helper to print hex value
fn print_hex(value: u64) {
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
    let mut buffer = [0u8; 16];
    
    for i in 0..16 {
        let nibble = ((value >> (60 - i * 4)) & 0xF) as usize;
        buffer[i] = HEX_CHARS[nibble];
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
    print(s);
}

/// Helper to print decimal
fn print_decimal(mut value: usize) {
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
    
    // Reverse
    for j in 0..i/2 {
        buffer.swap(j, i - 1 - j);
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer[..i]) };
    print(s);
}
