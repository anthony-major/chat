# Chat
Simple chat server and client written in Rust.

![screenshot](https://github.com/anthony-major/chat/blob/main/assets/screenshot.png)

The server hosts a single chat room where multiple clients can connect and exchange messages. The protocol used is very simple. The server simply passes around messages to all clients. Messages consist of a null-terminated JSON string containing a username and a content field. In theory, it should be fairly straightforward to add additional fields (i.e. timestamps). When the server receives a valid message from a client, it forwards the message to all clients, including the client that sent the message.

The client is a GUI program written using egui. At the moment, it is quite simple, but it should be fairly easy to expand. Currently, the UI features a scrolling area that displays messages and a single-line text input. The UI uses a client backend that can be used for almost any UI/CLI. The backend is responsible for connecting to the server, receiving messages from the server, and forwarding them to a channel. It also receives messages from the UI through a channel and sends them to the server. The UI reads messages from the exposed channel receiver and displays them.

### Technology Used
* [Rust](https://www.rust-lang.org/)
* [tokio](https://tokio.rs/)
* [serde](https://serde.rs/)
* [clap](https://docs.rs/clap/)
* [egui](https://www.egui.rs/)

---

## Instructions/Getting Started

First, start an instance of the server with ```cargo run -p server```. By default, the server will run on port 9000. Use the -p or --port option to configure the port. The server uses a broadcast channel to send messages across clients. By default, the channel's backlog capacity is 16 messages. This means that the channel can queue 16 messages before things begin to back up and slow down. Use the -c or --capacity option to configure the server's channel capacity. 

Next, start one or more instances of the client with ```cargo run -p client```. By default, the client will connect to localhost (127.0.0.1) port 9000. Use the -a or --address option to configure the address to connect to. Use the -p or --port option to configure the port to connect to. The default username is 'User' and can be configured with the -u or --username option.

The -h or --help option may be used with the server and client to display program usage.

---

## Notices/Todo
* Messages are not encrypted in any way going in to or out of the server. They are sent in plaintext as JSON. This means that, currently, messages can be easily read/snooped by a third party, so it is not recommended that any serious information is sent through the chat server.
* Message history is not persisted. A database integration could be added to save message history between server launches.
* The client GUI is pretty simple right now, it could probably be expanded, for example, a send button.
* The client is currently configured through command line arguments. It would be nice to be able to specify the connection address, port, and username from inside the GUI.

## Update Ideas
### Server
* Multiple chat rooms. This could either be implemented server-side where the server manages a list of rooms and the users in them and forwards messages only to users in a room, or client-side where the server does what it currently does (forwards all messages as is) and clients are responsible for how they handle room information (either discarding messages that are not for the user's current room or keeping them somewhere). Either way, the message protocol will have to be updated to contain room information; a simple room or room_id field should suffice. 
* Message timestamps.
### Client
* Start page for entering server connection information (address and port) and client information (username). (Room selection could be here as well if rooms are added.)
* Additional UI.
    * Send button.
    * Back button to go back to start page.
    * Server information display (address, name, etc.).
    * Client information display (address, username, etc.).
    * Etc.
