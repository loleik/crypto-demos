#[derive(Debug)]
pub struct Params {
    // Public parameters
    p: u32, // modulus
    g: u32, // base

    // Secret integers
    a: u32,
    b: u32,

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
            a: 4, b: 3, 
            pub_a: None, pub_b: None, s_a: None, s_b: None 
        }
    }
}

fn public_values(params: &mut Params) -> &mut Params {
    // A = (g ^ a) mod p
    params.pub_a = Some( params.g.pow(params.a) % params.p );
    // B = (g ^ b) mod p
    params.pub_b = Some( params.g.pow(params.b) % params.p );

    println!("pub_a / pub_b calculated: {:?}", params);

    params
}

fn shared_secrets(params: &mut Params) -> &mut Params {
    // s = (B ^ a) mod p
    params.s_a = Some( params.pub_b.expect("None!").pow(params.a) % params.p );
    // s = (A ^ b) mod p
    params.s_b = Some( params.pub_a.expect("None!").pow(params.b) % params.p );

    println!("s_a / s_b calculated: {:?}", params);

    params
}

pub fn key_agreement(p: u32, g: u32) -> Result<(),()> {
    let mut params: Params = Params::init(p, g);

    println!("initialized: {:?}", params);

    public_values(&mut params);

    shared_secrets(&mut params);

    Ok(())
}