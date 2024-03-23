# Commit 1 Reflection Notes

Based on Rust Documentation:
- The `handle_connection` function takes a mutable reference to a TcpStream as its parameter, allowing it to read from and write to the stream.
- It creates a `BufReader` from the TcpStream, which helps with efficient reading by buffering input.
- It uses the lines method of `BufReader` to iterate over lines in the buffered reader.
- The `map` method is used to unwrap each `Result` from `lines`, converting it into a `String`.
- The `take_while` method is used to take lines until an empty line is encountered.
- Finally, the collected lines are stored in a vector named `http_request`, which is printed out using println.

In summary, the function handles the incoming TCP connection, reads the HTTP request from the client, and prints it out.