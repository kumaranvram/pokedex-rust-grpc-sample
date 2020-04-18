## A Simple Pokedex like application built to demonstrate Rust + gRPC

### Table of Contents
* Introduction
* Prerequisites and dependencies
* Setup Instructions
* Build
* Running the server

### Introduction
This is a sample application which implements a Simple Pokedex like application, used for demonstrating gRPC implementation with Rust. 
This application is for demo purposes only.

### Prerequisites and dependencies

Apart from the [rust lang dependencies](https://www.rust-lang.org/learn/get-started), you need the PostgreSQL as a backend to run this project.

The following dependencies are used in this project

| *Dependency* | *Purpose*                                               |
|--------------|-------------------------------------------------------|
| diesel       | For ORM                                               |
| tonic        | gRPC                                                  |
| dotenv       | For configuring and reading environment variables     |

### Setup Instructions

Install `diesel-cli` by running
```bash
cargo install diesel_cli
```

Create a copy of the `.env.example` as `.env`
```bash
cp .env.example .env
```

Alter the values in the .env file to the required values. Then run 
```bash
diesel setup
```

### Build

First, to run all migrations, run
```bash
diesel migration run
```

To build the application, run
```bash
cargo build
```

### Running the server

To start the server, run

```
cargo run --bin server
```

The server will start running on the `APP_PORT` specified in `.env` file.

The application can be tested using a app like [BloomRPC](https://github.com/uw-labs/bloomrpc) 
or a CLI application like [grpcc](https://github.com/njpatel/grpcc)