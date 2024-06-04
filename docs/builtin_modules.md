# Built-in Modules

The Grape Virtual Machine provides some simple built-in modules:

- [std:out](#stdout)
- [file](#file)
- [tcp](#tcp)

## Stdout

The `std:out` module provides functions for standard output:

| function | descriptor  | description     |
| -------- | ----------- | --------------- |
| println  | (o: Any)    | Print to stdout |
| print    | (0: Any)    | Print to stdout |
| debug    | (o: Any)    | Debug to stdout |
| eprintln | (o: Any)    | Print to stderr |

## File

The `file` module provides functions for reading from files:

| function        | descriptor               | description         |
| --------------- | ------------------------ | ------------------- |
| read_to_string  | (path: String) -> String | Read file to String |
| read_to_bytes   | (path: String) -> Bytes  | Read file to Bytes  |

## TCP

The `file` module provides functions for TCP networking:

| function     | descriptor                           | description                      |
| ------------ | ------------------------------------ | -------------------------------- |
| new_listener | (addr: String)                       | Create new TCP Listener          |
| destroy      | (ref: Listener \| Stream)            | Destroy Listener or Stream       |
| accept       | (ref: Listener) -> Stream            | Block accept incoming connection |
| recv_string  | (ref: Stream) -> String              | Read string from stream          |
| send_string  | (ref: Stream, res: String) -> String | Send string to stream            |
