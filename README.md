# Svelte-CLI

This project is a **Rust**-based command-line interface (CLI) tool designed to simplify interactions with **Svelte** projects.

## Features

| Feature                  | Description |
| ------------------------ | ----------- |
| `init`                    | Initializes a new Svelte project with a basic structure. |
| `dev`                     | Starts a development server to see changes live. |
| `build`                   | Compiles the Svelte project for production. |
| `serve`                   | Serves the built project in a production environment. |
| `add <component>`         | Adds a specific Svelte component to your project. |
| `update`                  | Updates the project's dependencies. |
| `lint`                    | Runs a linter on the Svelte code to detect syntax issues. |
| `test`                    | Runs unit tests to verify project logic. |

### TODO



## Installation

### Prerequisites

Before installing and using this CLI, make sure you have the following tools installed:

- **Rust**: Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).
- **Node.js** and **npm**: Svelte requires Node.js for dependency management and the development server.

### Install the CLI

Clone this repository and build the project using the following commands:

```bash
git clone https://github.com/gJmry/svelte-cli
cd svelte-cli
cargo install --path .
```

This will add svelte-cli in your terminal, you can use it as 
```
svelte-cli <command> <args>
```

### Usage

Once the project is built, you can use the CLI like this:

```bash
./cli-rust-svelte <command> [arguments]
```

For example, to initialize a new Svelte project:

```bash
./cli-rust-svelte init my-project
```

## Crates Used

Here are the main Rust crates used in this project:

- **clap**: Provides an interface for parsing command-line arguments.
- **serde** and **serde_json**: Used for serialization and deserialization of data, essential for configuration handling.
- **tokio**: Asynchronous runtime for handling file I/O and networking operations efficiently.
- **reqwest**: A simple HTTP client for making requests to external services (e.g., for dependency management or fetching templates).
- **anyhow**: Error handling library that simplifies working with errors.

## Contributing

Contributions are welcome! If you want to report a bug or suggest a feature, feel free to open an issue. If you want to contribute code, please fork the repository and create a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```