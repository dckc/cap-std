use crate::os::unix::net::UnixStream;
use cap_primitives::{ambient_authority, AmbientAuthority};
use std::os::unix;
use std::{fmt, io};

/// An iterator over incoming connections to a [`UnixListener`].
///
/// This corresponds to [`std::os::unix::net::Incoming`].
///
/// [`std::os::unix::net::Incoming`]: https://doc.rust-lang.org/std/os/unix/net/struct.Incoming.html
/// [`UnixListener`]: struct.UnixListener.html
pub struct Incoming<'a> {
    std: unix::net::Incoming<'a>,
}

impl<'a> Incoming<'a> {
    /// Constructs a new instance of `Self` from the given
    /// `std::os::unix::net::Incoming`.
    ///
    /// # Ambient Authority
    ///
    /// `std::net::Incoming` is not sandboxed and may access any address that
    /// the host process has access to.
    #[inline]
    pub fn from_std(std: unix::net::Incoming<'a>, _: AmbientAuthority) -> Self {
        Self { std }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<UnixStream>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.std.next().map(|result| {
            let unix_stream = result?;
            Ok(UnixStream::from_std(unix_stream, ambient_authority()))
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
