## A Simple Pokedex like application built to demonstrate Rust + gRPC

### Table of Contents
* Introduction
* Dependencies
* Setup Instructions

### Introduction
This is a sample application which implements a Simple Pokedex like application, used for demonstrating gRPC implementation with Rust. 
This application is for demo purposes only.

### Dependencies

You need the PostgreSQL as a backend to run this project.

The following dependencies are used in this project

| Dependency | Purpose                                           |
|------------|---------------------------------------------------|
| diesel     | For ORM                                           |
| tonic      | gRPC                                              |
| dotenv     | For configuring and reading environment variables |

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

To run all migrations, run
```bash
diesel migration run
```