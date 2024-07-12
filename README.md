# InsuLink

InsulinSync is a microservice developed in Rust that acts as a bridge between the diabetes service and the external professional platforms that manage patients' insulin pumps. This service enables insulin pump data to be retrieved, analysed and used for diabetes monitoring.

## Features
- Recovers insulin pump data from various external platforms.
- Standardisation and aggregation of data for uniform use in the diabetes service.
- RESTful interface for communication with the diabetes service.

## Prerequisites

- Rust (stable version recommended)
- Cargo (package and build manager for Rust)

## Environment configuration

| Variable                 | Description                                       | Exemple                          |
|--------------------------|---------------------------------------------------|----------------------------------|
| `HOST`                   | Host address for the application                  | `0.0.0.0`                        |
| `PORT`                   | Port on which the application is listening        | `3333`                           |


## Installation

1. Clone the repository:
```bash
$ git clone https://github.com/libreconnect/insulink.git
$ cd insulink
```
2. Compiling the project:
```bash
$ cargo build --release
$ cargo run
```
