# comrak-wasm

<!-- Link references -->

[github-actions]: https://www.github.com/jeremy-code/comrak-wasm/actions/workflows/ci.yml
[github-actions-badge]: https://www.github.com/jeremy-code/comrak-wasm/actions/workflows/ci.yml/badge.svg
[license-badge]: https://img.shields.io/github/license/jeremy-code/comrak-wasm
[npm-version-badge]: https://img.shields.io/npm/v/comrak-wasm
[npm-package]: https://www.npmjs.com/package/comrak-wasm

[![GitHub Actions][github-actions-badge]][github-actions] [![License][license-badge]](LICENSE) [![NPM version][npm-version-badge]][npm-package]

Comrak markdown parser compiled to WebAssembly with JavaScript bindings. Documentation is avaliable at https://npmx.dev/package-docs/comrak-wasm/.

## Usage

### Browser

```js
import init, { markdownToHtml } from "comrak-wasm";

await init();
const html = markdownToHtml(
    `# Hello world!
1. Do ~~you~~ like [pretty](#) paintings?
2. Or *pretty* music?
`,
    {
        extension: {
            strikethrough: true,
        },
    },
);

// <h1>Hello world!</h1>
// <ol>
// <li>Do <del>you</del> like <a href="#">pretty</a> paintings?</li>
// <li>Or <em>pretty</em> music?</li>
// </ol>
//
console.log(html);
```

### Node.js

```js
import init, { markdownToHtml } from "comrak-wasm";
import { createReadStream } from "node:fs";

const wasmModuleResponse = new Response(
    createReadStream("comrak-wasm/comrak_wasm_bg.wasm"),
);
wasmModuleResponse.headers.set("Content-Type", "application/wasm");

await init({ module_or_path: wasmModuleResponse });
const html = markdownToHtml(/* ... */);
```

### Bundler

#### Vite

```js
import init, { markdownToHtml } from "comrak-wasm";
import wasmModuleUrl from "comrak-wasm/comrak_wasm_bg.wasm?url";

await init({
    // Alternatively, `import.meta.resolve("comrak-wasm/comrak_wasm_bg.wasm")`
    module_or_path: new URL(wasmModuleUrl),
});
const html = markdownToHtml(/* ... */);
```

You might also consider importing the Wasm module from a CDN like so:

```js
import init, { markdownToHtml } from "comrak-wasm";

await init({
    module_or_path: fetch(
        // https://unpkg.com/comrak-wasm@^0.0.1/comrak_wasm_bg.wasm
        "https://esm.sh/comrak-wasm@^0.0.1/comrak_wasm_bg.wasm",
        {
            headers: {
                Accept: "application/wasm",
            },
        },
    ),
});
const html = markdownToHtml(/* ... */);
```

#### Webpack

If [`experiments.futureDefault`](https://webpack.js.org/configuration/experiments/#experimentsfuturedefaults) is enabled, then use `?url` like Vite. Otherwise, update your webpack config like this:

```js
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export default {
    entry: "./src/index.js",
    output: {
        filename: "main.js",
        path: resolve(__dirname, "dist"),
    },
    module: {
        rules: [
            {
                // Or if you don't want to use a resourceQuery, you can enable it for .wasm files
                // test: /\.wasm/,
                resourceQuery: /url/,
                type: "asset/resource",
            },
        ],
    },
};
```

## License

This project is licensed under the BSD 2-Clause. See the [LICENSE](LICENSE) file for details.
