# examples/v1.0_tests/random_test.rl

print("Testing random number generation...")

val r_int: int = random_int(1, 100)
print(f"Random int (1-100): {r_int}")

val r_float: float = random_float()
print(f"Random float (0.0-1.0): {r_float}")

print("Random test finished.")
