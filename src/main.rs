use alloy_primitives::{hex, Bytes};

use alloy_sol_types::{sol, SolCall};

sol! {

    /// @notice Bridges tokens via Across
    /// @param _bridgeData the core information needed for bridging
    /// @param _acrossData data specific to Across
    ///
    /// https://github.com/lifinance/contracts/blob/b9dcf9d0a6eb57cf7ef45f4af993cc6119b8c814/src/Facets/AcrossFacet.sol
    #[derive(Debug)]
    function startBridgeTokensViaAcross(
        BridgeData memory bridgeData,
        AcrossData calldata acrossData
    );

    /// LiFi Shared BridgeData
    ///
    /// https://github.com/lifinance/contracts/blob/b9dcf9d0a6eb57cf7ef45f4af993cc6119b8c814/src/Interfaces/ILiFi.sol
    #[derive(Debug)]
    struct BridgeData {
        bytes32 transactionId;
        string bridge;
        string integrator;
        address referrer;
        address sendingAssetId;
        address receiver;
        uint256 minAmount;
        uint256 destinationChainId;
        bool hasSourceSwaps;
        bool hasDestinationCall;
    }

    /// @param relayerFeePct The relayer fee in token percentage with 18 decimals.
    /// @param quoteTimestamp The timestamp associated with the suggested fee.
    /// @param message Arbitrary data that can be used to pass additional information to the recipient along with the tokens.
    /// @param maxCount Used to protect the depositor from frontrunning to guarantee their quote remains valid.
    #[derive(Debug)]
    struct AcrossData {
        int64 relayerFeePct;
        uint32 quoteTimestamp;
        bytes message;
        uint256 maxCount;
    }
}

fn main() {
    // Taken from: https://basescan.org/tx/0xd0fb45163e9e4fbef28b50a8a14b663e9c1d2fff7e3e6e0caab4664533d9b8f0
    let data = Bytes::from(hex!("1fd8010c00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000200a2aad37895a1e4ade7481cea635135ad5f0a5b43f02608040050aaf625baca3b000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000001800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000833589fcd6edb6e08f4c7c32d4f71b54bda029130000000000000000000000000b3352b31449d7e22ea74b7dc2a2c2678f69ab390000000000000000000000000000000000000000000000000000000058fbe175000000000000000000000000000000000000000000000000000000000000a4b10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000066163726f73730000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f6a756d7065722e65786368616e6765000000000000000000000000000000000000000000000000000000000000000000000000000000000000008dd12667bae400000000000000000000000000000000000000000000000000000000668c15630000000000000000000000000000000000000000000000000000000000000080ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000d00dfeeddeadbeef8932eb23bad9bddb5cf81426f78279a53c6c3b71"));

    startBridgeTokensViaAcrossCall::abi_decode(&data, false).expect("Should pass without validate");

    startBridgeTokensViaAcrossCall::abi_decode(&data, true).expect("Should pass with validate");
}
