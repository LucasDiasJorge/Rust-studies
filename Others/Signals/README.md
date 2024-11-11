# Ctrl-C Ticker

A simple Rust program that uses channels to handle tick events and gracefully exits on `Ctrl-C`.

## Description

This project demonstrates a Rust application that performs work on a timed interval using `crossbeam_channel::tick`. It listens for `Ctrl-C` (SIGINT) events to allow for a clean exit.

## Features

- Emits a "working!" message every second.
- Listens for a `Ctrl-C` input, prints a "Goodbye!" message, and then exits gracefully.

## Dependencies

- [Crossbeam](https://crates.io/crates/crossbeam): A library providing concurrency tools like channels.
- [Anyhow](https://crates.io/crates/anyhow): Used for error handling.
- [Ctrlc](https://crates.io/crates/ctrlc): Used to handle `Ctrl-C` interrupts in a portable way.

## Code Explanation

- **`ctrl_channel` function**: Creates a channel for `Ctrl-C` events using the `ctrlc` library. When `Ctrl-C` is detected, the event is sent through this channel.
- **`main` function**:
    - Creates a tick channel that sends a signal every second.
    - Enters a loop that waits for two possible events:
        1. A tick from the `ticks` channel, triggering a print statement "working!"
        2. A `Ctrl-C` event from `ctrl_c_events`, which triggers a print statement "Goodbye!" and then breaks the loop to exit.

## How to Run

1. Clone this repository.
2. Navigate to the project folder.
3. Build and run the project with:

   ```bash
   cargo run
   ```

4. The program will start, and you’ll see "working!" printed every second. Press `Ctrl-C` to terminate the program gracefully.

## Requirements

- **Rust**: Ensure that Rust is installed on your system. You can install it [here](https://www.rust-lang.org/tools/install).

## Example Output

```plaintext
working!
working!
working!
...
```

When you press `Ctrl-C`, you’ll see:

```plaintext
Goodbye!
```

