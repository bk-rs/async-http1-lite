use std::io;

use futures_util::io::Cursor;

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
