#Documentation

The PROJECT API is the API of [PROJECT API](https://example.com)

## Author 🚀
> [@SofwanCoder](https://github.com/sofwancoder)

---

## Technologies

- Rust
- Actix-Web
---

## Database

- [MongoDB](https://www.mongodb.com/)
- [PostgreSQL](https://www.postgresql.org/)
- [Redis](https://redis.io/)

---

## Install Rust

To Install Rust, go to [RustLang](https://www.rust-lang.org/) and follow the necessary instructions required depending on
your PC Operating System

---

## Install Cargo

To Install Cargo, go to [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and follow the necessary instructions required depending on
your PC Operating System

---
## The Env File

Copy the `.env.sample` file and rename it to `.env` and update the values accordingly

---
## Setup Database
To set up your database for the project, after creation kindly open the .env file and update as necessary

```markdown
POSTGRES_URI=
REDIS_URI=
MONGO_URI=
```
---
## Development

Use this template `https://github.com/SofwanCoder/rust_project_api`

### Dev Dependency

To install the dev dependencies needed to monitor and continuously watch for changes, kindly run

```markdown
cargo install cargo-watch
```

### Installation

To install the necessary packages, in your folder directory kindly run

```markdown
cargo build
```

- To continuously watch for changes

  - ```markdown
    cargo watch -c -w src -x run
    ```
- To run test cases
  - ```markdown
    cargo test
    ```

- To build your app (for production append `--release` to the command)

  - ```markdown
    cargo build
    ```

- To run your app server (for production append `--release` to the command)

  - ```markdown
    cargo run
    ```

- To validate without building
  - ```markdown
    cargo check
    ```


