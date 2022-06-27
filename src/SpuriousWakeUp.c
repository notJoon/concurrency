// gcc SpuriousWakeUp.c -o executable
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

//sigal handler
void handler(int signal) { printf("received signal: %d\n", signal); }

int main(int argc, char *argv[]) {
  // 프로세스 ID 표시
  pid_t pid = getpid();
  printf("pid: %d\n", pid);

  // 시그널 핸들러 등록
  signal(SIGUSR1, handler);

  // wait. 하지만 누구도 notify를 하지 않으면 정지
  pthread_mutex_lock(&mutex);
  if (pthread_cond_wait(&cond, &mutex) != 0) {
    perror("pthread_cond_wait");
    exit(1);
  }

  printf("sprious wake up \n");
  pthread_mutex_unlock(&mutex);

  return 0;
}