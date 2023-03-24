# Simple partial IRC implementation
Please see specification and implementation PDF for details on implementation, and IRC complience.

## Build instructions
```
cargo build
```

## or for release builds
```
cargo build --release
```

## How to run this code

### Server startup
```
./target/<debug|release>/server --address 127.0.0.1:2323
```

### Client startup
```
./target/<debug|release>/client --address 127.0.0.1:2323 --hostname hephaestus --realname "Julian Lazaras" --username cardinal
```