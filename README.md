# Simple Asynchronous Web Servers in Rust

This project is a simple HTTP server built using the hyper and tokio crates in Rust. It demonstrates how to set up a basic web server that responds with a "Hello, World!" message.

## Table of Contents

Introduction
Features
Prerequisites
Installation
Usage
Project Structure
Contributing

### Introduction

This project serves as a basic introduction to building asynchronous web servers in Rust using hyper and tokio. It is designed for educational purposes and provides a foundation upon which more complex web servers can be built.

### Features

Asynchronous HTTP server using hyper
Simple request handler that responds with "Hello, World!"
Structured project setup for easy extension and modification

### Prerequisites

To build and run this project, you need to have the following software installed:

* Rust (latest stable version)
* Cargo (comes with Rust)
* Internet connection to download dependencies

### Installation

Clone the repository:

``` sh
git clone https://github.com/golnarmordadi/Simple-HTTP-Server.git
cd simple-http-server
```

Build the project:

```sh
cargo build
```

### Usage

Run the server:

```sh
cargo run
```

Access the server:

```sh
Open your web browser and go to http://127.0.0.1:3000. You should see a "Hello, World!" message.
```

### Project Structure

simple-http-server-rust/
├── Cargo.toml       # Project dependencies and metadata
└── src/
    └── main.rs      # Main application code

### Contributing

Contributions are welcome! If you have suggestions for improvements or new features, please open an issue or create a pull request. Make sure to follow the code of conduct.
