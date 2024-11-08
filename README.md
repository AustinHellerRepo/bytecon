# bytecon
Allows for the convenient conversion of types from and to arrays of bytes.

# Features
* Default
  * Usage of the ByteCon trait allows for appending and extracting bytes from a byte array. This is very useful for interacting with a byte stream.
* `"burn"`
  * Contains implementations for the `burn` crate.
* `"tokio"`
  * Contains implementations for the `tokio` crate.
    * `TlsStream<T: AsyncWrite + AsyncRead + Unpin>
    * `Sender<Vec<u8>>`
    * `Receiver<Vec<u8>>`
 
# Coming soon
* Additional implementations for other common crates
  * Feel free to add issues for your favorite crates
