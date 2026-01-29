#[derive(Debug)]
pub struct Params {
    // Public parameters
    p: u32, // modulus
    g: u32, // base
}

#[derive(Debug)]
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
    fn init(
        secret: u32, 
        public: Option<u32>,
        shared: Option<u32>
    ) -> Participant {
        Participant { secret, public, shared }
    }
}

fn public_value(
    params: &Params, 
    participant: &Participant
) ->  Participant {
    // public = (g ^ secret) mod p
    let public: Option<u32> = Some(
        params.g.pow( participant.secret ) % params.p
    );

    let new: Participant = Participant::init(
        participant.secret, public, None
    );
    
    new
}

fn shared_secret(
    params: &Params,
    target: &Participant,
    other: &Participant
) -> Participant {
    // s = (B ^ a) mod p and s = (A ^ b) mod p
    let shared: Option<u32>  = Some(
        other.public.expect("None!").pow(target.secret) % params.p
    );

    let new: Participant = Participant::init(
        target.secret, target.public, shared
    );

    new
}

pub fn key_agreement(
    p: u32, 
    g: u32
) {
    let params: Params = Params::init(p, g);
    let alice_0: Participant = Participant::init(4, None, None);
    let bob_0: Participant = Participant::init(3, None, None);

    println!("initialized: {:?}", params);
    println!("Alice: {:?}", alice_0);
    println!("Bob: {:?}", bob_0);

    let alice_1: Participant =  public_value(&params, &alice_0);
    let bob_1: Participant = public_value(&params, &bob_0);
    println!("Public values calculated: {:?} {:?}", alice_1.public, bob_1.public);

    let alice_2: Participant = shared_secret(&params, &alice_1, &bob_1);
    let bob_2: Participant = shared_secret(&params, &bob_1, &alice_1);
    println!("Shared secrets calculated: {:?} {:?}", alice_2.shared, bob_2.shared);
}