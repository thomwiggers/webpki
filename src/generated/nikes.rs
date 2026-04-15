
const CTIDH512_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-ctidh512.der")),
};

/// ctidh512 NIKE
pub static CTIDH512: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CTIDH512_ID,
    alg: NikeImpl::Ctidh(CtidhAlg::ctidh512),
};
