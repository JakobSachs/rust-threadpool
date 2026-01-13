.PHONY: all clean run

CXX = g++
CXXFLAGS = -std=c++17 -O3 -Wall -Wextra -pthread -march=native
TARGET = bench

all: $(TARGET)

$(TARGET): bench.cpp BS_thread_pool.hpp
	$(CXX) $(CXXFLAGS) bench.cpp -o $(TARGET)

run: $(TARGET)
	POOL_SIZE=4 ./$(TARGET)

clean:
	rm -f $(TARGET)
	rm -rf target
