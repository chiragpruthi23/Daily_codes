#include <iostream>
using namespace std;

void printFibonacci(int n) {
    if (n <= 0) {
        cout << "Invalid number!" << endl;
        return;
    }
    
    int first = 0, second = 1;
    
    cout << "\nFibonacci series: ";
    
    for (int i = 0; i < n; i++) {
        cout << first << " ";
        int next = first + second;
        first = second;
        second = next;
    }
    cout << endl;
}

int main() {
    while (true) {
        cout << "\n=== Fibonacci Generator ===" << endl;
        cout << "1. Generate series" << endl;
        cout << "2. Exit" << endl;
        
        int choice;
        cout << "Choose: ";
        cin >> choice;
        
        if (choice == 1) {
            int count;
            cout << "How many numbers? ";
            cin >> count;
            printFibonacci(count);
        } else if (choice == 2) {
            cout << "Bye!" << endl;
            break;
        } else {
            cout << "Invalid choice!" << endl;
        }
    }
    
    return 0;
}
