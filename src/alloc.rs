use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
    ptr::{null_mut, NonNull},
};
const ARENA_SIZE: usize = 128 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;

pub struct MemBlock {
    size: usize,
    next: Option<NonNull<MemBlock>>,
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct Allocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    free_list: UnsafeCell<Option<NonNull<MemBlock>>>,
}

unsafe impl Sync for Allocator {}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator {
            arena: UnsafeCell::new([0x55; ARENA_SIZE]),
            free_list: UnsafeCell::new(None),
        }
    }

    pub unsafe fn init(&self) {
        let arena_ptr = self.arena.get() as *mut u8;
        let node = MemBlock {
            size: ARENA_SIZE,
            next: None,
        };
        let node_ptr = arena_ptr as *mut MemBlock;
        node_ptr.write(node);
        *self.free_list.get() = Some(NonNull::new_unchecked(node_ptr));
    }

    unsafe fn merge_blocks(&self) {
        let mut current = *self.free_list.get();

        while let Some(mut node) = current {
            if let Some(mut next_node) = node.as_mut().next {
                let node_end = (node.as_ptr() as *mut u8).add(node.as_ref().size);
                if node_end == next_node.as_ptr() as *mut u8 {
                    node.as_mut().size +=
                        next_node.as_ref().size + core::mem::size_of::<MemBlock>();
                    node.as_mut().next = next_node.as_mut().next;
                }
            }
            current = node.as_ref().next;
        }
    }
    /// Align the given address upwards to the nearest multiple of `align` (ie. some power of two).
    fn align_up(addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let layout_size = layout.size();
        let layout_align = layout.align();
        if layout_align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }
        let mut current = *self.free_list.get();
        let mut prev: Option<NonNull<MemBlock>> = None;
        debug_assert!(current.is_some(), "current none");

        while let Some(node) = current {
            let node_ref = node.as_ref();
            let node_start = node.as_ptr() as usize;

            let aligned_start = Self::align_up(node_start, layout_align);
            let padding = aligned_start - node_start;

            if node_ref.size < layout_size + size_of::<MemBlock>() + padding {
                prev = current;
                current = node_ref.next;
                continue;
            }

            let alloc_start = aligned_start as *mut u8;
            let alloc_end = alloc_start.add(layout_size);

            let new_free_node = alloc_end as *mut MemBlock;
            new_free_node.write(MemBlock {
                size: node_ref.size - layout_size - size_of::<MemBlock>(),
                next: node_ref.next,
            });

            if let Some(mut prev_node) = prev {
                prev_node.as_mut().next = Some(NonNull::new_unchecked(new_free_node));
            } else {
                *self.free_list.get() = Some(NonNull::new_unchecked(new_free_node));
            }

            return alloc_start;
        }

        panic!("no avaliable blocks");
        return null_mut();
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let dealloc_node = MemBlock {
            size: layout.size(),
            next: None,
        };

        let dealloc_ptr = ptr as *mut MemBlock;
        dealloc_ptr.write(dealloc_node);

        let mut current = *self.free_list.get();
        let mut prev: Option<NonNull<MemBlock>> = None;

        // Find the correct position in the free list
        while let Some(mut node) = current {
            if node.as_ptr() > dealloc_ptr {
                break;
            }
            prev = current;
            current = node.as_mut().next;
        }

        // Insert the deallocated block into the free list
        if let Some(mut prev_node) = prev {
            prev_node.as_mut().next = Some(NonNull::new_unchecked(dealloc_ptr));
        } else {
            *self.free_list.get() = Some(NonNull::new_unchecked(dealloc_ptr));
        }

        (*dealloc_ptr).next = current;

        // Merge adjacent free blocks
        self.merge_blocks();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ALLOCATOR;

    #[test]
    fn test_basic_alloc_dealloc() {
        unsafe {
            ALLOCATOR.init();
        }
        panic!("{}", format_args!("{:?}", ALLOCATOR));

        let layout = Layout::array::<u8>(4).unwrap();
        let mem: *mut u8 = unsafe { ALLOCATOR.alloc(layout) };
        if mem.is_null() {
            panic!("Allocation failed: Null ptr returned");
        }
    }
}
