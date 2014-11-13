#ifndef __BENCH_H__
#define __BENCH_H__

#include <sys/time.h>
#include <time.h>
#include <stdio.h>

#ifdef __MACH__
#include <mach/clock.h>
#include <mach/mach.h>
#endif

void current_utc_time(struct timespec *ts) {
#ifdef __MACH__ // OS X does not have clock_gettime, use clock_get_time
  clock_serv_t cclock;
  mach_timespec_t mts;
  host_get_clock_service(mach_host_self(), CALENDAR_CLOCK, &cclock);
  clock_get_time(cclock, &mts);
  mach_port_deallocate(mach_task_self(), cclock);
  ts->tv_sec = mts.tv_sec;
  ts->tv_nsec = mts.tv_nsec;
#else
  clock_gettime(CLOCK_REALTIME, ts);
#endif
}

template<typename T>
void bench(const char* name, T& state, void (*f)(T&)) {
  int iters = 100000;

  struct timespec start_ts, end_ts;
  current_utc_time(&start_ts);

  for (int i = 0; i < iters; ++i) {
    (f)(state);
  }

  current_utc_time(&end_ts);

  unsigned long sec_delta = end_ts.tv_sec - start_ts.tv_sec;
  unsigned long nsec_delta = end_ts.tv_nsec - start_ts.tv_nsec;
  unsigned long delta = sec_delta * 1000000000 + nsec_delta;
  double delta_per_iter = (double)delta / (double)iters;

  printf("%s:\t%.0f ns/iter\n", name, delta_per_iter);
}

template<typename T>
void bench(const char* name, T& state, size_t size, void (*f)(T&)) {
  int iters = 100000;

  struct timespec start_ts, end_ts;
  current_utc_time(&start_ts);

  for (int i = 0; i < iters; ++i) {
    (f)(state);
  }

  current_utc_time(&end_ts);

  unsigned long sec_delta = end_ts.tv_sec - start_ts.tv_sec;
  unsigned long nsec_delta = end_ts.tv_nsec - start_ts.tv_nsec;
  unsigned long delta = sec_delta * 1000000000 + nsec_delta;
  double delta_per_iter = (double)delta / (double)iters;
  double bandwidth = ((double)size / (double)delta_per_iter) * 1000.0;

  printf("%s:\t%.0f ns/iter = %.0f MB/s\n", name, delta_per_iter, bandwidth);
}

#endif
