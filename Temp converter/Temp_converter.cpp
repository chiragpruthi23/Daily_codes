#include <iostream>
using namespace std;

int main() {
    while (true) {
        cout << "\n=== Temperature Converter ===" << endl;
        cout << "1. Celsius to Fahrenheit" << endl;
        cout << "2. Fahrenheit to Celsius" << endl;
        cout << "3. Exit" << endl;
        
        int choice;
        cout << "Choose: ";
        cin >> choice;
        
        if (choice == 1) {
            double celsius;
            cout << "Enter Celsius: ";
            cin >> celsius;
            double fahrenheit = (celsius * 9/5) + 32;
            cout << celsius << "°C = " << fahrenheit << "°F" << endl;
        } else if (choice == 2) {
            double fahrenheit;
            cout << "Enter Fahrenheit: ";
            cin >> fahrenheit;
            double celsius = (fahrenheit - 32) * 5/9;
            cout << fahrenheit << "°F = " << celsius << "°C" << endl;
        } else if (choice == 3) {
            cout << "Bye!" << endl;
            break;
        } else {
            cout << "Invalid choice!" << endl;
        }
    }
    
    return 0;
}
