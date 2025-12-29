#!/usr/bin/env python3
import os
import subprocess
import sys
from pathlib import Path


def main():
    if len(sys.argv) < 2:
        print("Usage: python redline.py <file.rl>")
        return

    # 1. Setup Absolute Paths
    project_root = Path(__file__).parent.resolve()
    source_file = Path(sys.argv[1]).resolve()

    # Path to your new Rust binary
    core_bin = project_root / "redline-core" / "target" / "release" / "redline-core"
    cpp_output = project_root / "output.cpp"
    exe_output = project_root / source_file.stem

    if not core_bin.exists():
        print(f"❌ Error: Rust binary not found at {core_bin}")
        print("Please run 'cargo build --release' inside redline-core first.")
        return

    try:
        # Step 1: Run the Rust Core (Source .rl -> C++ code)
        print(f"[1/2] 🧠 REDLINE Core parsing: {source_file.name}")
        with open(cpp_output, "w") as f:
            subprocess.run([str(core_bin), str(source_file)], stdout=f, check=True)

        # Step 2: Compile with G++
        # We add -I so G++ knows exactly where the stdlib is
        print(f"[2/2] 🛠️  G++ compiling: {exe_output.name}")
        subprocess.run(
            [
                "g++",
                "-std=c++11",
                str(cpp_output),
                "-o",
                str(exe_output),
                f"-I{project_root}",  # This helps find the stdlib
            ],
            check=True,
        )

        print(f"\n🚀 Success! Compiled to: ./{exe_output.name}")

        # Optional: Clean up the temp .cpp file
        # cpp_output.unlink()

    except subprocess.CalledProcessError as e:
        print(f"\n❌ Build Failed: {e}")


if __name__ == "__main__":
    main()
