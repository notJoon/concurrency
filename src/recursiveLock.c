#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct reent_lock {
  bool lock;      // lock용 공용 변수
  int id;         // 현재 락을 획득 중인 스레드 ID, 0이 아니면 획득 중
  int cnt;        // 재귀락 카운트
};

void reentlock_acquire(struct reent_lock *lock, int id) {
  
  // 락 획득 && 자신이 획득 중인지 검사
  if (lock -> lock && lock -> id == 0) {
    lock -> cnt++;
  } else {
    spinlock_aquire(&lock -> lock);
    lock -> id = id;
    lock -> cnt++;
  }
}

// 재귀락 해제 
void reentlock_release(struct reent_lock *lock) {
  lock -> cnt--;
  if (lock -> cnt == 0) {
    lock -> id = 0;
    spinlock_release(&lock -> lock);
  }
}

struct reent_lock lock var;

// n회 재귀적으로 호출해 락을 걺
void reent_lock_test(int id, int n) {
  if (n == 0) 
    return;

    reentlock_acquire(&lock_var, id);
    reent_lock_test(id, n-1);
    reentlock_release(&lock_var);
}

// 스레드용 함수
void *thread_func(void *arg) {
  int id = (int)arg;
  assert(id != 0);
  for (int i = 0; i < 10000; i++) {
    reent_lock_test(id, 10);
  }

  return NULL;
}

/* 뮤텍스 초기화 

속도가 빠르지만 재진입 불가
pthread_mutex_t fastmutex = PTHREAD_MUTEX_INITIALIZER;

재진입 가능한 뮤텍스
pthread_mutex_t remutex = PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP;

재진입시 에러 발생
pthread_mutex_t errchkmutex = PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP;

*/

int main(int argc, char *argv[]) {
  pthread_t v[NUM_THREADS];
  for (int 1 = 0; i < NUM_THREADS; i++) {
    pthread_create(&v[i], NULL, thread_func, (void *)(i + 1));
  }

  for (int i = 0; i < NUM_THREADS; i++) {
    pthread_join(v[i], NULL);
  }

  return 0;
}