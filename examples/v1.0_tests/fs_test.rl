# examples/v1.0_tests/fs_test.rl

print("Testing file system operations...")

val test_dir: string = "test_temp_dir"
val test_file: string = test_dir + "/test.txt"

# Clean up from previous runs, if any
if exists(test_dir):
    if exists(test_file):
        remove(test_file)
    remove(test_dir)

# Test mkdir
mkdir(test_dir)
if exists(test_dir):
    print("SUCCESS: Directory created.")
else:
    print("FAILURE: Directory not created.")

# Test write_file and exists
write_file(test_file, "hello")
if exists(test_file):
    print("SUCCESS: File created.")
else:
    print("FAILURE: File not created.")

# Test list_dir
val files: list[string] = list_dir(test_dir)
if len(files) == 1:
    if files[0] == "test.txt":
        print("SUCCESS: list_dir found the correct file.")
    else:
        print("FAILURE: list_dir found the wrong file.")
else:
    print("FAILURE: list_dir found wrong number of files.")

# Test remove
remove(test_file)
if exists(test_file):
    print("FAILURE: File not removed.")
else:
    print("SUCCESS: File removed.")

remove(test_dir)
if exists(test_dir):
    print("FAILURE: Directory not removed.")
else:
    print("SUCCESS: Directory removed.")

print("File system test finished.")
