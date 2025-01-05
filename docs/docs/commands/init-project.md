---
sidebar_position: 1
---

# Initialize your project

## 🌟 Learn how to initialize your project and add packages

The `init` command is the starting point for creating a new **Svelte** project using the **Svelt** tool. It allows you to quickly set up a new project with a specified name and optional toolkit integration, making it easy to get started with Svelte development.

---

## 🚀 How to Use the `init` Command

### Command Syntax

```bash
svelt init [PROJECT_NAME] [TOOLKIT]
```

- **`PROJECT_NAME`**: The name of your new Svelte project.
- **`TOOLKIT`** (optional): The toolkit to integrate with the project (e.g., `tailwind`, `flowbite`).

### Examples

```shell
svelt init my-new-project
```

This will create a new project named **my-new-project** using the default Svelte setup

## ⚙️ What Happens After Running `init`?

Once you run the `init` command, the CLI will automatically:

1. **Create a new directory**: A new folder with the specified `PROJECT_NAME` will be created in the current working directory.

2. **Generate the basic project structure**:
    - The default files for a Svelte project will be created (e.g., `src`, `public`, `package.json`, etc.).
    - The necessary dependencies will be installed using **npm** or **yarn**.

3. **Integrate the specified toolkit (if any)**:
    - If you chose a toolkit, the CLI will automatically configure the project with the necessary dependencies, configuration files, and example styles.
    - If no toolkit is specified, the default setup will be used without additional configuration.

4. **Final Setup**:
    - After setup, you can navigate into your newly created project directory and start developing.

   Example:
   ```bash
   cd my-new-project
   ```

## ❓ Troubleshooting

### I ran the `init` command, but my project isn’t working. What should I do?

- Ensure you have **Node.js** and **npm** installed. These are required to manage dependencies for the Svelte project.
- Verify your `svelt` installation by running `svelt version` to make sure it’s correctly installed.
- If you chose a toolkit, check that the respective dependencies are installed correctly by inspecting your `package.json`.

---