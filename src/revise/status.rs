use std::fmt::Formatter;

use anyhow::anyhow;

#[derive(Clone, Default, Debug)]
pub enum Status {
    #[default]
    Submit,
    Abort,
    Edit,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Submit => "Submit",
            Self::Abort => "Abort",
            Self::Edit => "Edit",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "y" | "yes" | "" => Ok(Self::Submit),
            "n" | "no" => Ok(Self::Abort),
            "e" | "edit" => Ok(Self::Edit),
            &_ => Err(anyhow!("input error")),
        }
    }
}
