#[derive(Debug)]
pub struct Params {
    // Public parameters
    p: u32, // modulus
    g: u32, // base
}

#[derive(Debug, Clone, Copy)]
pub struct Participant {
    secret: u32, // secret value
    public: Option<u32>, // public value
    shared: Option<u32>, // shared secret
}

impl Params {
    fn init(p: u32, g: u32) -> Params {
        Params { p, g }
    }
}

impl Participant {
    fn init(secret: u32) -> Participant {
        Participant { secret, public: None, shared: None }
    }
}

fn public_value<'a>(
    params: &'a Params, 
    participant: &'a mut Participant
) -> &'a mut Participant {
    // public = (g ^ secret) mod p
    participant.public = Some(
        params.g.pow( participant.secret ) % params.p
    );
    
    participant
}

fn shared_secret<'a>(
    params: &'a Params,
    target: &'a mut Participant,
    other: Participant
) -> &'a mut Participant {
    // s = (B ^ a) mod p and s = (A ^ b) mod p
    target.shared = Some(
        other.public.expect("None!").pow(target.secret) % params.p
    );

    target
}

pub fn key_agreement(
    p: u32, 
    g: u32
) {
    let params: Params = Params::init(p, g);
    let mut alice: Participant = Participant::init(4);
    let mut bob: Participant = Participant::init(3);

    println!("initialized: {:?}", params);
    println!("Alice: {:?}", alice);
    println!("Bob: {:?}", bob);

    public_value(&params, &mut alice);
    public_value(&params, &mut bob);
    println!("Public values calculated: {:?} {:?}", alice, bob);

    shared_secret(&params, &mut alice, bob);
    shared_secret(&params, &mut bob, alice);
    println!("Shared secrets calculated: {:?} {:?}", alice, bob);
}