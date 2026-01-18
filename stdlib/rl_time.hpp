#ifndef RL_TIME_HPP
#define RL_TIME_HPP

#include <chrono>
#include <thread>

namespace rl {

    // Returns the current time as a Unix timestamp (seconds since epoch).
    inline double time() {
        return std::chrono::duration_cast<std::chrono::duration<double>>(
            std::chrono::system_clock::now().time_since_epoch()
        ).count();
    }

    // Pauses the program for a given number of seconds.
    inline void sleep(double seconds) {
        std::this_thread::sleep_for(std::chrono::duration<double>(seconds));
    }

} // namespace rl

#endif // RL_TIME_HPP
