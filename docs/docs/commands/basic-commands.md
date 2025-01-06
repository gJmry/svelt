---
sidebar_position: 2
---

# Basic commands

Here are the basic commands available in the CLI. Most of them are essentially "enhanced" npm commands, so feel free to explore them! 🚀

### `version` / `v`
Displays the current version of the CLI.

```bash
svelt version
```

---

### `help` / `h`
Provides detailed help and usage information for the CLI and its commands.

```bash
svelte help
```

You can add a specific command after `help` to view its usage details. For example:

```bash
svelte help build
```

---

### `install [PACKAGE] [FLAG]`
Installs a specified package with optional flags.

```bash
svelt install some-package --save-dev
```


---

### `run [ENV]`
Runs the project in a specified environment (e.g., `production` or `development`).

```bash
svelt run production
```

---

### `dev`
Starts a development server for live preview of the project. Useful for testing your project during development.

```bash
svelt dev
```


---

### `build`
Compiles the svelte project for production. This will generate optimized output files for deployment.

```bash
svelt build
```


---

### `lint`
Runs a linter to detect and report syntax or stylistic issues in the svelt code.

```bash
svelt lint
```

---

### `start`
Starts the project, typically used for launching the application after building.

```bash
svelt start
```

---

### `update [PACKAGE]`
Updates a specific package or all dependencies in the project.

```bash
svelt update
svelt update some-package
```


---

### `uninstall [PACKAGE]`
Removes a specified package from your project.

```bash
svelt uninstall some-package
```


