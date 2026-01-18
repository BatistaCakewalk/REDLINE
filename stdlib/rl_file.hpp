#ifndef RL_FILE_H
#define RL_FILE_H

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <stdexcept> // For std::runtime_error
#include <filesystem> // C++17 filesystem

namespace rl {

    namespace fs = std::filesystem;

    // Rips the soul out of a file and returns it as a string.
    // Throws an exception if the file refuses to yield its secrets.
    inline std::string read_file(const std::string& path) {
        std::ifstream file(path);
        if (!file.is_open()) {
            throw std::runtime_error("Could not open file: " + path);
        }
        std::stringstream buffer;
        buffer << file.rdbuf();
        return buffer.str();
    }

    // Shoves a string into a file. Overwrites everything. No mercy.
    // Throws an exception if the hard drive rejects our offering.
    inline bool write_file(const std::string& path, const std::string& content) {
        std::ofstream file(path);
        if (!file.is_open()) {
            throw std::runtime_error("Could not write to file: " + path);
        }
        file << content;
        return true;
    }

    // Checks if a file or directory exists in this dimension.
    inline bool exists(const std::string& path) {
        return fs::exists(path);
    }

    // Creates a directory.
    inline void mkdir(const std::string& path) {
        fs::create_directory(path);
    }

    // Deletes a file or directory. It's gone. Reduced to atoms.
    inline void remove(const std::string& path) {
        if (fs::exists(path)) {
            if (!fs::remove(path)) {
                throw std::runtime_error("Could not remove: " + path);
            }
        }
    }

    // Lists all files in a directory.
    inline std::vector<std::string> list_dir(const std::string& path) {
        std::vector<std::string> files;
        if (!fs::exists(path) || !fs::is_directory(path)) {
             throw std::runtime_error("Path is not a valid directory: " + path);
        }
        for (const auto& entry : fs::directory_iterator(path)) {
            files.push_back(entry.path().filename().string());
        }
        return files;
    }
}

#endif
