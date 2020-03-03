use std::{error, fmt, io, num, result};
use std::fs::File;
use std::io::{BufReader, Read};

enum AgeReaderError {
    Io(io::Error),
    Parse(num::ParseIntError),
    NegativeAge(),
}

type Result<T> = result::Result<T, AgeReaderError>;

impl error::Error for AgeReaderError {
    fn to_string(&self) -> &str {
        match *self {
            AgeReaderError::Io(ref err) => err.description(),
            AgeReaderError::Parse(ref err) => err.description(),
            AgeReaderError::NegativeAge() => "age is negtive",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AgeReaderError::Io(ref err) => Some(err),
            AgeReaderError::Parse(ref err) => Some(err),
            AgeReaderError::NegativeAge() => None,
        }
    }
}

// impl fmt::Debug for AgeReaderError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<(), fmt::Error> {

//     }
// }

impl fmt::Display for AgeReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AgeReaderError::Io(ref err) => write!(f, "IO error: {}", err),
            AgeReaderError::Parse(ref err) => write!(f, "Parse error: {}", err),
            AgeReaderError::NegativeAge() => write!(f, "Logic error: Age cannot be negtive"),
        }
    }
}

impl From<io::Error> for AgeReaderError {
    fn from(err: io::Error) -> AgeReaderError {
        AgeReaderError::Io(err)
    }
}

impl From<num::ParseIntError> for AgeReaderError {
    fn from(err: num::ParseIntError) -> AgeReaderError {
        AgeReaderError::Parse(err)
    }
}




fn main() {}