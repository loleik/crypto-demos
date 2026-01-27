#[derive(Debug)]
pub struct Params {
    // Public parameters
    p: u32, // modulus
    g: u32, // base

    // Secret integers
    a: Option<u32>,
    b: Option<u32>,

    // Public values
    pub_a: Option<u32>,
    pub_b: Option<u32>,

    // Shared secrets
    s_a: Option<u32>,
    s_b: Option<u32>,
}

impl Params {
    fn init(p: u32, g: u32) -> Params {
        Params { 
            p: p, g: g, 
            a: None, b: None, pub_a: None, pub_b: None, s_a: None, s_b: None 
        }
    }
}

pub fn key_agreement(p: u32, g: u32) -> Params {
    Params::init(p, g)
}