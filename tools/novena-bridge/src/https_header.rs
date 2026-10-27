//! Feste Aufnahme vor Hyper; Statuszeile und Abschluss zählen mit.
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
pub(super) const MAX_HEADER_BYTES: usize = 32 * 1024;
pub(super) struct HeaderGate<T> {
    inner: T,
    header: Box<[u8; MAX_HEADER_BYTES]>,
    length: usize,
    sent: usize,
    complete: bool,
    failed: bool,
}
impl<T> HeaderGate<T> {
    pub(super) fn new(inner: T) -> Self {
        Self {
            inner,
            header: Box::new([0; MAX_HEADER_BYTES]),
            length: 0,
            sent: 0,
            complete: false,
            failed: false,
        }
    }
    fn reject(&mut self) -> Poll<io::Result<()>> {
        self.failed = true;
        Poll::Ready(Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "HTTP_HEADER_REJECTED",
        )))
    }
}
impl<T: AsyncRead + Unpin> AsyncRead for HeaderGate<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        out: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        if this.failed {
            return this.reject();
        }
        if out.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }
        while !this.complete {
            if this.length == MAX_HEADER_BYTES {
                return this.reject();
            }
            // Kein unvalidiertes Byte erreicht Hyper. TLS selbst besitzt
            // getrennte, durch rustls begrenzte Recordpuffer.
            let mut byte = [0u8; 1];
            let mut read = ReadBuf::new(&mut byte);
            match Pin::new(&mut this.inner).poll_read(cx, &mut read) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(e)) => {
                    this.failed = true;
                    return Poll::Ready(Err(e));
                }
                Poll::Ready(Ok(())) => {}
            }
            if read.filled().is_empty() {
                return this.reject();
            }
            this.header[this.length] = byte[0];
            this.length += 1;
            if this.length >= 4 && &this.header[this.length - 4..this.length] == b"\r\n\r\n" {
                // Kein Umgehen der Laufgrenze durch beliebig viele 1xx-Blöcke.
                if !(this.header.starts_with(b"HTTP/1.1 ") || this.header.starts_with(b"HTTP/1.0 "))
                    || !this.header[9..12].iter().all(u8::is_ascii_digit)
                    || this.header[9] == b'1'
                    || !matches!(this.header[12], b' ' | b'\r')
                {
                    return this.reject();
                }
                this.complete = true;
            }
        }
        if this.sent < this.length {
            let count = out.remaining().min(this.length - this.sent);
            out.put_slice(&this.header[this.sent..this.sent + count]);
            this.sent += count;
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut this.inner).poll_read(cx, out)
    }
}
impl<T: AsyncWrite + Unpin> AsyncWrite for HeaderGate<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_write(cx, bytes)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_shutdown(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    fn reply(length: usize) -> Vec<u8> {
        let mut h = b"HTTP/1.1 200 OK\r\nX-Test: ".to_vec();
        let end = b"\r\nContent-Length: 2\r\n\r\n";
        h.resize(length - end.len(), b'x');
        h.extend_from_slice(end);
        h.extend_from_slice(b"{}");
        h
    }
    // Produktionsgate und Hyperparser, speicherlokale Duplexleitung ohne Socket.
    fn parse(bytes: Vec<u8>) -> Result<Vec<u8>, ()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(async {
                let (client, mut server) = tokio::io::duplex(1024);
                let peer = tokio::spawn(async move {
                    let mut request = [0u8; 1024];
                    let _ = server.read(&mut request).await;
                    let _ = server.write_all(&bytes).await;
                });
                let (mut sender, connection) = hyper::client::conn::http1::Builder::new()
                    .max_headers(100)
                    .max_buf_size(MAX_HEADER_BYTES)
                    .handshake(hyper_util::rt::TokioIo::new(HeaderGate::new(client)))
                    .await
                    .expect("Parser muss ohne Timeout entscheiden");
                let driver = tokio::spawn(connection);
                let work = async {
                    let response = sender
                        .send_request(
                            hyper::Request::builder()
                                .uri("/")
                                .body(http_body_util::Empty::<bytes::Bytes>::new())
                                .unwrap(),
                        )
                        .await
                        .map_err(|_| ())?;
                    assert_eq!(response.status(), 200);
                    response
                        .into_body()
                        .collect()
                        .await
                        .map(|b| b.to_bytes().to_vec())
                        .map_err(|_| ())
                };
                let result = tokio::time::timeout(std::time::Duration::from_secs(2), work)
                    .await
                    .expect("Parser muss ohne Timeout entscheiden");
                driver.abort();
                peer.abort();
                result
            })
    }
    #[test]
    fn header_exact_boundary_before_parser() {
        for n in [MAX_HEADER_BYTES - 1, MAX_HEADER_BYTES] {
            assert_eq!(parse(reply(n)), Ok(b"{}".to_vec()));
        }
        for n in [MAX_HEADER_BYTES + 1, 64 * 1024] {
            assert!(parse(reply(n)).is_err());
        }
    }
    #[test]
    fn header_truncation_status_and_lengths_fail_closed() {
        for b in [
            b"HTTP/1.1 200 OK\r\nX: partial".as_slice(),
            b"HTTP/1.1 INVALID\r\n\r\n",
            b"HTTP/1.1  100 Continue\r\n\r\n",
            b"HTTP/1.1 200 OK\r\nContent-Length: -1\r\n\r\n{}",
            b"HTTP/1.1 200 OK\r\nContent-Length: nope\r\n\r\n{}",
            b"HTTP/1.1 200 OK\r\nContent-Length: 18446744073709551616\r\n\r\n{}",
            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Length: 3\r\n\r\n{}",
            b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\n{}",
            b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}",
        ] {
            assert!(parse(b.to_vec()).is_err(), "{b:?}");
        }
    }
    #[test]
    fn gate_never_releases_partial_header_or_reads_over_limit() {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async {
                let input = reply(MAX_HEADER_BYTES + 1);
                let mut gate = HeaderGate::new(input.as_slice());
                let mut out = Vec::new();
                assert!(gate.read_to_end(&mut out).await.is_err());
                assert!(out.is_empty());
                assert_eq!(gate.length, MAX_HEADER_BYTES);
                assert_eq!(gate.inner.len(), input.len() - MAX_HEADER_BYTES);
                assert!(gate.read_to_end(&mut out).await.is_err());
            });
    }
}
