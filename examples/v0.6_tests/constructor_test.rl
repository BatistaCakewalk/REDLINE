# examples/v0.6_tests/constructor_test.rl

class Person:
    var name: string = ""
    var age: int = 0

    # The constructor for the Person class.
    # It takes a name and an age and initializes the member variables.
    def init(new_name: string, new_age: int):
        this.name = new_name
        this.age = new_age

    # A method to print a greeting.
    def greet():
        print("Hello, my name is " + this.name + " and I am " + to_string(this.age) + " years old.")

# --- Main execution starts here ---

# Create an instance of the Person class using the constructor.
var p: Person = Person("Alex", 30)

# Call the object's method to verify it was constructed correctly.
p.greet()
