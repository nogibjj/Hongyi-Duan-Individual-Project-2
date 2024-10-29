# Hongyi-Duan-Individual-Project-2

[![Rust CI/CD Pipeline](https://github.com/nogibjj/Hongyi-Duan-Individual-Project-2/actions/workflows/hello.yml/badge.svg)](https://github.com/nogibjj/Hongyi-Duan-Individual-Project-2/actions/workflows/hello.yml)

## Project Structure

The project follows the typical structure of a Rust project, with source code and configuration files organized as follows:

```
Hongyi-Duan-Individual-Project-2/
│
├── .github/
│   └── workflows/
│       └── rust.yml           # GitHub Actions CI pipeline for build, test, and linting
│
├── src/
│   ├── lib.rs                 # Rust library file containing CRUD functions for SQLite
│   ├── main.rs                # Rust binary entry point that demonstrates the CRUD operations
│
├── tests/
│   └── test_main.py           # Python integration tests for database operations using Databricks SQL connector
│
├── Cargo.toml                 # Cargo file that defines project dependencies and metadata
├── .env                       # Environment variables for database configuration (not included in repository)
├── .gitignore                 # Git ignore file to exclude unnecessary files from version control
├── LICENSE                    # License file
└── README.md                  # This README file with project details
```

---

## Overview

This project is a Rust-based system designed to handle basic SQL operations, including **create**, **read**, **update**, and **delete** (CRUD) for managing hero data. The system interacts with an SQLite database and demonstrates how to work with SQL in a Rust environment, making use of the `rusqlite` crate for SQL connections and operations. The project also includes unit tests to ensure the correct functioning of the database operations.

## Features

- **CRUD Operations**: Create, read, update, and delete heroes from an SQLite database.
- **Efficient Data Handling**: Implements error handling and data manipulation using Rust's robust features.
- **Rust Tooling**: Uses `cargo` for building, testing, and formatting, along with `clippy` for linting.
- **GitHub Actions CI Pipeline**: Automates testing and linting to ensure continuous integration.

## Setup and Installation

To set up and run this project locally, follow the instructions below.

### Prerequisites

- **Rust**: Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
- **SQLite**: Ensure SQLite is installed on your system.
- **Python** (Optional, for Python tests): Install Python from [python.org](https://www.python.org/downloads/) if you wish to run the Python tests.
- **Git**: Ensure Git is installed to clone the repository.

### Cloning the Repository

1. Open a terminal.
2. Clone the repository using Git:

    ```bash
    git clone https://github.com/nogibjj/Hongyi-Duan-Individual-Project-2.git
    cd Hongyi-Duan-Individual-Project-2
    ```

### Installing Dependencies

1. Ensure that you have Rust installed:

    ```bash
    rustup install stable
    ```

2. Check that Rust is properly installed:

    ```bash
    rustc --version
    ```

3. Install the required Rust dependencies listed in `Cargo.toml`:

    ```bash
    cargo build
    ```

4. If you wish to run Python integration tests, install the necessary Python packages:

    ```bash
    pip install -r requirements.txt
    ```

### Environment Setup

This project requires some environment variables to be set for database connectivity. Create a `.env` file in the root directory and configure the following environment variables:

```bash
SERVER_HOSTNAME=your-server-hostname
HTTP_PATH=your-http-path
ACCESS_TOKEN=your-access-token
```

### Running the Application

To run the Rust application:

```bash
cargo run
```

This will execute the main Rust program, which performs the CRUD operations for managing hero data.

## Usage

The program demonstrates the following database operations:

1. **Creating a Table**: Initializes the `hero` table in an SQLite database.
2. **Inserting Data**: Inserts hero records with an `id`, `name`, and `power`.
3. **Querying Data**: Retrieves a hero's information based on their `id`.
4. **Updating Data**: Updates the `power` of an existing hero.
5. **Deleting Data**: Deletes a hero record by their `id`.

The functions in `lib.rs` can be reused or expanded upon for additional SQL operations or different database schemas.

### Example Usage

In the main program (`main.rs`), you can modify the heroes and the operations to match your use case.

```rust
// Insert a new hero
let hero = Hero {
    id: 1,
    name: String::from("Superman"),
    power: 100,
};
insert_hero(&conn, hero)?;

// Read and display hero details
let hero = get_hero(&conn, 1)?;
println!("Hero ID: {}, Name: {}, Power: {}", hero.id, hero.name, hero.power);
```

## Running Tests

### Rust Unit Tests

To run the Rust unit tests for the CRUD operations:

```bash
cargo test
```

This will run the unit tests defined in the project to verify the SQL functionality.

### Python Integration Tests

If you've set up the environment and installed Python dependencies, you can run the Python tests with:

```bash
python -m pytest -cov=main test_main.py
```

This will run integration tests for the database connectivity.

## Continuous Integration (CI)

The project includes a GitHub Actions workflow (`rust.yml`) that automates the following tasks on each push to the `main` branch:

1. **Build the project** using `cargo build`.
2. **Run the tests** using `cargo test`.
3. **Lint the code** using `cargo clippy`.
4. **Check code formatting** using `cargo fmt`.

The workflow ensures the codebase is consistently built, tested, and formatted properly.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes and push the branch to your fork.
4. Open a pull request with a detailed description of the changes.
