---
sidebar_position: 1
---

# Initialize your project

## 🌟 Learn how to initialize your project and add packages

The `init` command is the starting point for creating a new **Svelte** project using the **Svelt** tool. It allows you to quickly set up a new project with a specified name and optional toolkit integration, making it easy to get started with Svelte development.

This command is ideal for developers who want to avoid the hassle of manual project setup. With just one command, you can initialize a new project and choose which toolkit you'd like to integrate (such as **Tailwind CSS** or **Flowbite**).

---

## 🚀 How to Use the `init` Command

### Command Syntax

```bash
svelt init [PROJECT_NAME] [TOOLKIT]
```

- **`PROJECT_NAME`**: The name of your new Svelte project.
- **`TOOLKIT`** (optional): The UI toolkit to integrate with the project (e.g., `tailwind`, `flowbite`).

### Examples

```shell
svelt init my-new-project
```

This will create a new project named **my-new-project** using the default Svelte setup

## 🛠 Options for `init`

The `init` command accepts the following arguments:

| **Argument**          | **Description**                                             | **Example**                   |
|-----------------------|-------------------------------------------------------------|-------------------------------|
| `PROJECT_NAME`        | The name of the new Svelte project to create.               | `my-new-project`              |
| `TOOLKIT` (optional)  | The toolkit you want to integrate with your project.        | `tailwind`, `flowbite`, etc.   |

---

## ⚙️ What Happens After Running `init`?

Once you run the `init` command, the CLI will automatically:

1. **Create a new directory**: A new folder with the specified `PROJECT_NAME` will be created in the current working directory.

2. **Generate the basic project structure**:
    - The default files for a Svelte project will be created (e.g., `src`, `public`, `package.json`, etc.).
    - The necessary dependencies will be installed using **npm** or **yarn**.

3. **Integrate the specified toolkit (if any)**:
    - If you chose a toolkit like **Tailwind CSS**, the CLI will automatically configure the project with the necessary dependencies, configuration files, and example styles.
    - If no toolkit is specified, the default setup will be used without additional configuration.

4. **Final Setup**:
    - After setup, you can navigate into your newly created project directory and start developing.

   Example:
   ```bash
   cd my-new-project
   ```

---

## 🧳 Available Toolkits for `init`

When using the `init` command, you can specify one of the following toolkits:

### UI Toolkits

- **Tailwind CSS**: A utility-first CSS framework for rapid UI development.
    - Command Example: `svelt init my-tailwind-project tailwind`
- **Flowbite**: A UI kit based on Tailwind CSS with ready-to-use components.
    - Command Example: `svelt init my-flowbite-project flowbite`

*Note: More toolkits will be added in future updates. Check the documentation for the latest options.*

---

## ❓ Troubleshooting

### I ran the `init` command, but my project isn’t working. What should I do?

- Ensure you have **Node.js** and **npm** installed. These are required to manage dependencies for the Svelte project.
- Verify your `svelt` installation by running `svelt version` to make sure it’s correctly installed.
- If you chose a toolkit, check that the respective dependencies are installed correctly by inspecting your `package.json`.

### How do I update my project after initializing it?

You can update your project dependencies by using the `update` command:

```bash
svelt update
```

This will update the project dependencies to the latest versions as per your configuration.

---