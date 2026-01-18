# examples/v1.0_tests/loop_control_test.rl

print("Testing loop control...")

print("1. Skipping even numbers (continue):")
for i in 0..10:
    if i == 0:
        continue

    # Simple check
    if i == 2:
        continue
    if i == 4:
        continue
    if i == 6:
        continue
    if i == 8:
        continue

    print(i)

print("2. Breaking at 5 (break):")
for i in 0..10:
    if i == 5:
        print("Breaking!")
        break
    print(i)

print("Loop control test finished.")
