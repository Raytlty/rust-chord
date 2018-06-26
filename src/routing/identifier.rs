use bigint::U256;
use ring::digest;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

pub struct Identifier(U256);

impl Identifier {
    pub fn new(identifier: &[u8]) -> Self {
        Identifier(U256::from_big_endian(identifier))
    }

    fn generate(bytes: &[u8]) -> Self {
        let dig = digest::digest(&digest::SHA256, bytes);
        Self::new(dig.as_ref())
    }

    pub fn is_between(&self, first: &Identifier, second: &Identifier) -> bool {
        let (diff1, _) = second.0.overflowing_sub(self.0);
        let (diff2, _) = second.0.overflowing_sub(first.0);

        diff1 < diff2
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes = [0; 32];
        self.0.to_big_endian(&mut bytes);
        bytes
    }
}

pub trait Identify {
    fn get_identifier(&self) -> Identifier;
}

impl Identify for SocketAddrV4 {
    fn get_identifier(&self) -> Identifier {
        Identifier::generate(self.ip().octets().as_ref())
    }
}

impl Identify for SocketAddrV6 {
    fn get_identifier(&self) -> Identifier {
        Identifier::generate(self.ip().octets().as_ref())
    }
}

impl Identify for SocketAddr {
    fn get_identifier(&self) -> Identifier {
        match self {
            SocketAddr::V4(v4) => v4.get_identifier(),
            SocketAddr::V6(v6) => v6.get_identifier()
        }
    }
}

impl Identify for [u8; 32] {
    fn get_identifier(&self) -> Identifier {
        Identifier::generate(self.as_ref())
    }
}

pub struct IdentifierValue<T> {
    value: T,
    identifier: Identifier
}

impl<T: Identify> IdentifierValue<T> {
    pub fn new(value: T) -> Self {
        let identifier = value.get_identifier();

        Self { value, identifier }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_identifier(&self) -> &Identifier {
        &self.identifier
    }
}