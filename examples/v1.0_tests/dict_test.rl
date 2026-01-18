# examples/v1.0_tests/dict_test.rl

print("Testing dictionaries...")

var scores: dict[string, int] = {
    "Alice": 100,
    "Bob": 85,
    "Charlie": 92
}

print("Alice's score: " + to_string(scores["Alice"]))
print("Bob's score: " + to_string(scores["Bob"]))

# Modify a value
scores["Bob"] = 90
print("Bob's new score: " + to_string(scores["Bob"]))

print("Dictionary test finished.")
