### How to Build and Use

1. Update the code.
2. Add, update, or fix tests.
3. Run `cargo build && cargo test`.
4. Build the Docker image:
  ```sh
  docker build -t minswap-csl .
  ```
5. Run Docker Compose:
  ```sh
  docker compose up -d --build
  ```
6. Execute into the Docker container to build the Node.js SDK:
  ```bash
  # Execute into the Docker container
  docker exec -it ${CONTAINER_ID} /bin/bash
  # Build Rust
  cargo build
  # Change directory back to root
  cd ..
  # Install npm dependencies
  npm install
  # Build Node.js SDK
  npm run rust:build-nodejs
  # Exit Docker
  ```
7. The final build is stored on your local machine at `./rust/pkg` due to mounting `.:/usr/src/app`.
