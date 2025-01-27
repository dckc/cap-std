use crate::os::unix::net::UnixStream;
use async_std::io;
use async_std::os::unix;
use async_std::stream::Stream;
use async_std::task::{Context, Poll};
use cap_primitives::{ambient_authority, AmbientAuthority};
use std::fmt;
use std::pin::Pin;

/// An iterator over incoming connections to a [`UnixListener`].
///
/// This corresponds to [`async_std::os::unix::net::Incoming`].
///
/// [`async_std::os::unix::net::Incoming`]: https://docs.rs/async-std/latest/async_std/os/unix/net/struct.Incoming.html
/// [`UnixListener`]: struct.UnixListener.html
pub struct Incoming<'a> {
    std: unix::net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given
    /// `async_std::os::unix::net::Incoming`.
    ///
    /// # Ambient Authority
    ///
    /// `async_std::net::Incoming` is not sandboxed and may access any address
    /// that the host process has access to.
    #[inline]
    pub fn from_std(std: unix::net::Incoming<'a>, _: AmbientAuthority) -> Self {
        Self { std }
    }
}

impl<'a> Stream for Incoming<'a> {
    type Item = io::Result<UnixStream>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.std), cx).map(|poll| {
            poll.map(|result| {
                let unix_stream = result?;
                Ok(UnixStream::from_std(unix_stream, ambient_authority()))
            })
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.std.size_hint()
    }
}

impl<'a> fmt::Debug for Incoming<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
