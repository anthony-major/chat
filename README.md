# Chat
Simple chat server and client written in Rust.

The server uses tokio to handle clients concurrently. The server's port is configurable using the -p or --port command line option.

The client is currently temporary and is only being used to test the server at the moment. The server's address to connect to is configurable using the -a or --address command line option, and the port is configurable with -p or --port. A full client is planned.

---

## Instructions

First, start an instance of the server with ```cargo run -p server```. By default, the server will run on port 9000. Use the -p or --port option to configure the port. The server uses a broadcast channel to send messages across clients. By default, the channel's backlog capacity is 16 messages. This means that the channel can queue 16 messages before things begin to back up and slow down. Use the -c or --capacity option to configure the server's channel capacity. 

Next, start one or more instances of the client with ```cargo run -p client```. By default, the client will connect to localhost (127.0.0.1) port 9000. Use the -a or --address option to configure the address to connect to. Use the -p or --port option to configure the port to connect to.

---

## Notices/Todo
* The messaging protocol as of now is very simple. The server simply reads messages from clients and forwards them to all other clients. There is no check to see if a client is who they say they are in their messages. A client can very well pretend to be another user, even changing their username on every message. However, this does keep things simple and could allow users to change their username very easily whenever they want. 
* Messages are not encrypted in any way going in to or out of the server. They are sent in plaintext as JSON. This means that, currently, messages can be easily read/snooped by a third party, so it is not recommended that any serious information is sent through the chat server. 
