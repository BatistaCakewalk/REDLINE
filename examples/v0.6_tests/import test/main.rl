# examples/v0.6_tests/main.rl

# Import the math module.
# The path is relative to this file.
import "math.rl"

# The main entry point of the program.
val result: int = add(10, 5)
print("The result of add(10, 5) is:")
print(result)

# The following line would cause a compile error because `subtract` is not public.
# val result2: int = subtract(10, 5)
# print(result2)
