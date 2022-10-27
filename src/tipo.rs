use std::str::FromStr;

pub enum Tipo {
    BEGIN, // los enum pueden contener data!!
    MATCH,
    END,
    SUMMARY,
}

impl FromStr for Tipo {
    type Err = MiError;
    // type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    // fn from_str(s: &str) -> Result<Self, Self::TipoError> {
        /*
        match s {
            "begin" => Self::BEGIN,
            "match" => Self::MATCH,
            "end" => Self::END,
            "summary" => Self::SUMMARY,
        }
        */
        match s {
            "begin" => Ok(Self::BEGIN),
            "match" => Ok(Self::MATCH),
            "end" => Ok(Self::END),
            "summary" => Ok(Self::SUMMARY),
            _ => Err(MiError)
        }
    }
}

pub struct MiError;


