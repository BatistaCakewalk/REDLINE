# examples/v1.0_tests/fstring_test.rl

print("Testing f-strings...")

val name: string = "REDLINE"
val version: float = 1.0

# Simple interpolation
print(f"Welcome to {name} v{version}!")

# Expression interpolation
val a: int = 10
val b: int = 20
print(f"The sum of {a} and {b} is {a + b}.")

# Function call interpolation
def get_greeting() -> string:
    return "Hello"

print(f"{get_greeting()}, World!")

print("F-string test finished.")
