def convert_length():
    print("\n--- Length Converter ---")
    print("1. Meters to Feet")
    print("2. Feet to Meters")
    print("3. KM to Miles")
    
    choice = input("Choose: ")
    value = float(input("Enter value: "))
    
    if choice == "1":
        result = value * 3.28084
        print(f"{value}m = {result:.2f}ft")
    elif choice == "2":
        result = value / 3.28084
        print(f"{value}ft = {result:.2f}m")
    elif choice == "3":
        result = value * 0.621371
        print(f"{value}km = {result:.2f}mi")

def convert_weight():
    print("\n--- Weight Converter ---")
    print("1. KG to Pounds")
    print("2. Pounds to KG")
    
    choice = input("Choose: ")
    value = float(input("Enter value: "))
    
    if choice == "1":
        result = value * 2.20462
        print(f"{value}kg = {result:.2f}lbs")
    elif choice == "2":
        result = value / 2.20462
        print(f"{value}lbs = {result:.2f}kg")

def main():
    while True:
        print("\n=== Unit Converter ===")
        print("1. Length")
        print("2. Weight")
        print("3. Exit")
        
        choice = input("Choose: ")
        
        if choice == "1":
            convert_length()
        elif choice == "2":
            convert_weight()
        elif choice == "3":
            print("Bye!")
            break
        else:
            print("Invalid choice!")

if __name__ == "__main__":
    main()
