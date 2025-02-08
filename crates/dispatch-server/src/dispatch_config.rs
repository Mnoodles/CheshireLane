use proto::p10::Serverinfo;

#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct Version(pub Vec<String>);

#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct Servers(pub Vec<Serverinfo>);
