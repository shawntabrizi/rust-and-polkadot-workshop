
## Build the runtime with your new pallet

After you've copied all of the parts of the proof-of-existence pallet into the `pallets/template/lib.rs`file, you are ready to compile and start the node.

To compile and start the updated Substrate node:

1. Open a terminal shell.

1. Change to the root directory for the node template.

1. Compile the node template by running the following command:

    ```bash
    cargo build --release
    ```

1. Start the node in development mode by running the following command:

    ```bash
    ./target/release/node-template --dev
    ```

    The `--dev` option starts the node using the predefined `development` chain specification.
    Using the `--dev` option ensures that you have a clean working state any time you stop and restart the node.

1. Verify the node produces blocks.
