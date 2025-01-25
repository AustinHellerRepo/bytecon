# bytecon
Allows for the convenient conversion of types from and to arrays of bytes.

# Features
* Default
  * Usage of the ByteCon trait allows for appending and extracting bytes from a byte array. This is very useful for interacting with a byte stream.
* `"burn"`
  * Contains implementations for the `burn` crate.
    * Excludes: `DType`
* `"burn_dtype"`
  * Contains the specific implementation for the `burn` crate's `DType` type since it can conflict with the `"bincode"` feature.
  * Only use this feature if you are NOT using `"bincode"`.
* `"tokio"`
  * Contains implementations for the `tokio` and `tokio-rustls` crate.
    * `TlsStream<T: AsyncWrite + AsyncRead + Unpin>`
    * `Sender<Vec<u8>>`
    * `Receiver<Vec<u8>>`
* `"bincode"`
  * Contains implementations for the `bincode` crate.
  * This feature conflicts with the `"burn_dtype"` feature.
* `"rand"`
  * Contains implementations for the `rand` and `rand_chacha` crates.
* `"rustls"`
  * Contains implementations for the `rustls` crate
* `"bevy"`
  * Contains implementations for the `bevy` crate, version 0.15
 
# Coming soon
* Additional implementations for other common crates
  * Feel free to add issues for your favorite crates
