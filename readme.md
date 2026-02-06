# Pomonote

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Pomonote** is a terminal-based TUI application for managing your tasks, combined with a Pomodoro-inspired 25-minute timer to help you focus. It's designed to be a lightweight, distraction-free tool for developers and anyone who loves working in the terminal.

The goal of Pomonote is to provide a simple yet effective way to manage your to-do list without leaving your terminal, keeping you in the zone.

![Pomonote Screenshot](https://github.com/willnjl/pomonote/blob/main/pomonote.png?raw=true)
_(A placeholder for a screenshot of Pomonote in action)_

## Features

- **Terminal-Based Interface**: No need to switch contexts. Manage your tasks right where you code.
- **Pomodoro Timer**: A 25-minute timer to help you stay focused on your tasks.
- **Simple Task Management**: Add, remove, start, stop, and complete tasks with simple commands.
- **Lightweight and Fast**: Built with Rust for performance.
- **Persistent Storage**: Your tasks are saved in a lightweight JSON file, so you don't lose them.

## Installation

You can install Pomonote using the provided Zsh script.

1.  **Clone the repository:**

    ```sh
    git clone https://github.com/willnjl/pomonote.git
    cd pomonote
    ```

2.  **Run the installation script:**
    This script will build the application and install it into `~/bin/pomonote`.

    ```sh
    ./install.zsh
    ```

3.  **Ensure `~/bin` is in your PATH:**
    Make sure `~/bin` is in your shell's `PATH` variable. You can add it to your `.zshrc` or `.bashrc` file:
    ```sh
    export PATH="$HOME/bin:$PATH"
    ```

## Usage

To start Pomonote, simply run:

```sh
pomonote
```

The application will launch in your terminal.

## Commands

Pomonote uses a simple command-based interface for task management.

| Command    | Aliases     | Description                              | Example                  |
| ---------- | ----------- | ---------------------------------------- | ------------------------ |
| `add`      |             | Adds a new task.                         | `add "Fix the main bug"` |
| `remove`   | `rm`        | Removes one or more tasks by their ID.   | `remove 1 3`             |
| `start`    |             | Starts the timer for one or more tasks.  | `start 2`                |
| `stop`     |             | Stops the timer for one or more tasks.   | `stop 2`                 |
| `complete` | `done`      | Marks one or more tasks as complete.     | `complete 1`             |
| `toggle`   | `{id}`      | Toggles the status of one or more tasks. | `1 2`                    |
| `quit`     | `exit`, `q` | Exits the application.                   | `q`                      |

## Development

To build and run the application for development:

```sh
cargo run
```

To watch for changes and automatically rebuild:

```sh
cargo watch -x "test" -x "run"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
