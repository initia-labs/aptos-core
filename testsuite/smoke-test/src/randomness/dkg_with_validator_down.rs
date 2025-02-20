// Copyright © Aptos Foundation

use crate::{
    randomness::{decrypt_key_map, verify_dkg_transcript, wait_for_dkg_finish},
    smoke_test_environment::SwarmBuilder,
};
use aptos_forge::NodeExt;
use aptos_types::on_chain_config::{FeatureFlag, Features};
use std::sync::Arc;

#[tokio::test]
async fn dkg_with_validator_down() {
    let epoch_duration_secs = 10;
    let estimated_dkg_latency_secs = 20;
    let time_limit_secs = epoch_duration_secs + estimated_dkg_latency_secs;

    let mut swarm = SwarmBuilder::new_local(4)
        .with_num_fullnodes(1)
        .with_aptos()
        .with_init_genesis_config(Arc::new(|conf| {
            conf.epoch_duration_secs = 10;

            // Ensure vtxn is enabled.
            conf.consensus_config.enable_validator_txns();

            // Ensure randomness flag is set.
            let mut features = Features::default();
            features.enable(FeatureFlag::RECONFIGURE_WITH_DKG);
            conf.initial_features_override = Some(features);
        }))
        .build()
        .await;
    let decrypt_key_map = decrypt_key_map(&swarm);

    let client = swarm.validators().last().unwrap().rest_client();
    println!("Wait for an epoch start.");
    let dkg_session_1 = wait_for_dkg_finish(&client, None, time_limit_secs).await;

    println!("Current epoch is {}.", dkg_session_1.target_epoch());

    println!("Take one validator down.");
    swarm.validators_mut().take(1).for_each(|v| {
        v.stop();
    });

    println!(
        "Wait until we fully entered epoch {}.",
        dkg_session_1.target_epoch() + 1
    );

    let dkg_session_2 = wait_for_dkg_finish(
        &client,
        Some(dkg_session_1.target_epoch() + 1),
        time_limit_secs,
    )
    .await;

    assert!(verify_dkg_transcript(&dkg_session_2, &decrypt_key_map).is_ok());
}
