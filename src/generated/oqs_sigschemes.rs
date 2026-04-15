
const MLDSA44_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-mldsa44.der")),
};

/// mldsa44 signatures
pub static MLDSA44: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: MLDSA44_ID,
    signature_alg_id: MLDSA44_ID,
    verification_alg: VerificationAlgorithm::Oqs(&oqs::sig::Algorithm::MlDsa44),
};

const MLDSA65_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-mldsa65.der")),
};

/// mldsa65 signatures
pub static MLDSA65: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: MLDSA65_ID,
    signature_alg_id: MLDSA65_ID,
    verification_alg: VerificationAlgorithm::Oqs(&oqs::sig::Algorithm::MlDsa65),
};

const MLDSA87_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-mldsa87.der")),
};

/// mldsa87 signatures
pub static MLDSA87: SignatureAlgorithm = SignatureAlgorithm {
    public_key_alg_id: MLDSA87_ID,
    signature_alg_id: MLDSA87_ID,
    verification_alg: VerificationAlgorithm::Oqs(&oqs::sig::Algorithm::MlDsa87),
};
