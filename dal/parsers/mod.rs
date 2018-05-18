pub mod blocks;
pub mod certifications;
pub mod excluded;
pub mod identities;
pub mod memberships;
pub mod revoked;
pub mod transactions;

#[cfg(test)]
mod tests {
    use super::transactions::*;
    use duniter_crypto::keys::{PublicKey, Signature};
    use duniter_documents::blockchain::v10::documents::transaction::*;
    use duniter_documents::blockchain::DocumentBuilder;
    use duniter_documents::Blockstamp;

    #[test]
    fn parse_json_tx() {
        let tx_json = json!({
            "version": 10,
            "currency": "g1",
            "locktime": 0,
            "hash": "3424206EF64C69E5F8C3906AAE571E378A498FCDAE0B85E9405A5205D7148EFE",
            "blockstamp": "112533-000002150F2E805E604D9B31212D079570AAD8D3A4D8BB75F2C15A94A345B6B1",
            "blockstampTime": 0,
            "issuers": [
                "51EFVNZwpfmTXU7BSLpeh3PZFgfdmm5hq5MzCDopdH2"
            ],
            "inputs": [
                "1000:0:D:51EFVNZwpfmTXU7BSLpeh3PZFgfdmm5hq5MzCDopdH2:46496"
            ],
            "outputs": [
                "1000:0:SIG(2yN8BRSkARcqE8NCxKMBiHfTpx1EvwULFn56Myf6qRmy)"
            ],
            "unlocks": [
                "0:SIG(0)"
            ],
            "signatures": [
                "5olrjFylTCsVq8I5Yr7FpXeviynICyvIwe1yG5N0RJF+VZb+bCFBnLAMpmMCU2qzUvK7z41UXOrMRybXiLa2Dw=="
            ],
            "comment": "Merci pour la calligraphie ;) de Liam"
        });

        let tx_builder = TransactionDocumentBuilder {
            currency: "g1",
            blockstamp: &Blockstamp::from_string(
                "112533-000002150F2E805E604D9B31212D079570AAD8D3A4D8BB75F2C15A94A345B6B1",
            ).unwrap(),
            locktime: &0,
            issuers: &vec![
                PublicKey::from_base58("51EFVNZwpfmTXU7BSLpeh3PZFgfdmm5hq5MzCDopdH2").unwrap(),
            ],
            inputs: &vec![
                TransactionInput::parse_from_str(
                    "1000:0:D:51EFVNZwpfmTXU7BSLpeh3PZFgfdmm5hq5MzCDopdH2:46496",
                ).unwrap(),
            ],
            outputs: &vec![
                TransactionOutput::parse_from_str(
                    "1000:0:SIG(2yN8BRSkARcqE8NCxKMBiHfTpx1EvwULFn56Myf6qRmy)",
                ).unwrap(),
            ],
            unlocks: &vec![TransactionInputUnlocks::parse_from_str("0:SIG(0)").unwrap()],
            comment: "Merci pour la calligraphie ;) de Liam",
        };

        assert_eq!(
            parse_transaction("g1", &tx_json).expect("Fail to parse transaction !"),
            tx_builder.build_with_signature(vec![Signature::from_base64("5olrjFylTCsVq8I5Yr7FpXeviynICyvIwe1yG5N0RJF+VZb+bCFBnLAMpmMCU2qzUvK7z41UXOrMRybXiLa2Dw==").unwrap()])
        );
    }

    #[test]
    fn parse_json_tx2() {
        let tx_json = json!({
            "version": 10,
            "currency": "g1",
            "locktime": 0,
            "hash": "F98BF7A8BF82E76F5B69E70CEF0A07A08BFDB03561955EC57B254DB1E958529C",
            "blockstamp": "58-00005B9167EBA1E32C6EAD42AE7F72D8F14B765D3C9E47D233B553D47C5AEE0C",
            "blockstampTime": 1488990541,
            "issuers": [
                "FVUFRrk1K5TQGsY7PRLwqHgdHRoHrwb1hcucp4C2N5tD"
            ],
            "inputs": [
                "1000:0:D:FVUFRrk1K5TQGsY7PRLwqHgdHRoHrwb1hcucp4C2N5tD:1"
            ],
            "outputs": [
                "3:0:SIG(7vU9BMDhN6fBuRa2iK3JRbC6pqQKb4qDMGsFcQuT5cz)",
                "997:0:SIG(FVUFRrk1K5TQGsY7PRLwqHgdHRoHrwb1hcucp4C2N5tD)"
            ],
            "unlocks": [
                "0:SIG(0)"
            ],
            "signatures": [
                "VWbvsiybM4L2X5+o+6lIiuKNw5KrD1yGZqmV+lHtA28XoRUFzochSIgfoUqBsTAaYEHY45vSX917LDXudTEzBg=="
            ],
            "comment": "Un petit cafe ;-)"
        });

        let tx_builder = TransactionDocumentBuilder {
            currency: "g1",
            blockstamp: &Blockstamp::from_string(
                "58-00005B9167EBA1E32C6EAD42AE7F72D8F14B765D3C9E47D233B553D47C5AEE0C",
            ).unwrap(),
            locktime: &0,
            issuers: &vec![
                PublicKey::from_base58("FVUFRrk1K5TQGsY7PRLwqHgdHRoHrwb1hcucp4C2N5tD").unwrap(),
            ],
            inputs: &vec![
                TransactionInput::parse_from_str(
                    "1000:0:D:FVUFRrk1K5TQGsY7PRLwqHgdHRoHrwb1hcucp4C2N5tD:1",
                ).unwrap(),
            ],
            outputs: &vec![
                TransactionOutput::parse_from_str(
                    "3:0:SIG(7vU9BMDhN6fBuRa2iK3JRbC6pqQKb4qDMGsFcQuT5cz)",
                ).unwrap(),
                TransactionOutput::parse_from_str(
                    "997:0:SIG(FVUFRrk1K5TQGsY7PRLwqHgdHRoHrwb1hcucp4C2N5tD)",
                ).unwrap(),
            ],
            unlocks: &vec![TransactionInputUnlocks::parse_from_str("0:SIG(0)").unwrap()],
            comment: "Un petit cafe ;-)",
        };

        assert_eq!(
            parse_transaction("g1", &tx_json).expect("Fail to parse transaction !"),
            tx_builder.build_with_signature(vec![Signature::from_base64("VWbvsiybM4L2X5+o+6lIiuKNw5KrD1yGZqmV+lHtA28XoRUFzochSIgfoUqBsTAaYEHY45vSX917LDXudTEzBg==").unwrap()])
        );
    }
}
