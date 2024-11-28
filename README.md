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

This will add svelte-cli in your terminal

### Usage

Once the project is built, you can use the CLI like this:

```bash
svelte-cli <command> [arguments]
```

For example, to initialize a new Svelte project:

```bash
svelte-cli init my-project
```

## Dependecies

| **Crate**        | **Version** | **Description**                                                            |
|------------------|-------------|----------------------------------------------------------------------------|
| **cliclack**      | 0.3.5       | A crate for handling command-line input and output.                         |
| **log**           | 0.4.22      | Provides a flexible logging system for runtime messages.                    |
| **console**       | 0.15.8      | A crate for beautifying console output, making messages more readable.      |
| **serde_json**    | 1.0.132     | For serializing and deserializing JSON data.                                |
| **clap**          | 4.5.20      | A powerful CLI argument parser, supporting features like auto-generated help. |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```