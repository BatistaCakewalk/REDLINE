# Test for nested loops and variable mutation.
# This should print a right-angle triangle of asterisks.

print("Testing nested loops...")

for i in 0..5:
    var line: string = ""
    var j: int = 0
    while j <= i:
        line = line + "*"
        j = j + 1
    print(line)

print("Test complete.")
