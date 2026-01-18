# examples/v1.0_tests/time_test.rl

print("Testing time library...")

val start_time: float = time()
print(f"Start time: {start_time}")

sleep(0.5)

val end_time: float = time()
print(f"End time:   {end_time}")

val duration: float = end_time - start_time
print(f"Slept for approximately {duration} seconds.")

print("Time test finished.")
