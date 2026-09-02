#include <iostream>
using namespace std;

bool isPrime(int num) {
    if (num <= 1) return false;
    if (num == 2) return true;
    if (num % 2 == 0) return false;
    
    for (int i = 3; i * i <= num; i += 2) {
        if (num % i == 0) return false;
    }
    return true;
}

int main() {
    while (true) {
        cout << "\n=== Prime Number Checker ===" << endl;
        cout << "1. Check number" << endl;
        cout << "2. Exit" << endl;
        
        int choice;
        cout << "Choose: ";
        cin >> choice;
        
        if (choice == 1) {
            int num;
            cout << "Enter number: ";
            cin >> num;
            
            if (isPrime(num)) {
                cout << "\n✓ " << num << " is PRIME!" << endl;
            } else {
                cout << "\n✗ " << num << " is NOT prime!" << endl;
            }
        } else if (choice == 2) {
            cout << "Bye!" << endl;
            break;
        } else {
            cout << "Invalid choice!" << endl;
        }
    }
    
    return 0;
}
