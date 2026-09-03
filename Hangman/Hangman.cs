using System;
using System.Collections.Generic;

class Hangman {
    static void Main() {
        while (true) {
            Console.WriteLine("\n=== Hangman Game ===");
            Console.WriteLine("1. Play");
            Console.WriteLine("2. Exit");
            Console.Write("Choose: ");
            
            string choice = Console.ReadLine();
            
            if (choice == "1") {
                PlayGame();
            } else if (choice == "2") {
                Console.WriteLine("Bye!");
                break;
            }
        }
    }
    
    static void PlayGame() {
        string[] words = { "hello", "world", "csharp", "hangman", "developer" };
        Random rand = new Random();
        string word = words[rand.Next(words.Length)];
        
        HashSet<char> guessed = new HashSet<char>();
        int lives = 6;
        
        while (lives > 0) {
            Console.WriteLine($"\nWord: {GetDisplay(word, guessed)}");
            Console.WriteLine($"Lives: {lives}");
            Console.Write("Guess letter: ");
            
            char guess = char.ToLower(Console.ReadLine()[0]);
            
            if (guessed.Contains(guess)) {
                Console.WriteLine("Already guessed!");
                continue;
            }
            
            guessed.Add(guess);
            
            if (!word.Contains(guess)) {
                lives--;
                Console.WriteLine("❌ Wrong!");
            } else {
                Console.WriteLine("✓ Correct!");
            }
            
            if (IsWordComplete(word, guessed)) {
                Console.WriteLine($"\n🎉 You won! Word: {word}");
                return;
            }
        }
        
        Console.WriteLine($"\n😢 Game over! Word: {word}");
    }
    
    static string GetDisplay(string word, HashSet<char> guessed) {
        string display = "";
        foreach (char c in word) {
            display += guessed.Contains(c) ? c : '_';
            display += " ";
        }
        return display;
    }
    
    static bool IsWordComplete(string word, HashSet<char> guessed) {
        foreach (char c in word) {
            if (!guessed.Contains(c)) return false;
        }
        return true;
    }
}
