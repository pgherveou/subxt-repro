use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let instantiate_call = polkadot::apis().contracts_api().instantiate(
        dev::alice().public_key().into(),
        0,
        None,
        None,
        polkadot::runtime_apis::contracts_api::types::instantiate::Code::Upload(vec![0u8; 10]),
        vec![],
        vec![],
    );

    let instantiate_result = api
        .runtime_api()
        .at_latest()
        .await?
        .call(instantiate_call)
        .await?;

    dbg!(instantiate_result);

    Ok(())
}
