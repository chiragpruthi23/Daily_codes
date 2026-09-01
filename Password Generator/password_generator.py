import random
import string

def generate_password(length=12):
    characters = string.ascii_letters + string.digits + string.punctuation
    password = ''.join(random.choice(characters) for _ in range(length))
    return password

def main():
    while True:
        print("\n=== Password Generator ===")
        print("1. Generate password")
        print("2. Exit")
        
        choice = input("Choose: ")
        
        if choice == "1":
            try:
                length = int(input("Password length (default 12): ") or "12")
                if length < 4:
                    print("Length must be at least 4!")
                    continue
                password = generate_password(length)
                print(f"\n🔐 Generated: {password}\n")
            except ValueError:
                print("Invalid input!")
        elif choice == "2":
            print("Bye!")
            break
        else:
            print("Invalid choice!")

if __name__ == "__main__":
    main()
