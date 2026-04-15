
const MLKEM512_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-mlkem512.der")),
};

/// mlkem512 KEM
pub static MLKEM512: KemAlgorithm = KemAlgorithm {
    public_key_alg_id: MLKEM512_ID,
    kem: oqs::kem::Algorithm::MlKem512,
};

const MLKEM768_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-mlkem768.der")),
};

/// mlkem768 KEM
pub static MLKEM768: KemAlgorithm = KemAlgorithm {
    public_key_alg_id: MLKEM768_ID,
    kem: oqs::kem::Algorithm::MlKem768,
};

const MLKEM1024_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-mlkem1024.der")),
};

/// mlkem1024 KEM
pub static MLKEM1024: KemAlgorithm = KemAlgorithm {
    public_key_alg_id: MLKEM1024_ID,
    kem: oqs::kem::Algorithm::MlKem1024,
};
