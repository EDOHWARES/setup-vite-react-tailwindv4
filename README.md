# Setup Vite + React + Tailwind (Rust Script)

This Rust program automates the setup of a **Vite + React + TailwindCSS version 4** project, making it quick and easy to get started with modern frontend development.

## Features

- Creates a **Vite + React** project
- Works on **Windows, macOS, and Linux**

## Prerequisites

Make sure you have the following installed on your system:

- [Node.js](https://nodejs.org/) (Includes npm)
- [Rust](https://www.rust-lang.org/) (For building the executable)

## Installation

### 1️⃣ Build the Rust executable

```bash
cargo build --release
```

### 2️⃣ Move the binary to a global location

#### Linux/macOS

```bash
sudo mv target/release/setup-vite /usr/local/bin/setup-vite-react-tailwindcssv4
```

#### Windows

```bash
move target\release\setup-vite-react-tailwindcssv4.exe C:\Users\YourUsername\AppData\Local\
```

## Usage
```bash
setup-vite-react-tailwindcssv4 my-project
```