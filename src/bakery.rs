use std::ptr::{read_volatile, write_volatile};
use std::sync::atomic::{fence, Ordering};
use std::thread;

const THREADS: usize = 4;
const LOOPS: usize = 100000;

// volatile용 메크로
macro_rules! write_mem {
  ($addr: expr, $val: expr) => {
    unsafe {write_volatile($addr, $val)}
  };
}

macro_rules! read_mem {
  ($addr: expr) => { unsafe { read_volatile($addr) } };
}


struct BakeryLock {
  entering: [bool; THREADS],
  tickets: [Option<u64>; THREADS],
}

impl BakeryLock {
  fn lock(&mut self, idx: usize) -> LockGuard {
    fence(Ordering::SeqCst);
    write_mem!(&mut self.entering[idx], true);
    fence(Ordering::SeqCst);

    // 현재 티켓의 최댓값 가져오기
    let mut max = 0;
    for i in 0..THREADS {
      if let Some(t) = read_mem!(&self.tickets[i]) {
        max = max.max(t);
      }
    }

    let ticket = max + 1;
    write_mem!(&mut self.tickets[idx], Some(ticket));

    fence(Ordering::SeqCst);
    write_mem!(&mut self.entering[idx], false);
    fence(Ordering::SeqCst);

    // 대기 처리
    for i in 0..THREADS {
      if i == idx {
        continue;
      }

      while read_mem!(&self.entering[i]) {}
      loop {
        match read_mem!(&self.tickets[i]) {
          Some(t) => {
            // 대기 종료 판단
            if ticket < t ||
              (ticket == t && idx < i) {
                break;
            }
          }

          None => {
            // thread[i]가 처리 중 아님 
            break;
          }
        }
      }
    }

    fence(Ordering::SeqCst);
    LockGuard{ idx }
  }
}

static mut LOCk: BakeryLock = BakeryLock {
  entering: [false; THREADS],
  tickets: [None; THREADS],
};

static mut COUNT: usize = 64;


struct LockGuard {
  idx: usize,
}

// lock 획득 후 자동으로 해제되도록 `Drop` 트레이트 구현
impl Drop for LockGuard {
  fn drop(&mut self) {
    fence(Ordering::SeqCst);
    write_mem(&mut LOCK.tickets[self.idx], None);
  }
}

fn main() {
    let mut v = Vec::new();
  for i in 0..THREADS {
    let th = thread::spawn(move || {
      for _ in 0..LOOPS {
        let _lock = unsafe { LOCK.lock(i) };
        unsafe {
          let c = read_volatile(&COUNT);
          write_volatile(&mut COUNT, c + 1);
        }
      }
    });

    v.push(th);
  }

  for th in v {
    th.join().unwrap();
  }

  println!(
    "COUNT = {} (exp = {}",
    unsafe { COUNT };
    LOOPS * THREADS
  );
}