use hyperx::{
    header::{Formatter as HeaderFormatter, Header, RawLike},
    Error as HeaderError, Result as HyperResult,
};
use std::{fmt::Result as FmtResult, str::from_utf8};

#[derive(Debug, Copy, Clone, PartialEq)]
/// Message format version, defined in [RFC2045](https://tools.ietf.org/html/rfc2045#section-4)
pub struct MimeVersion {
    major: u8,
    minor: u8,
}

pub const MIME_VERSION_1_0: MimeVersion = MimeVersion::new(1, 0);

impl MimeVersion {
    pub const fn new(major: u8, minor: u8) -> Self {
        MimeVersion { major, minor }
    }

    #[inline]
    pub const fn major(self) -> u8 {
        self.major
    }

    #[inline]
    pub const fn minor(self) -> u8 {
        self.minor
    }
}

impl Default for MimeVersion {
    fn default() -> Self {
        MIME_VERSION_1_0
    }
}

impl Header for MimeVersion {
    fn header_name() -> &'static str {
        "MIME-Version"
    }

    fn parse_header<'a, T>(raw: &'a T) -> HyperResult<Self>
    where
        T: RawLike<'a>,
        Self: Sized,
    {
        raw.one().ok_or(HeaderError::Header).and_then(|r| {
            let mut s = from_utf8(r).map_err(|_| HeaderError::Header)?.split('.');

            let major = s.next().ok_or(HeaderError::Header)?;
            let minor = s.next().ok_or(HeaderError::Header)?;
            let major = major.parse().map_err(|_| HeaderError::Header)?;
            let minor = minor.parse().map_err(|_| HeaderError::Header)?;
            Ok(MimeVersion::new(major, minor))
        })
    }

    fn fmt_header(&self, f: &mut HeaderFormatter<'_, '_>) -> FmtResult {
        f.fmt_line(&format!("{}.{}", self.major, self.minor))
    }
}

#[cfg(test)]
mod test {
    use super::{MimeVersion, MIME_VERSION_1_0};
    use hyperx::header::Headers;

    #[test]
    fn format_mime_version() {
        let mut headers = Headers::new();

        headers.set(MIME_VERSION_1_0);

        assert_eq!(format!("{}", headers), "MIME-Version: 1.0\r\n");

        headers.set(MimeVersion::new(0, 1));

        assert_eq!(format!("{}", headers), "MIME-Version: 0.1\r\n");
    }

    #[test]
    fn parse_mime_version() {
        let mut headers = Headers::new();

        headers.set_raw("MIME-Version", "1.0");

        assert_eq!(headers.get::<MimeVersion>(), Some(&MIME_VERSION_1_0));

        headers.set_raw("MIME-Version", "0.1");

        assert_eq!(headers.get::<MimeVersion>(), Some(&MimeVersion::new(0, 1)));
    }
}
