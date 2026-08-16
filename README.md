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

For information regarding compatibility, see [COMPATIBILITY.md](COMPATIBILITY.md). For usage examples, see [jeremy-code/comrak-wasm-examples](https://github.com/jeremy-code/comrak-wasm-examples).

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
import { createReadStream } from "node:fs";
import { fileURLToPath } from "node:url";
import init, { markdownToHtml } from "comrak-wasm";

const wasmModuleResponse = new Response(
    createReadStream("comrak-wasm/comrak_wasm_bg.wasm"),
);
wasmModuleResponse.headers.set("Content-Type", "application/wasm");

await init({ module_or_path: wasmModuleResponse });
const html = markdownToHtml(/* ... */);
```

### Deno

You can use the `init` function with no configuration if you have `--allow-read` option enabled.

```js
import init, { markdownToHtml } from "comrak-wasm";

await init();
const html = markdownToHtml(/* ... */);
```

### Bun

```js
import init, { markdownToHtml } from "comrak-wasm";

await init();
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

#### Cloudflare Workers

Cloudflare Workers ban the usage of `WebAssembly.compile`, `WebAssembly.compileStreaming`, `WebAssembly.instantiate` with arbitrary data and `WebAssembly.instantiateStreaming`.[^1] In workerd, a Wasm file can be directly imported as a `WebAssembly.Module` object.[^2]

```js
import wasmModule from "comrak-wasm/comrak_wasm_bg.wasm";
import init, { markdownToHtml } from "comrak-wasm";

await init({
    module_or_path: wasmModule /* WebAssembly.Module */,
});

const html = markdownToHtml(/* ... */);
```

[^1]: https://developers.cloudflare.com/workers/runtime-apis/web-standards/#javascript-standards

[^2]: See https://github.com/cloudflare/workers-sdk/blob/b8fd112136abf4ff17c3d456eaa7b22880bcaf6a/packages/miniflare/src/runtime/config/generated/workerd.ts#L933-L941 and https://github.com/cloudflare/workers-sdk/blob/b8fd112136abf4ff17c3d456eaa7b22880bcaf6a/fixtures/wasm-app/worker/module/export_wasm.js#L4

## License

This project is licensed under the BSD 2-Clause. See the [LICENSE](LICENSE) file for details.
