# examples/v0.6_tests/class_test.rl

# Define a class for a 2D vector.
class Vector2:
    # Member variables
    var x: float = 0.0
    var y: float = 0.0

    # A method to calculate the length of the vector.
    # Note the use of the 'this' keyword to access member variables.
    def length() -> float:
        return sqrt(this.x * this.x + this.y * this.y)

# --- Main execution starts here ---

# Create an instance of the Vector2 class.
var my_vec: Vector2 = Vector2()

# Set the member variables of the instance.
my_vec.x = 3.0
my_vec.y = 4.0

# Call the object's method.
val len: float = my_vec.length()

print("Vector x:")
print(my_vec.x)
print("Vector y:")
print(my_vec.y)
print("Vector length:")
print(len)
