# bolt-rs

## Overview

This project provides a comprehensive set of libraries that allow for interaction with graph database servers that
support the [Bolt v1](https://boltprotocol.org/v1/) protocol, namely, [Neo4j](https://neo4j.com).

### bolt-client
([API docs](https://docs.rs/bolt-client))

Contains an asynchronous client for Bolt-compatible servers, using a [tokio](https://crates.io/crates/tokio) 
[`BufStream`](https://docs.rs/tokio/0.2.10/tokio/io/struct.BufStream.html) wrapping a 
[`TcpStream`](https://docs.rs/tokio/0.2.10/tokio/net/struct.TcpStream.html).

### bolt-proto
([API docs](https://docs.rs/bolt-proto))

Contains the traits and primitives used in the protocol. The `Message` and `Value` enums are of particular importance,
and are the primary units of information sent and consumed by Bolt clients/servers.

### bolt-proto-derive
([API docs](https://docs.rs/bolt-proto-derive))

Ugly procedural macros used in bolt-proto to derive serialization-related traits.

### r2d2-bolt

A bolt-client adapter crate for the [r2d2](https://crates.io/crates/r2d2) connection pool.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/lucis-fluxum/bolt-rs.

## License

This crate is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT), with
portions of the documentation licensed under the 
[Creative Commons Attribution-ShareAlike 3.0 License](https://creativecommons.org/licenses/by-sa/3.0/).
