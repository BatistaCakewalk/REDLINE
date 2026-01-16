#!/usr/bin/env python3
import os
import subprocess
import sys
import json
import re
from pathlib import Path

# --- Constants ---
PROJECT_ROOT = Path(__file__).parent.resolve()
CORE_DIR = PROJECT_ROOT / "redline-core"
CORE_BIN = CORE_DIR / "target" / "release" / "redline-core"
BUILD_DIR = PROJECT_ROOT / "build"

class Module:
    """Represents a single REDLINE module (a .rl file)."""
    def __init__(self, source_path, ast):
        self.source_path = source_path
        self.ast = ast
        self.name = source_path.stem
        self.cpp_path = BUILD_DIR / f"{self.name}.cpp"
        self.hpp_path = BUILD_DIR / f"{self.name}.hpp"
        self.obj_path = BUILD_DIR / f"{self.name}.o"

    def get_imports(self):
        """Extracts import paths from the module's AST."""
        imports = []
        for statement in self.ast.get('statements', []):
            if 'Import' in statement:
                imports.append(statement['Import'])
        return imports

class Compiler:
    """Orchestrates the compilation of a REDLINE project."""

    def __init__(self, core_bin_path):
        self.core_bin_path = core_bin_path
        self.modules = {} # Cache for compiled modules: path -> Module

    def get_ast(self, source_file):
        """Runs the core parser and returns the AST as a JSON object."""
        try:
            result = subprocess.run(
                [str(self.core_bin_path), str(source_file), "--json-ast"],
                capture_output=True, text=True, check=True,
            )
            return json.loads(result.stdout)
        except (subprocess.CalledProcessError, json.JSONDecodeError) as e:
            print(f"❌ Error parsing {source_file.name}: {e}")
            if hasattr(e, 'stderr'): print(e.stderr)
            return None

    def compile_module_recursive(self, source_path):
        """Recursively compiles a module and its dependencies."""
        if source_path in self.modules:
            return self.modules[source_path]

        print(f"   -> Analyzing module: {source_path.name}")
        ast = self.get_ast(source_path)
        if not ast:
            return None

        module = Module(source_path, ast)
        self.modules[source_path] = module

        for import_path_str in module.get_imports():
            # Resolve import path relative to the current module's directory
            import_path = source_path.parent / import_path_str
            if not self.compile_module_recursive(import_path):
                return None # Propagate failure
        
        return module

    def generate_code(self, module, mode):
        """Generates .cpp or .hpp file for a single module."""
        output_path = module.cpp_path if mode == "cpp" else module.hpp_path
        try:
            result = subprocess.run(
                [str(self.core_bin_path), str(module.source_path), "--gen", mode],
                capture_output=True, text=True, check=True,
            )
            output_path.write_text(result.stdout)
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Error generating {mode.upper()} for {module.name}:")
            print(e.stderr)
            return False

def init_core():
    """Initializes the REDLINE compiler core."""
    print("---")
    print("🚀 Initializing REDLINE Core (Rust)...")
    try:
        subprocess.run(
            ["cargo", "build", "--release"],
            cwd=CORE_DIR, check=True, capture_output=True, text=True
        )
        print("✅ REDLINE Core initialized successfully!")
        return True
    except (subprocess.CalledProcessError, FileNotFoundError) as e:
        print(f"❌ Core initialization failed: {e}")
        if hasattr(e, 'stderr'): print(e.stderr)
        return False

def get_source_file(command_name, args):
    if len(args) < 3:
        print(f"❌ Error: Missing file path for '{command_name}' command.")
        return None
    source_file = Path(args[2]).resolve()
    if not source_file.exists():
        print(f"❌ Error: File not found: {source_file}")
        return None
    return source_file

def main():
    if len(sys.argv) < 2:
        print("Usage: python redline.py [build|parse|init|lib] ...")
        return

    command = sys.argv[1]

    if command == "init":
        init_core()
        return

    if command not in ["build", "parse", "lib"]:
        print(f"❌ Error: Unknown command '{command}'")
        return

    if not CORE_BIN.exists():
        print("⚠️ REDLINE Core binary not found. Running 'init' first...")
        if not init_core():
            print("\n❌ Aborting due to core initialization failure.")
            return
        print("\n✅ Core initialized. Continuing...")

    source_file = get_source_file(command, sys.argv)
    if not source_file: return

    compiler = Compiler(CORE_BIN)
    BUILD_DIR.mkdir(exist_ok=True)

    # --- C++ Interop Build Mode ---
    if command == "build" and source_file.suffix == ".cpp":
        # ... (same as before)
        pass

    # --- Standard REDLINE Build Mode ---
    if source_file.suffix != ".rl":
        print(f"❌ Error: Expected .rl or .cpp file, got: {source_file.suffix}")
        return

    print(f"🚀 Starting build for entry point: {source_file.name}")
    
    # DEBUG: Print AST for parse command
    if command == "parse":
        # ast = compiler.get_ast(source_file)
        # print(json.dumps(ast, indent=2))
        
        # Generate C++ code and print it
        main_module = compiler.compile_module_recursive(source_file)
        if main_module:
            compiler.generate_code(main_module, "cpp")
            print("\n--- Generated C++ Code ---")
            print(main_module.cpp_path.read_text())
        return

    main_module = compiler.compile_module_recursive(source_file)

    if not main_module:
        print("\n❌ Build failed during module analysis.")
        return

    all_modules = list(compiler.modules.values())
    
    print("\n[1/2] 🧠 Generating C++ code...")
    for module in all_modules:
        print(f"   -> Generating {module.name}.hpp")
        if not compiler.generate_code(module, "hpp"):
            print("\n❌ Build failed during C++ header generation.")
            return
        print(f"   -> Generating {module.name}.cpp")
        if not compiler.generate_code(module, "cpp"):
            print("\n❌ Build failed during C++ source generation.")
            return

    if command == "lib":
        print(f"\n[2/2] 🛠️  Compiling object files...")
        for module in all_modules:
            print(f"   -> Compiling {module.name}.o")
            try:
                subprocess.run(
                    ["g++", "-std=c++11", "-c", str(module.cpp_path), "-o", str(module.obj_path), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"],
                    check=True
                )
            except subprocess.CalledProcessError as e:
                print(f"\n❌ Compilation failed for {module.name}: {e}")
                return
        print(f"\n✅ Success! Library object files generated in: {BUILD_DIR}")
        return

    if command == "build":
        exe_output = PROJECT_ROOT / source_file.stem
        cpp_files = [m.cpp_path for m in all_modules]

        print(f"\n[2/2] 🛠️  G++ compiling and linking...")
        try:
            cmd = ["g++", "-std=c++11", *cpp_files, "-o", str(exe_output), f"-I{BUILD_DIR}", f"-I{PROJECT_ROOT}"]
            print(f"   > {' '.join(map(str, cmd))}")
            subprocess.run(cmd, check=True)
            print(f"\n🚀 Success! Compiled to: ./{exe_output.name}")
        except subprocess.CalledProcessError as e:
            print(f"\n❌ G++ compilation failed: {e}")
        finally:
            pass

if __name__ == "__main__":
    main()
