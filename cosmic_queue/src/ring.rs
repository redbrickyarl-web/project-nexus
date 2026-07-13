use crossbeam_utils::CachePadded;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct CosmicRing<T, const CAP: usize> {
    buffer: [MaybeUninit<T>; CAP],
    head: CachePadded<AtomicUsize>,
    tail: CachePadded<AtomicUsize>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, const CAP: usize> CosmicRing<T, CAP> {
    pub const fn new() -> Self {
        assert!(CAP.is_power_of_two(), "CAP must be power of two");
        Self {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            head: CachePadded(AtomicUsize::new(0)),
            tail: CachePadded(AtomicUsize::new(0)),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn try_push(&self, value: T) -> Result<(), T> {
        let tail = self.tail.0.load(Ordering::Relaxed);
        let head = self.head.0.load(Ordering::Acquire);
        if tail.wrapping_sub(head) >= CAP {
            return Err(value);
        }
        unsafe { self.buffer[tail % CAP].as_mut_ptr().write(value); }
        self.tail.0.store(tail.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    pub fn try_pop(&self) -> Option<T> {
        let head = self.head.0.load(Ordering::Relaxed);
        let tail = self.tail.0.load(Ordering::Acquire);
        if head == tail {
            return None;
        }
        let value = unsafe { self.buffer[head % CAP].as_ptr().read() };
        self.head.0.store(head.wrapping_add(1), Ordering::Release);
        Some(value)
    }
}
