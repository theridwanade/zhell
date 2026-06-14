# Zhell 🦀

Zhell is a lightweight, custom command-line shell written in Rust. It is capable of handling daily tasks by executing external programs, managing built-in commands, and providing a robust user experience with persistent history and tab completion.

## ✨ Features

* **External Command Execution:** Seamlessly run standard CLI tools available in your `$PATH`.
* **Shell Built-ins:** Native, fast implementations of essential commands:
* `cd` (supports `cd ~`, `cd -`, and relative/absolute paths)
* `pwd`, `echo`, `exit`, `history`, and `type`
* `type` (identifies whether a command is a shell built-in or an external binary)


* **I/O Redirection:** Built-in support for standard output and error redirection ( `>`, `>>`, `2>`, `2>>`).
* **Custom Lexing:** Accurately processes arguments with support for single quotes (`'`), double quotes (`"`), and backslash escaping (`\`).
* **Command History:** Navigate your past commands using the up and down arrow keys (history is persisted locally in `history.txt`).
* **Tab Completion:** Context-aware auto-completion for both shell commands and file paths.
* **Dynamic Prompt:** Displays a bash-style contextual prompt format (`user@hostname:cwd$ `), automatically aliasing your home directory to `~`.

## 🛠️ Tech Stack

* **Rust:** Built entirely using Rust for memory safety and performance.
* **[rustyline](https://crates.io/crates/rustyline):** Handles the shell editor interface, input validation, auto-completion, and history management.
* **[hostname](https://crates.io/crates/hostname):** Used for reliably reading the device hostname to format the prompt.

## 🚀 Getting Started

### Prerequisites

Make sure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/zhell.git
cd zhell

```


2. Build the project:
```bash
cargo build --release

```


3. Run the shell:
```bash
cargo run
# or run the executable directly
./target/release/zhell

```



## 📖 Usage Examples

```bash
# Check if a command is built-in or external
user@ubuntu:~$ type ls
ls is /usr/bin/ls

# Redirect output to a file
user@ubuntu:~$ echo "Hello Zhell" > greeting.txt

# Append error output to a file
user@ubuntu:~$ some_failing_command 2>> errors.log

# Navigate using previous working directory
user@ubuntu:~$ cd -

```

## 💡 Motivation & Backstory

The original intention behind Zhell was to build a fast, 100% dependency-free command-line interface from scratch to dive deep into low-level programming and system calls.

However, as the project evolved to handle the realities of daily use, practical features like command history and tab completion became necessary. To speed up development and ensure a smooth, stable user experience without reinventing the wheel, powerful crates like `rustyline` and `hostname` were integrated into the architecture.
