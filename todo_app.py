import json
import os

TODO_FILE = "todos.json"

def load_todos():
    if os.path.exists(TODO_FILE):
        with open(TODO_FILE, "r") as f:
            return json.load(f)
    return []

def save_todos(todos):
    with open(TODO_FILE, "w") as f:
        json.dump(todos, f)

def add_todo(task):
    todos = load_todos()
    todos.append({"task": task, "done": False})
    save_todos(todos)
    print(f"✓ Added: {task}")

def view_todos():
    todos = load_todos()
    if not todos:
        print("No todos!")
        return
    for i, todo in enumerate(todos, 1):
        status = "✓" if todo["done"] else "✗"
        print(f"{i}. [{status}] {todo['task']}")

def mark_done(index):
    todos = load_todos()
    if 0 <= index < len(todos):
        todos[index]["done"] = True
        save_todos(todos)
        print(f"✓ Marked done: {todos[index]['task']}")

def delete_todo(index):
    todos = load_todos()
    if 0 <= index < len(todos):
        task = todos.pop(index)
        save_todos(todos)
        print(f"✗ Deleted: {task['task']}")

def main():
    while True:
        print("\n--- TODO APP ---")
        print("1. Add todo")
        print("2. View todos")
        print("3. Mark done")
        print("4. Delete todo")
        print("5. Exit")
        
        choice = input("Choose: ")
        
        if choice == "1":
            task = input("Enter task: ")
            add_todo(task)
        elif choice == "2":
            view_todos()
        elif choice == "3":
            view_todos()
            idx = int(input("Enter number: ")) - 1
            mark_done(idx)
        elif choice == "4":
            view_todos()
            idx = int(input("Enter number: ")) - 1
            delete_todo(idx)
        elif choice == "5":
            print("Bye!")
            break

if __name__ == "__main__":
    main()
