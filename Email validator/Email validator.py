import re

def validate_email(email):
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
    return re.match(pattern, email) is not None

def main():
    while True:
        print("\n=== Email Validator ===")
        print("1. Validate email")
        print("2. Exit")
        
        choice = input("Choose: ")
        
        if choice == "1":
            email = input("Enter email: ")
            if validate_email(email):
                print("✓ Valid email!")
            else:
                print("✗ Invalid email!")
        elif choice == "2":
            print("Bye!")
            break
        else:
            print("Invalid choice!")

if __name__ == "__main__":
    main()
