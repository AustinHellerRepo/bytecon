use std::error::Error;
use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio_rustls::TlsStream;
use crate::{ByteConverter, ByteStreamReaderAsync, ByteStreamWriterAsync};

impl<TAsyncRead: AsyncRead + std::marker::Unpin> ByteStreamReaderAsync for TAsyncRead {
    async fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        let mut chunk = [0u8; 64];

        let mut initial_packet = [0u8; 8];
        let read_result = self.read_exact(&mut initial_packet)
            .await;
        if let Err(error) = read_result {
            let result: Result<T, Box<dyn Error>> = Err(Box::new(error));
            return result;
        }

        let expected_bytes_length: u64 = u64::from_le_bytes(initial_packet);
        while (bytes.len() as u64) < expected_bytes_length {
            let read_bytes_length_result = self.read(&mut chunk)
                .await;
            if let Err(error) = read_bytes_length_result {
                let result: Result<T, Box<dyn Error>> = Err(Box::new(error));
                return result;
            }

            let read_bytes_length = read_bytes_length_result.unwrap();

            if read_bytes_length != 0 {
                bytes.extend_from_slice(&chunk[..read_bytes_length]);
            }
        }

        let mut index = 0;
        T::extract_from_bytes(&bytes, &mut index)
    }
}

impl ByteStreamWriterAsync for TlsStream<TcpStream> {
    async fn write_from_byte_converter(&mut self, byte_converter: impl crate::ByteConverter) -> Result<(), Box<dyn Error>> {
        let mut stream_bytes = Vec::new();
        byte_converter.append_to_bytes(&mut stream_bytes)?;
        self.write(&stream_bytes)
            .await?;
        Ok(())
    }
}

impl ByteStreamReaderAsync for tokio::sync::mpsc::Receiver<Vec<u8>> {
    async fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn Error>> {
        let bytes = self.recv()
            .await
            .ok_or(TokioByteConError::OptionVariantNoneReceivedFromReceiver)?;
        let mut index = 0;
        T::extract_from_bytes(&bytes, &mut index)
    }
}

impl ByteStreamWriterAsync for tokio::sync::mpsc::Sender<Vec<u8>> {
    async fn write_from_byte_converter(&mut self, byte_converter: impl ByteConverter) -> Result<(), Box<dyn Error>> {
        let mut bytes = Vec::new();
        byte_converter.append_to_bytes(&mut bytes)?;
        self.send(bytes)
            .await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
enum TokioByteConError {
    #[error("Option variant None received from receiver.")]
    OptionVariantNoneReceivedFromReceiver,
}