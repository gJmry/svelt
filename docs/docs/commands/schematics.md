---
sidebar_position: 5
---

# Schematics

In SvelteKit, you have a variety of schematics, each serving a specific purpose. With the **Svelt CLI**, you can easily generate these schematics using straightforward commands to streamline your development process. 🚀

### General Syntax

```bash
svelt create [SCHEMATIC] [NAME] [OPTIONS]
```

- **SCHEMATIC**: The type of file or structure you want to generate (e.g., `component`, `store`). 🧩
- **NAME**: The name of the file or feature you want to create. ✏️
- **OPTIONS**: Additional options to customize the generated output. ⚙️

---

### 🗂️ Create a Store

```bash
svelt create store myStore
```

- 📂 Generates a new file `myStore.js` in the `/stores` directory.
- Default format is JavaScript; add the `--ts` flag to generate TypeScript:

```bash
svelt create store myStore --ts
```

---

### 🛠️ Create a Component

```bash
svelt create component Button
```

- 📁 Creates a `Button.svelte` file in the `/components` directory.

---

### 🌐 Generate a Page

```bash
svelt create page about
```

- 🗺️ Creates a new `+page.svelte` file in `/routes/about/`.

To include route parameters:

```bash
svelt create page [id]
```

- 🛤️ This generates `/routes/[id]/+page.svelte`.

---

### 📐 Add a Layout

```bash
svelt create layout blog
```

- 🖼️ Adds a `+layout.svelte` file to the blog route.

---

### 🛑 Add an Error

```bash
svelt create error blog
```

- ❌ Adds a `+error.svelte` file to the blog route.

---

### ⚙️ Create a Service

```bash
svelt create service commands
```

- 🛠️ Adds a `commands.js` file to the `/service` directory.
- Default format is JavaScript; add the `--ts` flag to generate TypeScript:

```bash
svelt create service commands --ts
```

---

### 🖥️ Add a Server-Side File

```bash
svelt create server blog
```

- 🛡️ Creates a `+page.server.ts` file for the blog route.

---

### 🎨 Generate a CSS File

```bash
svelt create css styles
```

- 🖌️ Creates a `styles.css` file in the `/styles` directory.
- Use the `--sass` or `--scss` flag to generate `styles.sass` or `styles.scss`:

```bash
svelt create css styles --scss
```