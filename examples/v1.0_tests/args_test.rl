# examples/v1.0_tests/args_test.rl

print("Testing command line arguments...")

print(f"Received {len(args)} arguments.")

for i in 0..len(args):
    print(f"Arg {i}: {args[i]}")

print("Args test finished.")
