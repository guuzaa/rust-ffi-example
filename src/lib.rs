mod bindings;

use core::ffi;
use std::{ops::Index, slice};

/// Macro to create a Packet with the given elements
///
/// # Examples
///
/// ```
/// use ffi_example::packet;
///
/// // Create a packet with specific values
/// let p = packet![1, 2, 3, 4];
/// assert_eq!(p.len(), 4);
/// assert!(!p.is_empty());
/// assert_eq!(p.data(), &[1, 2, 3, 4]);
///
/// // Create an empty packet
/// let empty = packet![];
/// assert_eq!(empty.len(), 0);
/// assert!(empty.is_empty());
/// assert_eq!(empty.data(), &[]);
///
/// // Create a packet with repeated values
/// let repeated = packet![42; 5];
/// assert_eq!(repeated.len(), 5);
/// assert!(!repeated.is_empty());
/// assert_eq!(repeated.data(), &[42, 42, 42, 42, 42]);
/// ```
#[macro_export]
macro_rules! packet {
    // Empty packet
    () => {
        $crate::Packet::new(0).expect("Failed to create empty packet")
    };

    // Packet with repeated value: packet![value; count]
    ($elem:expr; $n:expr) => {
        {
            let mut packet = $crate::Packet::new($n as u16).expect("Failed to create packet");
            let data = packet.data_mut();
            for i in 0..$n {
                data[i] = $elem;
            }
            packet
        }
    };

    // Packet with specific values: packet![1, 2, 3]
    ($($x:expr),+ $(,)?) => {
        {
            let data = [$($x),+];
            $crate::Packet::try_from(&data[..]).expect("Failed to create packet from data")
        }
    };
}

/// Rust wrapper for the C Packet struct
pub struct Packet {
    ptr: *mut bindings::Packet,
    layout: std::alloc::Layout,
}

impl Packet {
    /// Create a new Packet with the specified length
    pub fn new(length: u16) -> Option<Self> {
        let data_size = length as usize * std::mem::size_of::<i32>();
        let total_size = std::mem::size_of::<bindings::Packet>() + data_size;

        let layout = std::alloc::Layout::from_size_align(
            total_size,
            std::mem::align_of::<bindings::Packet>(),
        )
        .ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) as *mut bindings::Packet };

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
        unsafe { bindings::get_packet_len(self.ptr as *mut ffi::c_void) }
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
    pub fn as_ptr(&self) -> *const bindings::Packet {
        self.ptr
    }

    /// Get the raw mutable pointer
    pub fn as_mut_ptr(&mut self) -> *mut bindings::Packet {
        self.ptr
    }
}

impl<'a> IntoIterator for &'a Packet {
    type Item = &'a i32;
    type IntoIter = slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data().iter()
    }
}

impl<'a> IntoIterator for &'a mut Packet {
    type Item = &'a mut i32;
    type IntoIter = slice::IterMut<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_mut().iter_mut()
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Packet")
            .field("length", &self.len())
            .field("data", &self.data())
            .finish()
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Packet(length: {}, data: {:?})", self.len(), self.data())
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                std::alloc::dealloc(self.ptr as *mut u8, self.layout);
            }
        }
    }
}

impl TryFrom<&[i32]> for Packet {
    type Error = ();

    fn try_from(data: &[i32]) -> Result<Self, Self::Error> {
        let length = data.len() as u16;
        let mut wrapper = Packet::new(length).ok_or(())?;
        wrapper.data_mut().copy_from_slice(data);
        Ok(wrapper)
    }
}

impl Index<usize> for Packet {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

impl Index<std::ops::Range<usize>> for Packet {
    type Output = [i32];

    fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
        &self.data()[range]
    }
}

impl Index<std::ops::RangeFrom<usize>> for Packet {
    type Output = [i32];

    fn index(&self, range: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.data()[range]
    }
}

impl Index<std::ops::RangeTo<usize>> for Packet {
    type Output = [i32];

    fn index(&self, range: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.data()[range]
    }
}

impl Index<std::ops::RangeFull> for Packet {
    type Output = [i32];

    fn index(&self, _range: std::ops::RangeFull) -> &Self::Output {
        self.data()
    }
}

impl Index<std::ops::RangeInclusive<usize>> for Packet {
    type Output = [i32];

    fn index(&self, range: std::ops::RangeInclusive<usize>) -> &Self::Output {
        &self.data()[range]
    }
}

impl Index<std::ops::RangeToInclusive<usize>> for Packet {
    type Output = [i32];

    fn index(&self, range: std::ops::RangeToInclusive<usize>) -> &Self::Output {
        &self.data()[range]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_wrapper() {
        // Create a new packet wrapper
        let mut wrapper = Packet::new(3).expect("Failed to create packet wrapper");

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
        let wrapper = Packet::new(0).expect("Failed to create packet wrapper");
        assert_eq!(wrapper.len(), 0);
        assert!(wrapper.is_empty());
        assert_eq!(wrapper.data(), &[]);
        assert_eq!(wrapper.get(3), None);
    }

    #[test]
    fn test_packet_wrapper_from_slice() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let wrapper = Packet::try_from(&data[..]).expect("Failed to create packet wrapper");
        assert_eq!(wrapper.len(), 10);
        assert!(!wrapper.is_empty());
        assert_eq!(wrapper.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(wrapper[0], 1);
        assert_eq!(wrapper[1], 2);

        let vec: Vec<i32> = (1..=4).collect();
        let wrapper2 = Packet::try_from(&*vec).expect("Failed to create packet wrapper");
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
        let packet = packet![1, 2, 3];
        _ = packet[10];
    }

    #[test]
    fn test_packet_macro_single_value() {
        let packet = packet![123];
        assert_eq!(packet.len(), 1);
        assert!(!packet.is_empty());
        assert_eq!(packet.data(), &[123]);
    }

    #[test]
    fn test_packet_slice_indexing() {
        let packet = packet![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Test various slice patterns
        assert_eq!(&packet[..], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(&packet[..5], &[1, 2, 3, 4, 5]);
        assert_eq!(&packet[5..], &[6, 7, 8, 9, 10]);
        assert_eq!(&packet[2..7], &[3, 4, 5, 6, 7]);
        assert_eq!(&packet[..=4], &[1, 2, 3, 4, 5]);
        assert_eq!(&packet[5..=9], &[6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_packet_slice_edge_cases() {
        let packet = packet![1, 2, 3];

        // Test empty slices
        assert_eq!(&packet[0..0], &[]);
        assert_eq!(&packet[3..3], &[]);

        // Test full range
        assert_eq!(&packet[..], &[1, 2, 3]);

        // Test single element slices
        assert_eq!(&packet[1..2], &[2]);
        assert_eq!(&packet[..=0], &[1]);
        assert_eq!(&packet[2..=2], &[3]);
    }
}
