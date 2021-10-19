# tonic-listenfd-graceful-shutdown

## Requirements

- `curl`
- `systemfd` => `cargo install systemfd`

## How to test

### Don't set the TCP listener to nonblocking mode

1. Comment out this line in `main.rs`:

```rust
std_listener.set_nonblocking(true).expect("Failed to set non-blocking mode");
```

to

```rust
// std_listener.set_nonblocking(true).expect("Failed to set non-blocking mode");
```

2. Build

`cargo build`

3. Run with systemfd

`systemfd -s http::5000 -- ./target/debug/tonic-graceful-shutdown`

(Replace http port with another value if that port is already used on your
system)

3. Make a request with curl

`curl localhost:5000`

4. Attempt to shut down the server with pkill

`pkill -SIGINT tonic`

Notice that the server does not shut down.

5. Kill the process

`pkill -SIGKILL tonic`

### Set the TCP listener to nonblocking mode

1. Uncomment this line in `main.rs`:

```rust
// std_listener.set_nonblocking(true).expect("Failed to set non-blocking mode");
```

to

```rust
std_listener.set_nonblocking(true).expect("Failed to set non-blocking mode");
```

2. Build

`cargo build`

3. Run with systemfd

`systemfd -s http::5000 -- ./target/debug/tonic-graceful-shutdown`

(Replace http port with another value if that port is already used on your
system)

3. Make a request with curl

`curl localhost:5000`

4. Attempt to shut down the server with pkill

`pkill -SIGINT tonic`

Notice that the server shuts down properly.
