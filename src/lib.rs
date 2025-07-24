// Include the generated bindings
#![allow(clippy::missing_safety_doc)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use core::ffi;
use std::{ops::Index, slice};

/// Rust wrapper for the C Packet struct
pub struct PacketWrapper {
    ptr: *mut Packet,
    layout: std::alloc::Layout,
}

impl PacketWrapper {
    /// Create a new PacketWrapper with the specified length
    pub fn new(length: u16) -> Option<Self> {
        let data_size = length as usize * std::mem::size_of::<i32>();
        let total_size = std::mem::size_of::<Packet>() + data_size;

        let layout =
            std::alloc::Layout::from_size_align(total_size, std::mem::align_of::<Packet>()).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) as *mut Packet };

        if ptr.is_null() {
            return None;
        }

        unsafe {
            (*ptr).length = length;
        }

        Some(Self { ptr, layout })
    }

    /// Get the length of the packet
    pub fn len(&self) -> u16 {
        unsafe { get_packet_len(self.ptr as *mut ffi::c_void) }
    }

    /// Check if the packet is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a slice to the packet data
    pub fn data(&self) -> &[i32] {
        if self.ptr.is_null() {
            return &[];
        }

        let length = self.len() as usize;
        if length == 0 {
            return &[];
        }

        unsafe { slice::from_raw_parts((*self.ptr).data.as_ptr(), length) }
    }

    /// Get the value at the specified index
    pub fn get(&self, index: usize) -> Option<&i32> {
        if self.ptr.is_null() {
            return None;
        }

        let length = self.len() as usize;
        if index >= length {
            return None;
        }

        unsafe { Some(&*(*self.ptr).data.as_ptr().add(index)) }
    }

    /// Get a mutable slice to the packet data
    pub fn data_mut(&mut self) -> &mut [i32] {
        if self.ptr.is_null() {
            return &mut [];
        }

        let length = self.len() as usize;
        if length == 0 {
            return &mut [];
        }

        unsafe { slice::from_raw_parts_mut((*self.ptr).data.as_mut_ptr(), length) }
    }

    /// Get the raw pointer
    pub fn as_ptr(&self) -> *const Packet {
        self.ptr
    }

    /// Get the raw mutable pointer
    pub fn as_mut_ptr(&mut self) -> *mut Packet {
        self.ptr
    }
}

impl<'a> IntoIterator for &'a PacketWrapper {
    type Item = &'a i32;
    type IntoIter = slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data().iter()
    }
}

impl<'a> IntoIterator for &'a mut PacketWrapper {
    type Item = &'a mut i32;
    type IntoIter = slice::IterMut<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_mut().iter_mut()
    }
}

impl std::fmt::Debug for PacketWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketWrapper")
            .field("length", &self.len())
            .field("data", &self.data())
            .finish()
    }
}

impl std::fmt::Display for PacketWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet(length: {}, data: {:?})", self.len(), self.data())
    }
}

impl Drop for PacketWrapper {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                std::alloc::dealloc(self.ptr as *mut u8, self.layout);
            }
        }
    }
}

impl TryFrom<&[i32]> for PacketWrapper {
    type Error = ();

    fn try_from(data: &[i32]) -> Result<Self, Self::Error> {
        let length = data.len() as u16;
        let mut wrapper = PacketWrapper::new(length).ok_or(())?;
        wrapper.data_mut().copy_from_slice(data);
        Ok(wrapper)
    }
}

impl Index<usize> for PacketWrapper {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_wrapper() {
        // Create a new packet wrapper
        let mut wrapper = PacketWrapper::new(3).expect("Failed to create packet wrapper");

        // Set some test data
        let data = wrapper.data_mut();
        data[0] = 1;
        data[1] = 2;
        data[2] = 3;

        assert_eq!(wrapper.len(), 3);
        assert!(!wrapper.is_empty());
        assert_eq!(wrapper.data(), &[1, 2, 3]);

        // Test iterator
        let collected: Vec<i32> = wrapper.into_iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_packet_wrapper_length_zero() {
        let wrapper = PacketWrapper::new(0).expect("Failed to create packet wrapper");
        assert_eq!(wrapper.len(), 0);
        assert!(wrapper.is_empty());
        assert_eq!(wrapper.data(), &[]);
        assert_eq!(wrapper.get(3), None);
    }

    #[test]
    fn test_packet_wrapper_from_slice() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let wrapper = PacketWrapper::try_from(&data[..]).expect("Failed to create packet wrapper");
        assert_eq!(wrapper.len(), 10);
        assert!(!wrapper.is_empty());
        assert_eq!(wrapper.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(wrapper[0], 1);
        assert_eq!(wrapper[1], 2);

        let vec: Vec<i32> = (1..=4).collect();
        let wrapper2 = PacketWrapper::try_from(&*vec).expect("Failed to create packet wrapper");
        assert_eq!(wrapper2.len(), 4);
        assert!(!wrapper2.is_empty());
        assert_eq!(wrapper2.data(), &[1, 2, 3, 4]);

        assert_eq!(wrapper2.get(2), Some(&3));
        assert_eq!(wrapper2.get(3), Some(&4));
        assert_eq!(wrapper2.get(4), None);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_packet_wrapper_out_of_bounds() {
        let data = [1, 2, 3];
        let wrapper = PacketWrapper::try_from(&data[..]).expect("Failed to create packet wrapper");
        _ = wrapper[10];
    }
}
