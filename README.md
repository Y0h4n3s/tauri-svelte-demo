# Tauri + Svelte App

This repository contains a Tauri desktop application built using the Svelte framework for the frontend. The app demonstrates secure data handling and local storage encryption with a polished UI powered by Tailwind CSS.

## Features

- **Frontend**: Built with Svelte and styled using Tailwind CSS.
- **Backend**: Powered by Tauri, with commands to handle data securely.
- **Secure Storage**: Encrypted local storage using `tauri-plugin-stronghold`.
- **Cross-Platform**: Compatible with Windows, macOS, and Linux.

## Prerequisites

Ensure you have the following installed on your system:

1. **Rust** (latest stable version)  
   Install Rust using [Rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. **Node.js** (latest LTS version)
Install Node.js from https://nodejs.org/.

3. **Tauri CLI**
Install Tauri CLI globally:

```bash
cargo install tauri-cli
```
4. **Package Manager**:
Install pnpm or use npm (comes with Node.js).


---

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/Y0h4n3s/tauri-svelte-demo
cd tauri-svelte-demo

```


---

## Development Workflow
## Development Workflow

### 1. Run the Frontend in Development Mode

Start the development server for the app:
```bash
npm run tauri dev
```


---

## Building the App


Generate a production build for the app:
```bash
cd frontend
npm run tauri build

