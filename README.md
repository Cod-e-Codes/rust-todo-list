# Rust To-Do List Application

A simple command-line To-Do List Manager written in Rust. This project allows users to add, list, mark as complete, and delete tasks. Task data is saved in a JSON file (`tasks.json`) to ensure persistence between sessions.

## Features
- **Add tasks**: Create a new task with a description.
- **List tasks**: View all tasks with their status (completed or not).
- **Mark tasks as complete**: Update the status of a task to completed.
- **Delete tasks**: Remove a task from the list.
- **Persistent storage**: Tasks are saved in a JSON file.

## Requirements
- [Rust](https://www.rust-lang.org/) (version 1.65.0 or later)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Cod-e-Codes/rust-todo-list.git
   cd rust-todo-list
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## Usage
When you run the application, you will be presented with a menu:

```
To-Do List Manager
1. Add Task
2. List Tasks
3. Mark Task as Complete
4. Delete Task
5. Quit
Choose an option:
```

### Add Task
1. Select `1` from the menu.
2. Enter a description for the new task.
3. The task will be added and displayed in the list of tasks.

### List Tasks
1. Select `2` from the menu.
2. View all tasks with their status (✔ for completed, ✘ for not completed).

### Mark Task as Complete
1. Select `3` from the menu.
2. Enter the ID of the task you want to mark as complete.
3. The task will be updated with a ✔ status.

### Delete Task
1. Select `4` from the menu.
2. Enter the ID of the task you want to delete.
3. The task will be removed from the list.

### Quit
1. Select `5` from the menu to save tasks and exit the application.

## Example
```
To-Do List Manager
1. Add Task
2. List Tasks
3. Mark Task as Complete
4. Delete Task
5. Quit
Choose an option: 1
Enter task description: Write a README for my Rust project
Task added successfully!

Choose an option: 2
Tasks:
1: Write a README for my Rust project [✘]

Choose an option: 3
Enter task ID to mark as complete: 1
Task marked as complete.

Choose an option: 2
Tasks:
1: Write a README for my Rust project [✔]

Choose an option: 5
Exiting...
```

## File Structure
- **`main.rs`**: The main program logic.
- **`tasks.json`**: The JSON file where tasks are saved.

## Dependencies
- `serde`: Used for serializing and deserializing task data.
- `serde_json`: Used for working with JSON data.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

