use std::io;

use futures_lite::future::block_on;
use futures_util::io::Cursor;
use http::{Method, Version};

use async_http1_lite::{decoder::Http1RequestDecoder, stream::Http1StreamDecoder};

#[test]
fn request_simple() -> io::Result<()> {
    block_on(async {
        let mut stream = Cursor::new(
            "GET / HTTP/1.1\r\nHost: foo.com\r\n\r\nPOST /x HTTP/1.0\r\nHost: bar.com\r\n\r\n",
        );

        let mut decoder = Http1RequestDecoder::new(1024, None);

        let request = decoder.read_head(&mut stream).await?;
        assert_eq!(request.method(), Method::GET);
        assert_eq!(request.uri(), "/");
        assert_eq!(request.version(), Version::HTTP_11);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.headers().get("host").unwrap().to_str().unwrap(),
            "foo.com"
        );

        let request = decoder.read_head(&mut stream).await?;
        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.uri(), "/x");
        assert_eq!(request.version(), Version::HTTP_10);
        assert_eq!(request.headers().len(), 1);
        assert_eq!(
            request.headers().get("host").unwrap().to_str().unwrap(),
            "bar.com"
        );

        let err = decoder.read_head(&mut stream).await.err().unwrap();
        assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);
        assert_eq!(err.to_string(), "read 0");

        Ok(())
    })
}
