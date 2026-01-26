#include "BS_thread_pool.hpp"
#include <chrono>
#include <cstdint>
#include <cstdlib>
#include <cstdio>

constexpr uint64_t TASK_COUNT = 10'000'000;
constexpr uint64_t PRINT_INTERVAL = 333333;

void collatz(uint64_t n) {
    if (n == 0) {
        return;
    }
    uint64_t original = n;
    uint32_t steps = 0;
    while (n != 1) {
        if (n % 2 == 0) {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps++;
    }
    if (original % PRINT_INTERVAL == 0) {
        printf("%llu took %u steps to converge\n", (unsigned long long)original, steps);
    }
}

int main() {
    // get pool size from env
    const char* pool_size_str = std::getenv("POOL_SIZE");
    int pool_size = pool_size_str ? std::atoi(pool_size_str) : 1;

    BS::thread_pool pool(pool_size);

    auto start = std::chrono::high_resolution_clock::now();

    auto future = pool.submit_loop(0ULL, TASK_COUNT, [](uint64_t i) {
        collatz(i);
    });

    future.wait();

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    double per_second = TASK_COUNT / (elapsed.count() * 1000.0);

    printf("[CPP-BS T=%d] Processed %llu numbers in %.3fs (%.2f k-numbers/sec)\n",
           pool_size, (unsigned long long)TASK_COUNT, elapsed.count(), per_second);

    return 0;
}
