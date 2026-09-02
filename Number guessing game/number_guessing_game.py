import random

def play_game():
    secret = random.randint(1, 100)
    attempts = 0
    guessed = False
    
    print("🎯 Guess the number (1-100)!")
    
    while not guessed:
        try:
            guess = int(input("\nGuess: "))
            
            if guess < 1 or guess > 100:
                print("Enter number between 1-100!")
                continue
            
            attempts += 1
            
            if guess == secret:
                print(f"🎉 Correct! Number was {secret}")
                print(f"Attempts: {attempts}")
                guessed = True
            elif guess < secret:
                print("⬆️ Too low!")
            else:
                print("⬇️ Too high!")
                
        except ValueError:
            print("Invalid! Enter a number.")

def main():
    while True:
        print("\n=== Number Guessing Game ===")
        print("1. Play")
        print("2. Exit")
        
        choice = input("Choose: ")
        
        if choice == "1":
            play_game()
        elif choice == "2":
            print("Bye!")
            break
        else:
            print("Invalid choice!")

if __name__ == "__main__":
    main()
