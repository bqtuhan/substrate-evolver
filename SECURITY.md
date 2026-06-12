# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability within **substrate-evolver**, please do **not** open a public issue. Instead, send a detailed report to **bqtuhan** via the GitHub private reporting feature or through encrypted communication channels (available upon request).

We strive to acknowledge reports within **48 hours** and provide a timeline for resolution within **one week**.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |
| < latest| :x:                |

## Security Architecture Considerations

- The WebAssembly module (`pkg/`) runs entirely inside the browser's sandbox. It does **not** access files, network, or system resources.
- All memory is managed by Rust’s ownership model, preventing buffer overflow issues.
- JavaScript only reads from WASM linear memory via raw pointers; no complex objects cross the boundary.
- Updates to dependencies are monitored via `cargo audit` and GitHub Dependabot.

If you find a way to exploit the WASM memory interface or crash the simulation through unexpected inputs, please report it immediately.