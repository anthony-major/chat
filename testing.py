import socket

message1 = '{"username": "User", "content": ""}\0'.encode()
message2 = '{"username": "OtherUser", "content": ""}\0'.encode()

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

s.connect(("127.0.0.1", 9000))

print(message1)
s.send(message1)

print(message2)
s.send(message2)

input("Press enter to exit...")
s.close()