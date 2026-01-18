#ifndef RL_RANDOM_HPP
#define RL_RANDOM_HPP

#include <random>
#include <chrono>

namespace rl {

    // A simple random number generator.
    // Seeded once to ensure different results on each run.
    inline std::mt19937& get_random_engine() {
        static std::mt19937 engine(static_cast<unsigned int>(std::chrono::high_resolution_clock::now().time_since_epoch().count()));
        return engine;
    }

    // Generates a random integer between min and max (inclusive).
    inline int random_int(int min, int max) {
        std::uniform_int_distribution<int> dist(min, max);
        return dist(get_random_engine());
    }

    // Generates a random float between 0.0 and 1.0.
    inline double random_float() {
        std::uniform_real_distribution<double> dist(0.0, 1.0);
        return dist(get_random_engine());
    }

} // namespace rl

#endif // RL_RANDOM_HPP
