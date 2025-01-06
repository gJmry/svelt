---
sidebar_position: 5
---

# Schematics Guide

The **Svelt CLI** is your tool for quickly generating files and structures in SvelteKit projects. Whether you're creating pages, stores, or services, the CLI provides a fast and efficient way to set up your project with consistent patterns. 🚀

## General Syntax 🛠️

```bash
svelt create [SCHEMATIC] [NAME] [OPTIONS]
```

- **SCHEMATIC**: The type of file or structure to generate (e.g., `page`, `store`, `service`). 🧩
- **NAME**: The name of the feature you want to create. ✏️
- **OPTIONS**: Additional flags to customize the generated files (e.g., `layout`, `error`, `css`, etc.). ⚙️

---

## Creating Pages and Additional Files 🌐

### Basic Page Creation

```bash
svelt create page blog
```

- 🗺️ Creates a `+page.svelte` file in the `/routes/blog` directory.
- Ideal for setting up a standalone page.

### Adding Multiple Files to a Route

You can extend a route with additional files by including multiple schematic names in a single command. For example:

```bash
svelt create page blog layout error ts css
```

This command generates the following in `/routes/blog`:

- `+page.svelte`: The main page file.
- `+layout.svelte`: A layout file for the route.
- `+error.svelte`: An error handling file.
- `+page.ts`: A TypeScript server-side file.
- `+page.css`: A CSS file for the page.

For the `svelt create page` command, you can use the following options to add specific files to your route:

- `layout`
- `error`
- `script`
- `ts`
- `js`
- `css`
- `scss`
- `sass`
- `server`


---

## Generating Other Files 📁

### Creating a Store

```bash
svelt create store myStore
```

- 📂 Generates `myStore.js` in the `/stores` directory.

---

### Creating a Component

```bash
svelt create component Button
```

- 📁 Creates `Button.svelte` in the `/components` directory.

---

### Creating a Service

```bash
svelt create service api
```

- 🛠️ Generates an `api.js` file in the `/services` directory.

---

### File Type Defaults

- Pages, layouts, and error files can include additional options like `ts` (TypeScript) or `css` (CSS).
- Stores and services are always generated as `.js` files.
- Components are always `.svelte` files.

---

## Examples 📚

### Example 1: Simple Page Creation

```bash
svelt create page about
```

- Creates:
    - `/routes/about/+page.svelte`

---

### Example 2: Full Route Setup

```bash
svelt create page blog layout error ts css
```

- Creates:
    - `/routes/blog/+page.svelte`
    - `/routes/blog/+layout.svelte`
    - `/routes/blog/+error.svelte`
    - `/routes/blog/+page.ts`
    - `/routes/blog/+page.css`

---

### Example 3: Adding a Store

```bash
svelt create store userStore
```

- Creates:
    - `/stores/userStore.js`

---

### Example 4: Creating a Component

```bash
svelt create component Header
```

- Creates:
    - `/components/Header.svelte`

---

### Example 5: Adding a Service

```bash
svelt create service fetchData
```

- Creates:
    - `/services/fetchData.js`

---