use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const NUM_THREADS: usize = 10;
const NUM_LOOP: usize = 20;

// 스핀락용 타입
struct SpinLock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// 록 해제 및 록 중에 보호 대상 데이터를 조작하기 위한 타입
struct SpinLockGuard<'a, T> {
    spin_lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    fn new(v: T) -> Self {
        SpinLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(v),
        }
    }

    fn lock(&self) -> SpinLockGuard<T> {
        loop {
            while self.lock.load(Ordering::Relaxed) {}

            if let Ok(_) = self.lock.compare_exchange_weak(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                break;
            }
        }
        SpinLockGuard {
            spin_lock: self,
        }
    }
}

unsafe impl<T> Sync for SpinLock<T> {}
unsafe impl<T> Send for SpinLock<T> {}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.spin_lock.lock.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spin_lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spin_lock.data.get() }
    }
}

// 큐 구조체
struct VecQueue<T> {
    data: Vec<T>,
}

impl<T> VecQueue<T> {
    fn new() -> Self {
        VecQueue { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

fn main() {
    let lock = Arc::new(SpinLock::new(VecQueue::new()));
    let mut v = Vec::new();

    for i in 0..NUM_THREADS {
        let lock0 = Arc::clone(&lock);
        let t = std::thread::spawn(move || {
            for j in 0..NUM_LOOP {
                let mut data = lock0.lock();
                data.push(format!("DATA thread:{} ,number: {}", i+1, j+1));
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    let mut queue = lock.lock();
    let count = queue.data.len();
    // 큐의 내용 출력
    let mut count_number = 0;
    while let Some(item) = queue.pop() {
        println!("index: {} / {}",count_number ,item);
        count_number += 1;
    }
    println!("COUNT = {} (expected = {})", count, NUM_LOOP * NUM_THREADS);
}