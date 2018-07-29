


// !! Structs meant for Http Requests and Responses.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub text: String,
}

#[derive(Serialize)]
pub struct Health {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hash {
    pub hash: u64,
}


