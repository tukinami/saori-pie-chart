use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum PieChartError {
    Success,
    NotEnoughArguments,
    InvalidArgumentKind(usize, ArgumentKind),
    UsvgError(resvg::usvg::Error),
    IoError(std::io::Error),
    EncodingError(png::EncodingError),
    SizeTooSmall,
    InvalidData,
}

#[derive(Debug)]
pub(crate) enum ArgumentKind {
    Integer,
    FloatNumber,
}

impl Display for PieChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "0"),
            Self::NotEnoughArguments => write!(f, "Error: arguments are less than 9."),
            Self::InvalidArgumentKind(number, expect) => {
                write!(f, "Error: arguments of {number} expect {expect}.")
            }
            Self::UsvgError(e) => write!(f, "Error: {e}."),
            Self::IoError(e) => write!(f, "Error: {e}."),
            Self::EncodingError(e) => write!(f, "Error: {e}."),
            Self::SizeTooSmall => write!(f, "Error: size too small."),
            Self::InvalidData => write!(f, "Error: invalid data."),
        }
    }
}

impl From<resvg::usvg::Error> for PieChartError {
    fn from(value: resvg::usvg::Error) -> Self {
        Self::UsvgError(value)
    }
}

impl From<std::io::Error> for PieChartError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<png::EncodingError> for PieChartError {
    fn from(value: png::EncodingError) -> Self {
        Self::EncodingError(value)
    }
}

impl Display for ArgumentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind_str = match self {
            Self::Integer => "integer",
            Self::FloatNumber => "float number",
        };
        write!(f, "{kind_str}")
    }
}
