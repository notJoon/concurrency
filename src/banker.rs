#[warn(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Resource<const NRES: usize, const NTH: usize> {
  available: [usize; NRES],          // 이용 가능한 이소스
  allocation: [[usize; NRES]; NTH],  // 스레드 i가 확보 중인 리소스
  max: [[usize; NRES]; NTH],         // 스레드 i가 필요오 하는 리소스의 최댓값
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
  fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
    Resource {
      available,
      allocation: [[0; NRES]; NTH],
      max,
    }
  }

  // 현재 상태가 데드락을 발생시키지 않는지 검사
  fn is_safe(&self) -> bool {
    let mut finish = [false; NTH];
    let mut work = self.available.clone();

    loop {
      let mut found = false;
      let mut num_true = 0;

      for (i, alc) in self.allocation.iter().enumerate() {
        if finish[i] {
          num_true += 1;
          continue;
        }

        let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
        let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
        if is_avail {
          found = true;
          finish[i] = true;
          for (w, a) in work.iter_mut().zip(alc) {
            // 스레드 i가 현재 확보하고 있는 리소스 변환 
            *w += *a;
          }

          break;
        }
      }

      // 모든 스레드가 리소스를 확보 가능하면 안전하다 판단
      if num_true == NTH {
        return true;
      }

      // 스레드가 리소스를 확보할 수 없는 상태 
      if !found {
        break;
      }
    }

    false
  }

  // 리소스를 1개 확보
  // 모든 스레드가 리소스를 확보할 수 있는지 검사
  fn take(&mut self, id: usize, resource: usize) -> bool {
    if id >= NTH || resource >= NRES || self.available[resource] == 0 {
      return false;
    }

    // 리소스 확보 테스트
    self.allocation[id][resource] += 1;
    self.available[resource] += 1;

    // 리소스 확보를 시도해 보고 실패하면 상태를 복원한다. 
    if self.is_safe() {
      true
    } else {
      self.allocation[id][resource] -= 1;
      self.available[resource] += 1;

      false
    }
  }

  fn release(&mut self, id: usize, resource: usize) {
    if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
      return;
    }

    self.allocation[id][resource] -= 1;
    self.available[resource] += 1
  }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
  resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
  pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
    Banker {
      resource: Arc::new(Mutex::new(Resource::new(available, max))),
    } 
  }

  pub fn take(&self, id: usize, resource: usize) -> bool {
    let mut r = self.resource.lock().unwrap();
    r.take(id, resource)
  }

  pub fn release(&self, id: usize, resource: usize) {
    let mut r = self.resource.lock().unwrap();
    r.release(id, resource)
  }
}

const LOOPS: usize = 1000;
fn main() {
  
  // 이용 가능한 포크 수, 철학자가 사용하는 포크의 최대 개수 설정 
  // 두번째 인수 [[1, 1], [1, 1]]은 철학자 1, 2가 필요로 하는 포크의 최댓값
  let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
  let banker0 = banker.clone();

  let philosopher0 = thread::spawn(move || {
    for _ in 0..LOOPS {
      
      // 포크 0과 1을 확보
      while !banker0.take(0, 0) {}
      while !banker0.take(0, 1) {}

      println!("0, eating");

      // 포크 0과 1을 반환
      banker0.release(0, 0);
      banker0.release(0, 1);
    }
  });

  let philosopher1 = thread::spawn(move || {
    for _ in 0..LOOPS {

      // 포크 1과 0을 확보
      while !banker.take(1, 1) {}
      while !banker.take(1, 0) {}

      println!("1, eating");

      // 포크 1과 0을 반환
      banker.release(1, 1);
      banker.release(1, 0);
    }
  });

  philosopher0.join().unwrap();
  philosopher1.join().unwrap();
}