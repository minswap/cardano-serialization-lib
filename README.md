### How to build and use?

1. **Build the Docker image:**

  Run the following command to build the Docker image with the tag `minswap-csl`:

  ```sh
  docker build -t minswap-csl .
  ```

2. **Create a temporary container:**

  Use the following command to create a temporary container from the `minswap-csl` image:

  ```sh
  docker create --name temp_container minswap-csl:latest
  ```

3. **Copy the Rust package:**

  Copy the Rust package from the temporary container to your local machine using the command below:

  ```sh
  docker cp temp_container:/usr/src/app/rust/pkg rust/pkg
  ```

  This command copies the `pkg` directory from the container's `/usr/src/app/rust` path to your local `rust/pkg` directory.

4. **Clean up the temporary container:**

  After copying the necessary files, you can remove the temporary container:

  ```sh
  docker rm temp_container
  ```

  This helps to keep your Docker environment clean.
