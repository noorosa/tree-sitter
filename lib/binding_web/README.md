
[npmjs.com]: https://www.npmjs.org/package/web-tree-sitter
[npmjs.com badge]: https://img.shields.io/npm/v/web-tree-sitter.svg?color=%23BF4A4A

WebAssembly 

You can download the `web-tree-sitter.js` and `web-tree-sitter.wasm` files from [the latest GitHub release][gh release] and load
them using a standalone script:

```html
<script src="/the/path/to/web-tree-sitter.js"></script>

<script>
  const { Parser } = window.TreeSitter;
  Parser.init().then(() => { /* the library is ready */ });
</script>
```

You can also install [the `web-tree-sitter` module][npm module] from NPM and load it using a system like Webpack:

```js
const { Parser } = require('web-tree-sitter');
Parser.init().then(() => { /* the library is ready */ });
```

or Vite:

```js
import { Parser }  from 'web-tree-sitter';
Parser.init().then(() => { /* the library is ready */ });
```

With Vite, you also need to make sure your server provides the `tree-sitter.wasm`
file to your `public` directory. You can do this automatically with a `postinstall`
[script](https://docs.npmjs.com/cli/v10/using-npm/scripts) in your `package.json`:

```js
"postinstall": "cp node_modules/web-tree-sitter/tree-sitter.wasm public"
```

You can also use this module with [deno](https://deno.land/):

```js
import Parser from "npm:web-tree-sitter";
await Parser.init();
// the library is ready
```

To use the debug version of the library, replace your import of `web-tree-sitter` with `web-tree-sitter/debug`:

```js
import { Parser } from 'web-tree-sitter/debug'; // or require('web-tree-sitter/debug')

Parser.init().then(() => { /* the library is ready */ });
```

This will load the debug version of the `.js` and `.wasm` file, which includes debug symbols and assertions.

> [!NOTE]
> The `web-tree-sitter.js` file on GH releases is an ES6 module. If you are interested in using a pure CommonJS library, such
> as for Electron, you should use the `web-tree-sitter.cjs` file instead.

### Basic Usage

First, create a parser:

```js
const parser = new Parser();
```

Then assign a language to the parser. Tree-sitter languages are packaged as individual `.wasm` files (more on this below):

```js
const { Language } = require('web-tree-sitter');
const JavaScript = await Language.load('/path/to/tree-sitter-javascript.wasm');
parser.setLanguage(JavaScript);
```

Now you #
console.log(callExpression);

// { type: 'call_expression',
//   startPosition: {row: 0, column: 16},
//   endPosition: {row: 0, column: 30},
//   startIndex: 0,
//   
  }
}
```

[docker]: https://www.docker.com
[emscripten]: https://emscripten.org
[emscripten-module-options]: https://emscripten.org/docs/api_reference/module.html#affecting-execution
[gh release]: https://github.com/tree-sitter/tree-sitter/releases/latest
[gh release js]: https://github.com/tree-sitter/tree-sitter-javascript/releases/latest
[node bindings]: https://github.com/tree-sitter/node-tree-sitter
[npm module]: https://www.npmjs.com/package/web-tree-sitter
[podman]: https://podman.io
