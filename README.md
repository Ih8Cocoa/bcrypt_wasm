# In-browser BCrypt (using Rust & WASM)

## Getting started
- First, install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Then, `cd` into this project's folder
- Execute `wasm-pack build --target web` (for ES Module support) or `wasm-pack build --target no-modules` (without ES Modules)
- The rest of the instructions can be found [here](https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html)

## Available functions
`hash_pw(password: string, rounds: number): string`:
Hashes the password, and return the hash
- `password`: Your password
- `rounds`: Number of rounds. 13 is recommended
- Return value: The hashed password

`check_pw(password: string, hash: string): boolean`: Check if the password matches the hash
- `password`: Your password
- `hash`: The BCrypt hash
- Return value: Whether the password matches the hash

All of these functions are blocking, which means that your browser may freeze or something if used as-is.

Use [Web Workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers) if you want asynchronous execution