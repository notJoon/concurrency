use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const THREADS: usize = 4;
const LOOPS: usize = 10000;

struct SpinLock<T> {
  lock: AtomicBool,        // 락용 공유 변수
  data: UnsafeCell<T>,     // 보호 대상 데이터 
}

// 락 해제 및 막 보호 대상 데이터를 조작하기 위한 타입
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
      // 락용 공유변수가 false가 될 때 까지 대기
      while self.lock.load(Ordering::Relaxed) {}

      if let Ok(_) = 
        self.lock
            .compare_exchange_weak(
              false,
              true,
              Ordering::Acquire,
              Ordering::Relaxed
            )
      {
        break;
      }
    }

    SpinLockGuard{ spin_lock: self }
  }
}

// `SpinLock` 타입을 스레드 사이에서 동유 가능하도록 설정
unsafe impl<T> Sync for SpinLock<T> {}
unsafe impl<T> Send for SpinLock<T> {}

// 락 획득 후 자동으로 해제 
impl <'a, T> Drop for SpinLockGuard<'a, T> {
  fn drop(&mut self) {
    self.spin_lock.lock.store(false, Ordering::Release);
  }
}

// 보호대상의 immutable한 참조는 제외
impl <'a, T> Deref for SpinLockGuard<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.spin_lock.data.get() }
  }
}

// 보호 대상의 mutable한 참조 제외
impl <'a, T> DerefMut for SpinLockGuard<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.spin_lock.data.get() }
  }
}

fn main() {
  let lock = Arc::new(SpinLock::new(0));
  let mut v = Vec::new();

  for _ in 0..THREADS {
    let lock0 = lock.clone();

    //스레드 생성
    let t = std::thread::spawn(move || {
      for _ in 0..LOOPS {
        let mut data = lock0.lock();
        *data += 1;
      }
    });

    v.push(t);
  }

  for t in v {
    t.join().unwrap();
  }

  println!(
    "COUNT = {} (expected={})",
    *lock.lock(),
    LOOPS * THREADS
  );
}