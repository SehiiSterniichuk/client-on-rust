use std::str::FromStr;

#[derive(Debug)]
pub enum Status {
    WAITING, RUNNING, DONE
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DONE" => Ok(Status::DONE),
            "RUNNING" => Ok(Status::RUNNING),
            "WAITING" => Ok(Status::WAITING),
            _ => Err(()),
        }
    }
}
