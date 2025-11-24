# Contributing to dispman

Thank you for your interest in contributing to `dispman`!

## Development Setup

1.  **Prerequisites**:
    *   Rust (latest stable)
    *   Windows (for testing DDC/CI)

2.  **Build**:
    ```bash
    cargo build
    ```

3.  **Run**:
    ```bash
    cargo run -- detect
    ```

## Code Style

*   Follow standard Rust formatting (`cargo fmt`).
*   Ensure `cargo clippy` passes without warnings.
*   Keep `unsafe` blocks as small as possible and document why they are needed.

## Pull Requests

1.  Fork the repository.
2.  Create a feature branch.
3.  Submit a Pull Request with a clear description of changes.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.
