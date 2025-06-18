# echo

A simple HTTP server that will echo the request details back in the response.

## Overview

This project provides an HTTP server that echoes back details of any request it
receives. It is useful for debugging, testing HTTP clients, or inspecting
requests. The server responds to any path and method, returning a JSON object
containing:

- Host
- HTTP method
- Path
- Headers
- Query parameters (if any)
- Request body (as JSON, a UTF-8 string, or base64-encoded binary)

## Usage

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)

### Running the Server

1. Clone the repository and navigate to the project directory.
2. Build and run the server:

```sh
cargo run
```

By default, the server listens on port `8000`. You can change the port by
setting the `PORT` environment variable:

```sh
PORT=3000 cargo run
```

### Example Request

```sh
curl -X POST 'http://localhost:8081/hello?foo=bar' \
  -H 'Content-Type: application/json' \
  -d '{"message": "hi"}'
```

The response will be a JSON object echoing the request details.

## License

See [LICENSE](LICENSE).
