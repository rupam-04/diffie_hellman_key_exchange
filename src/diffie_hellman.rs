use crate::u256::RD256;
use crate::secp256k1::{Point, SECP256K1};

pub struct User {
    pub private_key: RD256,
    pub public_key: Point,
    pub shared_secret: Point
}

impl User {
    pub fn generate_public_key(&mut self) {
        self.public_key = SECP256K1::mul_point(&SECP256K1::g(), &self.private_key.clone());
    }

    pub fn generate_shared_secret(mut a: User, mut b: User) {
        a.shared_secret = SECP256K1::mul_point(&b.public_key.clone(), &a.private_key.clone());
        b.shared_secret = SECP256K1::mul_point(&a.public_key.clone(), &b.private_key.clone());
    }
}

