## Overview
1. Client sends a message to the server through a socket/stream. This message is really a request to the server to broadcast a message (the message that was sent) to ALL clients (including the client that sent the message).
2. The server reads the message through the stream in the client's handling task and keeps it on the side.
3. The server sends the message through the client task's tx.
4. The rxs of all the client tasks (including the task for the client that sent the message) receive the message and keep it on the side.
5. For each task, the server sends the message received through the task's rx to the task's client using its socket/stream.
6. The client gets the message through the socket/stream and displays it.

## Protocol
1. Client connects to the server.
2. Client sends their message containing their username. The content field will be ignored.
3. The server sends a connected message to all clients with the newly connected client's username.
4. Client begins sending their chat messages. See overview for the rest.