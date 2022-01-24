use std::{error::Error, fmt::Display, num::ParseIntError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[derive(Debug)]
pub enum ParseVersionError {
    IncorrectNumberOfVersionSegments(usize),
    FailedToParseVersionInt(ParseIntError),
}

impl Error for ParseVersionError {}

impl From<ParseIntError> for ParseVersionError {
    fn from(e: ParseIntError) -> Self {
        ParseVersionError::FailedToParseVersionInt(e)
    }
}

impl Display for ParseVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseVersionError::IncorrectNumberOfVersionSegments(num_segments) => {
                writeln!(f, "Expected 3 segments, got {}", num_segments)
            }
            ParseVersionError::FailedToParseVersionInt(e) => {
                writeln!(f, "Failed to parse version segment to int: {}", e)
            }
        }
    }
}

fn try_version_from_str<S: AsRef<str>>(version_str: S) -> Result<Version, ParseVersionError> {
    let parts = version_str.as_ref().split('.').collect::<Vec<&str>>();
    if parts.len() != 3 {
        return Err(ParseVersionError::IncorrectNumberOfVersionSegments(
            parts.len(),
        ));
    }
    let major: u8 = parts[0].parse()?;
    let minor: u8 = parts[1].parse()?;
    let patch: u8 = parts[2].parse()?;
    Ok(Version {
        major,
        minor,
        patch,
    })
}

impl TryFrom<String> for Version {
    type Error = ParseVersionError;

    fn try_from(version_str: String) -> Result<Self, Self::Error> {
        try_version_from_str(version_str)
    }
}

impl TryFrom<&str> for Version {
    type Error = ParseVersionError;

    fn try_from(version_str: &str) -> Result<Self, Self::Error> {
        try_version_from_str(version_str)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_patch() {
        let larger = Version::try_from("1.1.2");
        let smaller = Version::try_from("1.1.1");

        assert!(larger.is_ok());
        assert!(smaller.is_ok());

        let larger = larger.unwrap();
        let smaller = smaller.unwrap();
        assert!(larger > smaller);
    }

    #[test]
    fn compare_minor() {
        let larger = Version::try_from("1.2.1");
        let smaller = Version::try_from("1.1.5");

        assert!(larger.is_ok());
        assert!(smaller.is_ok());

        let larger = larger.unwrap();
        let smaller = smaller.unwrap();
        assert!(larger > smaller);
    }

    #[test]
    fn compare_major() {
        let larger = Version::try_from("2.1.1");
        let smaller = Version::try_from("1.10.5");

        assert!(larger.is_ok());
        assert!(smaller.is_ok());

        let larger = larger.unwrap();
        let smaller = smaller.unwrap();
        assert!(larger > smaller);
    }

    #[test]
    fn compare_equal() {
        let version_1 = Version::try_from("2.2.2");
        let version_2 = Version::try_from("2.2.2");

        assert!(version_1.is_ok());
        assert!(version_2.is_ok());

        let version_1 = version_1.unwrap();
        let version_2 = version_2.unwrap();
        assert_eq!(version_1, version_2);
    }
}
