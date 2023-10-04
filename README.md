# little-rusty

HTTP server built using Rust.

Limited implementation of [RFC2616 14.35.1](https://www.rfc-editor.org/rfc/rfc2616.html#section-14.35.1), just enough so that "wget -c" works.
You can resume paused downloads.

## How to build

* Clone the repo:

  ```sh
  git clone git@github.com:dafaqdhruv/little-rusty.git && cd little-rusty
  ```

* Build binary:

  ```sh
  cargo build
  ```

  or, to build in release mode:

  ```sh
  cargo build --release
  ```

## Usage

* In target directory:

  ```sh
  ./path/to/binary [port]
  ```

  `[port]` The TCP port number to listen on. Default is `8990`.
  Incoming requests are manually logged to `stdout`. Verbose logging will be added in future.

* Send `ctrl+c` (SIGINT) to kill server
