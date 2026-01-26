.PHONY: all clean run

CXX = clang++
CXXFLAGS = -std=c++17 -O3 -flto -Wall -Wextra -mcpu=native -pthread -ffast-math -fno-exceptions -DNDEBUG
TARGET = bench

all: $(TARGET)

$(TARGET): bench.cpp BS_thread_pool.hpp
	$(CXX) $(CXXFLAGS) bench.cpp -o $(TARGET)

run: $(TARGET)
	POOL_SIZE=4 ./$(TARGET)

clean:
	rm -f $(TARGET)
	rm -rf target
