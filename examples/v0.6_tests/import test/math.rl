# examples/v0.6_tests/math.rl

# A public function that can be imported by other modules.
pub def add(a: int, b: int) -> int:
    return a + b

# A private function that is only visible within this module.
def subtract(a: int, b: int) -> int:
    return a - b
