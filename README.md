# React-Wasm-HelloWorld

This is a simple React application that demonstrates how to integrate WebAssembly (Wasm) using `wasm-pack` and `React`.

## Tech Stack

- **Frontend:** React + TypeScript
- **Wasm:** Rust + wasm-pack

## Project Structure

```tree
├── src/              # React source code
│   ├── App.tsx       # main React component
│   ├── main.tsx      # entry point for React
├── wasm/             # WebAssembly source code
│   └── src/  
│       ├── lib.rs    # main Rust file
│       └── types.rs  # type definition
```

## Install Dependencies

```bash
pnpm install
```

## Wasm Build

```bash
cd wasm
wasm-pack build --target web
```

## Run Development Server

```bash
pnpm dev
```
