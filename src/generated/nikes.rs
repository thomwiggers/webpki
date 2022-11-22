
const CSIDH2047K221_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CSIDH2047k221.der")),
};

/// CSIDH2047k221 NIKE
pub static CSIDH2047K221: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CSIDH2047K221_ID,
    alg: secsidh::Algorithm::CSIDH2047k221,
};

const CSIDH4095K256_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CSIDH4095k256.der")),
};

/// CSIDH4095k256 NIKE
pub static CSIDH4095K256: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CSIDH4095K256_ID,
    alg: secsidh::Algorithm::CSIDH4095k256,
};

const CSIDH5119K234_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CSIDH5119k234.der")),
};

/// CSIDH5119k234 NIKE
pub static CSIDH5119K234: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CSIDH5119K234_ID,
    alg: secsidh::Algorithm::CSIDH5119k234,
};

const CSIDH6143K256_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CSIDH6143k256.der")),
};

/// CSIDH6143k256 NIKE
pub static CSIDH6143K256: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CSIDH6143K256_ID,
    alg: secsidh::Algorithm::CSIDH6143k256,
};

const CSIDH8191K332_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CSIDH8191k332.der")),
};

/// CSIDH8191k332 NIKE
pub static CSIDH8191K332: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CSIDH8191K332_ID,
    alg: secsidh::Algorithm::CSIDH8191k332,
};

const CSIDH9215K384_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CSIDH9215k384.der")),
};

/// CSIDH9215k384 NIKE
pub static CSIDH9215K384: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CSIDH9215K384_ID,
    alg: secsidh::Algorithm::CSIDH9215k384,
};

const CTIDH2047K221_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CTIDH2047k221.der")),
};

/// CTIDH2047k221 NIKE
pub static CTIDH2047K221: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CTIDH2047K221_ID,
    alg: secsidh::Algorithm::CTIDH2047k221,
};

const CTIDH4095K256_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CTIDH4095k256.der")),
};

/// CTIDH4095k256 NIKE
pub static CTIDH4095K256: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CTIDH4095K256_ID,
    alg: secsidh::Algorithm::CTIDH4095k256,
};

const CTIDH5119K234_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CTIDH5119k234.der")),
};

/// CTIDH5119k234 NIKE
pub static CTIDH5119K234: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CTIDH5119K234_ID,
    alg: secsidh::Algorithm::CTIDH5119k234,
};

const CTIDH6143K256_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CTIDH6143k256.der")),
};

/// CTIDH6143k256 NIKE
pub static CTIDH6143K256: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CTIDH6143K256_ID,
    alg: secsidh::Algorithm::CTIDH6143k256,
};

const CTIDH8191K332_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CTIDH8191k332.der")),
};

/// CTIDH8191k332 NIKE
pub static CTIDH8191K332: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CTIDH8191K332_ID,
    alg: secsidh::Algorithm::CTIDH8191k332,
};

const CTIDH9215K384_ID: AlgorithmIdentifier = AlgorithmIdentifier {
    asn1_id_value: untrusted::Input::from(include_bytes!("../data/alg-CTIDH9215k384.der")),
};

/// CTIDH9215k384 NIKE
pub static CTIDH9215K384: NikeAlgorithm = NikeAlgorithm {
    public_key_alg_id: CTIDH9215K384_ID,
    alg: secsidh::Algorithm::CTIDH9215k384,
};
