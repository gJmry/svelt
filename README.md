# Svelte-CLI

This project is a **Rust**-based command-line interface (CLI) tool designed to simplify interactions with **Svelte** projects.

## Features

---

## Available Commands

| **Command**                 | **Description**                                                   | **Status** |
|-----------------------------|-------------------------------------------------------------------|-----------|
| `init [NAME] [TOOLKIT]`     | Initializes a new Svelte project with the specified name.         |🚧|
| `version` / `v`             | Displays the current version of the CLI.                          |  ✅ |
| `help` / `h`                | Displays help information about the CLI and its commands.         |   ✅ |
| `install [PACKAGE] [FLAG]`  | Installs a specific package with optional flags.                  | ✅ |
| `run [ENV]`                 | Runs the project in the specified environment.                    | ✅ |
| `dev`                       | Starts a development server for live preview of the project.      |   ✅ |
| `build`                     | Compiles the Svelte project for production.                       |  ✅ |
| `lint`                      | Runs the linter to detect syntax issues in the Svelte code.       |  ✅ |
| `start`                     | Starts the project, typically used for launching the application. | ✅ |
| `update [PACKAGE]`          | Updates the specified package or the entire project.              | ✅ |
| `uninstall [PACKAGE]`       | Uninstalls a specific package from the project.                   | ✅ |
| `uninstall [PACKAGE]`       | Uninstalls a specific package from the project.                   | ✅ |
| `create [SCHEMATIC] [NAME]` | Generates a new component or file based on a schematic template.  | 🚧 |
| `add [TOOLKIT]`             | Add a toolkit in your project (check [Toolkits](#toolkits) )              | 🚧 |

### Toolkits
| Name               | Description                                                                                           | Status |
|--------------------|-------------------------------------------------------------------------------------------------------|--------|
| **UI Toolkits**     |                                                                                                       | 🚧     |
| Tailwind           | A utility-first CSS framework for rapid UI development.                                               | 🚧     |
| Bootstrap          | A popular CSS framework with a lot of pre-built components.                                           | ⛔     |
| Skeleton           | A lightweight CSS framework for minimalistic designs.                                                 | ⛔     |
| Flowbite           | A UI kit based on Tailwind CSS with ready-to-use components.                                          | ⛔     |
| **Development Toolkits** |                                                                                                   | ⛔     |
| ESLint             | A popular linter for identifying and fixing JavaScript/TypeScript issues.                             | ⛔     |
| Prettier           | An opinionated code formatter that supports multiple languages.                                       | ⛔     |
| Husky              | A tool to prevent bad `git commit` or `push` by running scripts before those actions.                 | ⛔     |
| Lint-staged        | Runs linters on pre-committed files in Git, useful with Husky for pre-commit hooks.                    | ⛔     |
| **Testing Toolkits** |                                                                                                      | ⛔     |
| Jest               | A delightful JavaScript testing framework with a focus on simplicity.                                  | ⛔     |
| Mocha              | A feature-rich JavaScript test framework running on Node.js and in the browser.                        | ⛔     |
| Cypress            | A JavaScript end-to-end testing framework for web applications.                                        | ⛔     |
| Jasmine            | A behavior-driven testing framework for JavaScript, often used with Angular.                           | ⛔     |
| **Build & Deployment Toolkits** |                                                                                               | ⛔     |
| Webpack            | A static module bundler for modern JavaScript applications.                                           | ⛔     |
| Vite               | A next-generation, fast build tool that focuses on speed and simplicity.                               | ⛔     |
| Parcel             | A zero-config, blazingly fast web application bundler.                                                | ⛔     |
| Rollup             | A JavaScript module bundler optimized for ES modules.                                                 | ⛔     |

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