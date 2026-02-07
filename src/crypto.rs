
pub struct signature(ECDSASignature<Secp256k1>);
pub struct public_key(Veryfyingkey<Secp256k1>);
pub struct private_key(SigningKey<Secp256k1>);
