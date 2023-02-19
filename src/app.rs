use std::str::FromStr;

#[derive(PartialEq)]
pub enum Network {
    Bitcoin,
    Ethereum
}
impl FromStr for Network {
    type Err = ();

    fn from_str(input: &str) -> Result<Network, Self::Err> {
        match input {
            "bitcoin" => Ok(Network::Bitcoin),
            "ethereum" => Ok(Network::Ethereum),
            _ => Err(()),
        }
    }
}
impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::Bitcoin => return String::from("Bitcoin"),
            Network::Ethereum => return String::from("Ethereum")
        };
    }
}