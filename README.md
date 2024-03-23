# Commit 1 Reflection Notes

Based on Rust Documentation:
- The `handle_connection` function takes a mutable reference to a TcpStream as its parameter, allowing it to read from and write to the stream.
- It creates a `BufReader` from the TcpStream, which helps with efficient reading by buffering input.
- It uses the lines method of `BufReader` to iterate over lines in the buffered reader.
- The `map` method is used to unwrap each `Result` from `lines`, converting it into a `String`.
- The `take_while` method is used to take lines until an empty line is encountered.
- Finally, the collected lines are stored in a vector named `http_request`, which is printed out using println.

In summary, the function handles the incoming TCP connection, reads the HTTP request from the client, and prints it out.

# Commit 2 Reflection Notes

![Commit 2 screen capture](/assets/images/commit2.png)

In the new `handle_connection` function, here is what it does based on Rust Documentation:

- It creates a `BufReader` from the TcpStream to efficiently read data from the stream.
- It reads lines from the buffered reader until an empty line is encountered, representing the end of HTTP headers.
- It sets the HTTP response status line to "HTTP/1.1 200 OK", indicating a successful response.
- It reads the contents of the file "hello.html" into a string.
- It calculates the length of the contents to include in the response body.
- It formats the HTTP response including the status line, content length, and contents of "hello.html".
- It writes the response to the TCP stream using `write_all`, ensuring that the entire response is sent.

Overall, the function handles an incoming TCP connection, reads the HTTP request, prepares an HTTP response (with a status line, content length, and html content) and writes the response back to the client over the TCP stream.

# Commit 3 Reflection Notes

![Commit 3 screen capture](/assets/images/commit3.png)

### Changes Made:
- **Modularized Request Handling:**
Moved request handling logic to a separate function `handle_request` to enhance code modularity. `handle_request` retrieves file contents based on the provided request.
- **Pattern Matching for Response Selection:**
Used pattern matching to determine the appropriate response based on the request type. Responds with "hello.html" content for valid requests and "404.html" content for others.

### Reflection:
#### Enhanced Clarity and Modularity:
Separating request handling into a dedicated function improves code organization and readability.

#### Improved Scalability and Reusability:
The refactoring enables easier extension for handling additional request types without modifying core logic.