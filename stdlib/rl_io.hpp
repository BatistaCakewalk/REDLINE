// stdlib/rl_io.hpp
#ifndef RL_IO_H
#define RL_IO_H

#include <iostream>
#include <string>

namespace rl {
    // Using fast I/O
    inline void print(const std::string& msg) {
        std::ios_base::sync_with_stdio(false); // Speed boost
        std::cout << msg << "\n";
    }

    inline void print(int val) {
        std::cout << val << "\n";
    }
}
#endif
