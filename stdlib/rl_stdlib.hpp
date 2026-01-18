#ifndef RL_STDLIB_HPP
#define RL_STDLIB_HPP

#include <vector>
#include <string>
#include <algorithm> // For sort, reverse, find

namespace rl {
    // Global command line arguments
    extern std::vector<std::string> args;

    // Returns the number of elements in a vector.
    template<typename T>
    int len(const std::vector<T>& vec) {
        return vec.size();
    }

    // Appends an element to a vector.
    template<typename T>
    void append(std::vector<T>& vec, const T& value) {
        vec.push_back(value);
    }

    // Sorts a vector in ascending order.
    // Warning: This modifies the list in place. Chaos is order.
    template<typename T>
    void sort(std::vector<T>& vec) {
        std::sort(vec.begin(), vec.end());
    }

    // Reverses the order of elements in a vector.
    // Time flows backwards for this list.
    template<typename T>
    void reverse(std::vector<T>& vec) {
        std::reverse(vec.begin(), vec.end());
    }

    // Finds the first occurrence of a value in a vector.
    // Returns the index, or -1 if the universe hides it from you.
    template<typename T>
    int find(const std::vector<T>& vec, const T& value) {
        auto it = std::find(vec.begin(), vec.end(), value);
        if (it != vec.end()) {
            return std::distance(vec.begin(), it);
        }
        return -1;
    }

    // --- Type Conversion Helpers ---

    // Identity function for strings.
    inline std::string to_string(const std::string& s) {
        return s;
    }

    // Wrapper for std::to_string (int)
    inline std::string to_string(int val) {
        return std::to_string(val);
    }

    // Wrapper for std::to_string (double)
    inline std::string to_string(double val) {
        return std::to_string(val);
    }

    // Wrapper for std::to_string (bool)
    inline std::string to_string(bool val) {
        return val ? "true" : "false";
    }
}

#endif // RL_STDLIB_HPP
