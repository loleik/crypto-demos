mod protocols;

use protocols::dh::*;

fn main() {
    let params:Params = key_agreement(23, 5);

    println!("{:?}", params);
}
