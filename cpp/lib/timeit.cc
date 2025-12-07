#include <functional>
#include <iostream>

#include <signal.h>
#include <unistd.h>

#if 0
#include <boost/chrono.hpp>
using namespace boost::chrono;
using Clock = process_cpu_clock;
#else
#include <chrono>
using namespace std::chrono;
using Clock = high_resolution_clock;
#endif



void
timeit(std::function<void(void)> f) {
    //using Clock = std::chrono::high_resolution_clock;
    std::cerr << "running payload...";
    static volatile bool done;
    done = false;
    signal(SIGALRM, [] (int) { done = true; } );
    alarm(4);
    auto start = Clock::now();
    unsigned long long iters;
    for (iters = 0; !done; ++iters)
        f();
    auto end = Clock::now();
    auto micros = duration_cast<microseconds>(
          end - start).count();
    double secondsfloat = double(micros) / 1000000;
    auto persec = double(iters) / double(secondsfloat);
    auto perop = double(micros) / double(iters);
    std::cout << "time taken: " << micros << "us, iterations=" << iters
       << ", operations per sec=" << (unsigned long long)(persec)
       << ", usec/operation: " << perop;
}
