# 🟥 REDLINE v1.0.0

**C++ but Simplified.**

REDLINE is a high-performance, transpiled systems programming language designed to combine the **readability of Python** with the **raw speed of C++**. It compiles your REDLINE code (`.rl`) into optimized C++, which is then compiled into a native executable.

This project is feature-complete and ready for its v1.0 release.

## 🚀 Key Features

*   **Python-like Syntax**: Clean, indentation-based structure. No semicolons or curly braces required.
*   **C++ Performance**: Transpiles directly to C++, leveraging the full power of the G++ compiler.
*   **Object-Oriented**: Full support for classes, methods, and constructors with automatic memory management via smart pointers (`new` keyword).
*   **Modular**: Organize your code with `import` and `pub`, and manage projects with `RedConfig.toml`.
*   **Rich Standard Library**: Includes dictionaries, f-strings, file system I/O, command-line arguments, random number generation, and a time library.
*   **Modern Tooling**: Built-in Lexer, Parser, and Code Generator written in Rust for speed and safety.

## 📦 Installation

### Prerequisites
*   **Rust & Cargo**: To build the REDLINE compiler core.
*   **G++ (C++17 compatible)**: To compile the generated C++ code.
*   **Python 3**: To run the build script wrapper.
*   **Git**: To clone the repository.

### Linux & macOS
Run the installer script from your terminal. This will install the `redline` command system-wide.
```bash
# Download and run the installer
curl -sSL https://raw.githubusercontent.com/REDTOPS-Enterprise/REDLINE/main/install.sh | sudo bash```
You may be prompted for your password to create the system-wide command in `/usr/local/bin`.

### Windows
Download and run the `install.bat` script. This will copy the necessary files and add the `redline` command to your user's PATH.
```powershell
# 1. Download install.bat from the latest release.
# 2. Run the script from your terminal:
.\install.bat
```
You will need to **restart your terminal** for the `redline` command to become available.

## 🛠️ Usage

Once installed, you can use the `redline` command from anywhere.

### Building a Project
If your project has a `RedConfig.toml` file, you can build it from within the project directory:
```bash
# Navigate to your project
cd MyTestProject

# Build the project
redline build
```

### Compiling a Single File
You can also compile a single file directly:
```bash
redline build path/to/my_file.rl
```

## 📝 Example Code

**f-strings and Dictionaries:**
```redline
val scores: dict[string, int] = {
    "Alice": 100,
    "Bob": 90
}

for name in ["Alice", "Bob"]:
    print(f"{name}'s score is {scores[name]}.")
```

**Command-Line Arguments:**
```redline
# main.rl
print(f"You passed {len(args)} arguments.")
for arg in args:
    print(f" - {arg}")
```
```bash
$ redline build
$ ./my_program hello world
You passed 3 arguments.
 - ./my_program
 - hello
 - world
```

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) for the full list of planned features and future goals.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
