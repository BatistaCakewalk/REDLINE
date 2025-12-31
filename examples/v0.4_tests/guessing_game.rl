# Number Guessing Game

print("I'm thinking of a number between 1 and 100.")
val secret_number: int = 42
var running: bool = true

while running:
    val guess_str: string = input("Take a guess: ")

    if guess_str == "quit":
        running = false
        print("Quitting.")
    else:
        val guess_num: int = to_int(guess_str)
        if guess_num < secret_number:
            print("Too low!")
        else:
            if guess_num > secret_number:
                print("Too high!")
            else:
                print("You got it!")
                running = false
