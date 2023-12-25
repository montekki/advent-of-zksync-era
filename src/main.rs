use ethers::{
    abi::AbiEncode,
    contract::abigen,
    types::Bytes,
    types::{H256, U256},
    utils::hex,
};

abigen!(ZkSync, "./IZkSync.json");

fn main() {
    let factory_deps_batch_1 = include_str!("./factory_deps_1");
    let call = CommitBatchesCall {
        last_committed_block_data: StoredBlockInfo {
            block_number: 363172,
            block_hash: "f77a6dd85db62ad3d5178d41f5fbff72bfcb39bd0fd549472b45dcf6e44f7d44"
                .parse::<H256>()
                .unwrap()
                .into(),
            index_repeated_storage_changes: 190635923,
            number_of_layer_1_txs: U256::zero(),
            priority_operations_hash:
                "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
                    .parse::<H256>()
                    .unwrap()
                    .into(),
            l_2_logs_tree_root: "6b66978051fa94f8174f16156ffbd5b626ec8445da1c7604003cb20f896d9b7f"
                .parse::<H256>()
                .unwrap()
                .into(),
            timestamp: 1703482757u64.into(),
            commitment: "6bb166e1545bd5b191f8d3196d040e3b04e1acc7238bf233cd71e2a6cb8e2901"
                .parse::<H256>()
                .unwrap()
                .into(),
        },
        new_blocks_data: vec![CommitBlockInfo {
            block_number: 363173,
            timestamp: 1703482802u64.into(),
            index_repeated_storage_changes: 190636483,
            new_state_root: "539baf27461c2bfc7d59cb74fcb89815da3961a24cbf564161562892add72fba"
                .parse::<H256>()
                .unwrap()
                .into(),
            number_of_layer_1_txs: U256::zero(),
            l_2_logs_tree_root: "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
                .parse::<H256>()
                .unwrap()
                .into(),

            priority_operations_hash:
                "4b89316f38ff21f56d9d6ef072e8b6449baa40abc0390888028e5695a12d7ecf"
                    .parse::<H256>()
                    .unwrap()
                    .into(),
            initial_storage_changes: hex::decode(
                "468c3f5288853d76c9b693654147902526e16a5817e0ce1b855adb6a97b5f4cd",
            )
            .unwrap()
            .into(),
            repeated_storage_changes: Bytes::new().into(),
            l_2_logs: Bytes::new(),
            l_2_arbitrary_length_messages: vec![Bytes::new()],

            factory_deps: vec![Bytes::new()], // TODO: what is this blob? hex::decode(factory_deps_batch_1).unwrap(),
        }],
    };
    println!("Hello, world! {}", hex::encode(call.encode()));
}
