use crate::GetMessageBySrcTxHashResponse;

#[tokio::test]
pub async fn query_sample_lz_tx() {
    let src_tx_hash = "0xc18e9ecf66d598ab7faab6e1d557951238207f92203deef968a1783871b24a4d";
    let src_chain_id = 106;
    let response = crate::get_message_by_src_tx_hash(src_chain_id, src_tx_hash)
        .await
        .unwrap();
    assert_eq!(response.messages.first().unwrap().src_tx_hash, src_tx_hash);
}

#[tokio::test]
pub async fn query_empty_lz_tx() {
    let src_tx_hash = "0x00000";
    let src_chain_id = 101;
    let response = crate::get_message_by_src_tx_hash(src_chain_id, src_tx_hash)
        .await
        .unwrap();
    let expected = GetMessageBySrcTxHashResponse { messages: vec![] };
    assert_eq!(response, expected);
}
