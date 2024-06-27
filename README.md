# Chat
Simple chat server and client written in Rust.

The server uses tokio to handle clients concurrently. The server's port is configurable using the -p or --port command line option.

The client is currently temporary and is only being used to test the server at the moment. The server's address to connect to is configurable using the -a or --address command line option, and the port is configurable with -p or --port. A full client is planned.
