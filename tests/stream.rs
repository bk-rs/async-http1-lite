use std::io;

use futures_lite::future::block_on;
use futures_util::io::Cursor;
use futures_util::io::{AsyncReadExt, AsyncWriteExt};

use async_http1_lite::stream::{Http1ClientStream, Http1ServerStream};

#[test]
fn client_get_ref() -> io::Result<()> {
    let cursor = Cursor::new(vec![]);

    let stream = Http1ClientStream::new(cursor);

    assert_eq!(stream.get_ref().get_ref(), &[]);

    Ok(())
}

#[test]
fn client_get_mut() -> io::Result<()> {
    let cursor = Cursor::new(vec![]);

    let mut stream = Http1ClientStream::new(cursor);

    assert_eq!(stream.get_mut().get_mut(), &mut []);

    Ok(())
}

#[test]
fn client_into_inner() -> io::Result<()> {
    let cursor = Cursor::new(vec![]);

    let stream = Http1ClientStream::new(cursor);

    assert_eq!(stream.into_inner()?.into_inner(), []);

    Ok(())
}

#[test]
fn client_read_and_write() -> io::Result<()> {
    block_on(async {
        let cursor = Cursor::new(b"foo".to_vec());

        let mut stream = Http1ClientStream::new(cursor);

        let mut buf = vec![0u8; 5];
        let n = stream.read(&mut buf).await?;
        assert_eq!(n, 3);
        assert_eq!(buf, b"foo\0\0");

        stream.write(b"bar").await?;

        Ok(())
    })
}

#[test]
fn server_get_ref() -> io::Result<()> {
    let cursor = Cursor::new(vec![]);

    let stream = Http1ServerStream::new(cursor);

    assert_eq!(stream.get_ref().get_ref(), &[]);

    Ok(())
}

#[test]
fn server_get_mut() -> io::Result<()> {
    let cursor = Cursor::new(vec![]);

    let mut stream = Http1ServerStream::new(cursor);

    assert_eq!(stream.get_mut().get_mut(), &mut []);

    Ok(())
}

#[test]
fn server_into_inner() -> io::Result<()> {
    let cursor = Cursor::new(vec![]);

    let stream = Http1ServerStream::new(cursor);

    assert_eq!(stream.into_inner()?.into_inner(), []);

    Ok(())
}

#[test]
fn server_read_and_write() -> io::Result<()> {
    block_on(async {
        let cursor = Cursor::new(b"foo".to_vec());

        let mut stream = Http1ServerStream::new(cursor);

        let mut buf = vec![0u8; 5];
        let n = stream.read(&mut buf).await?;
        assert_eq!(n, 3);
        assert_eq!(buf, b"foo\0\0");

        stream.write(b"bar").await?;

        Ok(())
    })
}
