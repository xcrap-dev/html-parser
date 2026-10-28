<div align="center">

# 🕷️ @xcrap/html-parser

**A blazing-fast HTML parser for Node.js, powered by Rust and NAPI-RS**

[![npm version](https://img.shields.io/npm/v/@xcrap/html-parser?style=flat-square&color=e05d44)](https://www.npmjs.com/package/@xcrap/html-parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](./LICENSE)
[![Node.js >= 18](https://img.shields.io/badge/Node.js-%3E%3D18-339933?style=flat-square&logo=node.js)](https://nodejs.org)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[@xcrap/html-parser](https://www.npmjs.com/package/@xcrap/html-parser) is an **experimental** HTML parsing library written in **Rust**, exposed to Node.js through the [NAPI-RS](https://napi.rs/) framework. It is designed to be **fast**, **lightweight**, and to support both **CSS selectors** and **XPath** queries — with built-in support for result limits and element nesting.

Although part of the [Xcrap](https://github.com/Xcrap-Cloud) scraping ecosystem, this library can be used as a **standalone package** in any Node.js project.

</div>

---

## 📋 Table of Contents

- [✨ Features](#-features)
- [⚡ Performance](#-performance)
- [📦 Installation](#-installation)
- [🚀 Quick Start](#-quick-start)
- [📖 API Reference](#-api-reference)
  - [`HtmlParser` / `HTMLParser`](#htmlparser--htmlparser)
  - [`HTMLElement`](#htmlelement)
  - [`css()` and `xpath()`](#css-and-xpath)
  - [Types](#types)
- [🔍 Usage Examples](#-usage-examples)
  - [CSS Selectors](#css-selectors)
  - [XPath Queries](#xpath-queries)
  - [Navigating Nested Elements](#navigating-nested-elements)
  - [Working with Attributes](#working-with-attributes)
- [🏗️ Architecture](#️-architecture)
- [🛠️ Development](#️-development)
- [🤝 Contributing](#-contributing)
- [📝 License](#-license)

---

## ✨ Features

- **⚡ Blazing Fast** — Core parsing done in Rust; significantly faster than JS-based parsers at instance initialization.
- **🎯 Dual Query Support** — Query elements using both **CSS selectors** (via `scraper`) and **XPath** expressions (via `sxd-xpath`).
- **🦥 Lazy Loading** — Internal CSS and XPath engines are only initialized when first needed, reducing unnecessary overhead.
- **🔢 Built-in Limits** — Pass a `limit` option to `selectMany` to cap the number of returned elements.
- **🌲 Element Traversal** — Navigate nested elements with `selectFirst` / `selectMany`, and move through the tree with `parent`, `children`, `nextSibling`, `closest()` and more.
- **🔒 Type-Safe** — Fully typed TypeScript declarations included (`index.d.ts`).
- **🖥️ Platform Support** — Pre-built native binary currently available for **Windows x64** only. Other platforms require compilation from source (see [Development](#️-development)).

---

## ⚡ Performance

Benchmarks below compare parser **initialization speed** (instantiation time per file):

```
@xcrap/html-parser    :  0.246214 ms/file  ±  0.136808  ✅ Fastest
html-parser           : 36.825500 ms/file  ± 28.855100
htmljs-parser         :  0.501577 ms/file  ±  1.210800
html-dom-parser       :  2.180280 ms/file  ±  1.796170
html5parser           :  1.674640 ms/file  ±  1.222790
cheerio               :  8.679980 ms/file  ±  6.328520
parse5                :  4.821180 ms/file  ±  2.668220
htmlparser2           :  1.497390 ms/file  ±  1.398040
htmlparser            : 16.171200 ms/file  ± 109.076000
high5                 :  2.982290 ms/file  ±  1.927480
node-html-parser      :  2.901670 ms/file  ±  1.908040
```

> Benchmarks sourced from [node-html-parser repository](https://github.com/taoqf/node-html-parser).

The performance advantage comes from lazy loading: the internal `Html` (CSS engine) and `Package` (XPath engine) instances are only initialized on first use and reused across subsequent calls on the same parser instance.

---

## 📦 Installation

Install via your preferred package manager:

```bash
# npm
npm install @xcrap/html-parser

# yarn
yarn add @xcrap/html-parser

# pnpm
pnpm add @xcrap/html-parser
```

**Requirements:**
- Node.js **>= 18.0.0**

Native binaries are pre-built and distributed for the following platforms:

| Platform         | Architecture | Support         |
|------------------|--------------|-----------------|
| Windows          | x64          | ✅ Pre-built    |
| macOS            | x64          | 🔧 Build from source |
| macOS            | ARM64        | 🔧 Build from source |
| Linux            | x64 (GNU)    | 🔧 Build from source |

> **⚠️ Note:** Currently only the **Windows x64** binary is pre-built and included in the published package. Users on other platforms must compile the native addon locally — see the [Development](#️-development) section for instructions.

---

## 🚀 Quick Start

```ts
import { HtmlParser, css, xpath } from "@xcrap/html-parser"

const html = `
  <html>
    <body>
      <h1 class="title">Hello World</h1>
      <ul>
        <li class="item">Item 1</li>
        <li class="item">Item 2</li>
        <li class="item">Item 3</li>
      </ul>
    </body>
  </html>
`

const parser = new HtmlParser(html)

// Select a single element using a CSS selector
const heading = parser.selectFirst({ query: css("h1") })
console.log(heading?.text) // "Hello World"

// Select multiple elements and limit results
const items = parser.selectMany({ query: css("li.item"), limit: 2 })
console.log(items.map(el => el.text)) // ["Item 1", "Item 2"]

// Use XPath instead
const firstItem = parser.selectFirst({ query: xpath("//li[@class='item']") })
console.log(firstItem?.text) // "Item 1"
```

> **CommonJS** is also fully supported via `require`:
>
> ```js
> const { parse, css, xpath } = require("@xcrap/html-parser")
> const parser = parse(html)
> ```

---

## 📖 API Reference

### `HtmlParser` / `HTMLParser`

The main entry point for parsing an HTML string. CSS and XPath engines are lazily initialized on first use and reused across subsequent queries.

#### Constructor

```ts
new HtmlParser(content: string): HtmlParser
```

| Parameter | Type     | Description                    |
|-----------|----------|--------------------------------|
| `content` | `string` | The raw HTML string to parse.  |

> **Alias:** You can also use the `parse(content: string)` function as a convenience wrapper:
> ```ts
> import { parse } from "@xcrap/html-parser"
> const parser = parse(html)
> ```

#### `selectFirst(options)`

Selects the **first** element matching the given query.

```ts
parser.selectFirst(options: SelectFirstOptions): HTMLElement | null
```

| Parameter        | Type              | Description                              |
|------------------|-------------------|------------------------------------------|
| `options.query`  | `QueryConfig`     | A query config built with `css()` or `xpath()`. |

Returns `HTMLElement | null` — `null` if no element matches.

#### `selectMany(options)`

Selects **all** elements matching the given query.

```ts
parser.selectMany(options: SelectManyOptions): HTMLElement[]
```

| Parameter        | Type              | Description                              |
|------------------|-------------------|------------------------------------------|
| `options.query`  | `QueryConfig`     | A query config built with `css()` or `xpath()`. |
| `options.limit`  | `number?`         | Optional. Maximum number of elements to return. Values `<= 0` are ignored (returns all). |

Returns `HTMLElement[]` — an empty array if no matches.

---

### `HTMLElement`

Represents a matched DOM element. Provides properties and methods to inspect and traverse its content.

> **Note:** `HTMLElement` instances also support `selectFirst` and `selectMany`, allowing scoped queries within a found element.

#### Properties

| Property     | Type                      | Description                                                        |
|--------------|---------------------------|--------------------------------------------------------------------|
| `outerHTML`  | `string`                  | The full HTML of the element, including its opening and closing tags. |
| `innerHTML`  | `string` *(getter)*       | The inner HTML content (children only, excluding the element's own tags). |
| `text`       | `string` *(getter)*       | The concatenated plain-text content of the element and its descendants. |
| `id`         | `string \| null` *(getter)* | The element's `id` attribute, or `null` if not present.           |
| `tagName`    | `string` *(getter)*       | The element's tag name in **UPPERCASE** (e.g., `"DIV"`, `"H1"`).  |
| `className`  | `string` *(getter)*       | The full `class` attribute string (e.g., `"post featured"`).       |
| `classList`  | `string[]` *(getter)*     | An array of individual class names. Empty array if no class.       |
| `attributes` | `Record<string, string>` *(getter)* | All attributes as a key-value object.                  |
| `firstChild` | `HTMLElement \| null` *(getter)* | The first child element, or `null` if none.              |
| `lastChild`  | `HTMLElement \| null` *(getter)* | The last child element, or `null` if none.               |
| `children`   | `HTMLElement[]` *(getter)* | Direct child elements, in document order.                    |
| `parent`     | `HTMLElement \| null` *(getter)* | The parent element, or `null` at the root.                |
| `ancestors`  | `HTMLElement[]` *(getter)* | All ancestors, from the closest parent up to the root.       |
| `nextSibling` | `HTMLElement \| null` *(getter)* | The next sibling element, or `null` if none.            |
| `previousSibling` | `HTMLElement \| null` *(getter)* | The previous sibling element, or `null` if none.    |
| `siblings`   | `HTMLElement[]` *(getter)* | Sibling elements, excluding the element itself.              |

> **Navigation and XPath:** tree navigation works on elements returned by CSS queries (and anything reached from them). Elements returned by XPath queries carry no document context, so `parent`, `ancestors`, siblings and `closest()` return `null` / `[]` for them.

#### Methods

##### `getAttribute(name)`

```ts
element.getAttribute(name: string): string | null
```

Returns the value of the named attribute, or `null` if the attribute does not exist.

##### `selectFirst(options)`

```ts
element.selectFirst(options: SelectFirstOptions): HTMLElement | null
```

Scoped version of `HtmlParser.selectFirst`. Searches **within** the current element.

##### `selectMany(options)`

```ts
element.selectMany(options: SelectManyOptions): HTMLElement[]
```

Scoped version of `HtmlParser.selectMany`. Searches **within** the current element.

##### `closest(selector)`

```ts
element.closest(selector: string): HTMLElement | null
```

Returns the closest ancestor (or the element itself) matching the CSS `selector`, or `null`.

##### `toString()`

```ts
element.toString(): string
```

Returns the `outerHTML` string of the element.

---

### `css()` and `xpath()`

Helper functions to create typed `QueryConfig` objects.

```ts
css(query: string): QueryConfig
xpath(query: string): QueryConfig
```

These functions are the **recommended way** to build query configurations. They ensure the correct query type is set.

```ts
import { css, xpath } from "@xcrap/html-parser"

css("article.post")           // → { query: "article.post", type: QueryType.CSS }
xpath("//article[@class]")    // → { query: "//article[@class]", type: QueryType.XPath }
```

---

### Types

```ts
// Identifies the query engine to use
export declare const enum QueryType {
  CSS   = 0,
  XPath = 1,
}

// Holds a raw query string and its associated engine type
export interface QueryConfig {
  query: string
  type: QueryType
}

// Options for single-element selection
export interface SelectFirstOptions {
  query: QueryConfig
}

// Options for multi-element selection
export interface SelectManyOptions {
  query: QueryConfig
  limit?: number  // <= 0 or undefined means no limit
}
```

---

## 🔍 Usage Examples

### CSS Selectors

```ts
import { HtmlParser, css } from "@xcrap/html-parser"

const html = `
  <main>
    <article id="post-1" class="post featured" data-author="alice">
      <h2 class="post-title">First Post</h2>
      <p class="excerpt">A short description.</p>
    </article>
    <article id="post-2" class="post" data-author="bob">
      <h2 class="post-title">Second Post</h2>
      <p class="excerpt">Another description.</p>
    </article>
  </main>
`

const parser = new HtmlParser(html)

// Select by tag name
const firstArticle = parser.selectFirst({ query: css("article") })
console.log(firstArticle?.id) // "post-1"

// Select by class
const allPosts = parser.selectMany({ query: css(".post") })
console.log(allPosts.length) // 2

// Select by attribute
const featuredPost = parser.selectFirst({ query: css("[data-author='alice']") })
console.log(featuredPost?.getAttribute("data-author")) // "alice"

// Select with limit
const limited = parser.selectMany({ query: css("article"), limit: 1 })
console.log(limited.length) // 1
```

### XPath Queries

```ts
import { HtmlParser, xpath } from "@xcrap/html-parser"

const html = `
  <ul>
    <li class="tag">rust</li>
    <li class="tag">napi</li>
    <li class="tag">nodejs</li>
  </ul>
`

const parser = new HtmlParser(html)

// Select all <li> with class "tag"
const tags = parser.selectMany({ query: xpath("//li[@class='tag']") })
console.log(tags.map(t => t.text)) // ["rust", "napi", "nodejs"]

// Limit XPath results
const limited = parser.selectMany({ query: xpath("//li"), limit: 2 })
console.log(limited.length) // 2
```

### Navigating Nested Elements

```ts
import { HtmlParser, css } from "@xcrap/html-parser"

const html = `
  <nav id="main-nav">
    <ul>
      <li><a href="/home">Home</a></li>
      <li><a href="/about">About</a></li>
      <li><a href="/contact">Contact</a></li>
    </ul>
  </nav>
`

const parser = new HtmlParser(html)

// Find the nav, then narrow down inside it
const nav = parser.selectFirst({ query: css("#main-nav") })

if (nav) {
  const links = nav.selectMany({ query: css("a") })
  links.forEach(link => {
    console.log(`${link.text} → ${link.getAttribute("href")}`)
    // "Home → /home"
    // "About → /about"
    // "Contact → /contact"
  })

  // First and last child shortcuts
  console.log(nav.firstChild?.tagName)  // "UL"
  console.log(nav.lastChild?.tagName)   // "UL"

  // Walk up and sideways from any element
  const about = links[1]
  console.log(about.parent?.tagName)            // "LI"
  console.log(about.closest("nav")?.id)         // "main-nav"
  console.log(about.parent?.nextSibling?.text)  // "Contact"
  console.log(nav.children.length)              // 1
}
```

### Working with Attributes

```ts
import { HtmlParser, css } from "@xcrap/html-parser"

const html = `
  <a
    id="cta"
    class="btn btn-primary"
    href="https://example.com"
    target="_blank"
    data-track="click"
  >
    Click here
  </a>
`

const parser = new HtmlParser(html)
const link = parser.selectFirst({ query: css("a") })

if (link) {
  console.log(link.id)                        // "cta"
  console.log(link.tagName)                   // "A"
  console.log(link.className)                 // "btn btn-primary"
  console.log(link.classList)                 // ["btn", "btn-primary"]
  console.log(link.getAttribute("href"))      // "https://example.com"
  console.log(link.getAttribute("target"))    // "_blank"
  console.log(link.getAttribute("missing"))   // null
  console.log(link.attributes)
  // {
  //   id: "cta",
  //   class: "btn btn-primary",
  //   href: "https://example.com",
  //   target: "_blank",
  //   "data-track": "click"
  // }
}
```

---

## 🏗️ Architecture

The library is structured as a native Node.js addon written in Rust, bridged via [NAPI-RS](https://napi.rs/).

```
src/
├── lib.rs             # Crate entry point; exposes the `parse()` function via NAPI
├── parser.rs          # HTMLParser struct — lazy-loads CSS (scraper) and XPath (sxd) engines
├── types.rs           # HTMLElement struct — all DOM properties and methods
├── engines.rs         # Internal: select_first/many by CSS and XPath (pure Rust)
└── query_builders.rs  # css() and xpath() helper functions exposed to JS
```

### Key Design Decisions

- **Lazy Initialization**: `HTMLParser` holds `Option<Html>` and `Option<Package>` fields. Each engine is only allocated on first use and reused automatically, so calling `selectFirst` (CSS) and then `selectMany` (XPath) on the same parser creates only two parsing passes total — one per engine.

- **Dual Engine**: CSS queries use the [`scraper`](https://crates.io/crates/scraper) crate; XPath queries use [`sxd-xpath`](https://crates.io/crates/sxd-xpath) with [`sxd_html`](https://crates.io/crates/sxd_html) for HTML→XML normalization.

- **Zero-copy Approach**: Elements are represented by their `outerHTML` string, avoiding complex lifetime management across the FFI boundary.

### Internal Rust Dependencies

| Crate         | Version  | Role                                      |
|---------------|----------|-------------------------------------------|
| `napi`        | `3.0.0`  | NAPI-RS runtime for Node.js integration   |
| `napi-derive` | `3.0.0`  | Procedural macros for NAPI bindings       |
| `scraper`     | `0.25.0` | HTML parsing and CSS selector engine      |
| `sxd-document`| `0.3.2`  | XML document model (used for XPath)       |
| `sxd-xpath`   | `0.4.2`  | XPath expression evaluator                |
| `sxd_html`    | `0.1.2`  | HTML → sxd document converter            |

---

## 🛠️ Development

### Prerequisites

- **Rust** (stable toolchain) — [Install](https://rustup.rs/)
- **Node.js** >= 18 — [Install](https://nodejs.org/)
- **Yarn** >= 4 — `npm install -g yarn`
- **NAPI-RS CLI** — installed automatically via dev dependencies

### Setup

```bash
# Clone the repository
git clone https://github.com/Xcrap-Cloud/html-parser.git
cd html-parser

# Install Node.js dependencies
yarn install
```

### Building

```bash
# Build native addon in release mode
yarn build

# Build in debug mode (faster compilation, slower runtime)
yarn build:debug
```

The output binary (`html-parser.<platform>.node`) will be placed in the project root.

### Running Tests

```bash
yarn test
```

Tests are written with [AVA](https://github.com/avajs/ava) and located in the `__test__/` directory.

### Formatting

```bash
# Format all (TypeScript/JS, Rust, TOML)
yarn format

# Individual formatters
yarn format:prettier   # Prettier for TS/JS/JSON/YAML/Markdown
yarn format:rs         # cargo fmt for Rust
yarn format:toml       # Taplo for TOML files
```

### Linting

```bash
yarn lint   # OXLint for TypeScript/JavaScript files
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. **Fork** the repository.
2. **Create a branch**: `git checkout -b feat/your-feature` or `git checkout -b fix/your-bug`.
3. **Make your changes**, ensuring all tests pass: `yarn test`.
4. **Format your code**: `yarn format`.
5. **Commit** with a descriptive message: `git commit -m "feat: add support for XYZ"`.
6. **Push** your branch: `git push origin feat/your-feature`.
7. **Open a Pull Request** with a clear description of the changes.

Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

---

## 📝 License

Distributed under the [MIT License](./LICENSE).  
© [Marcuth](https://github.com/Marcuth) and contributors.
