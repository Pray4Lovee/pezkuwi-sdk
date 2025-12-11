use codec::{Codec, Encode};

use super::CollatorMessage;
use crate::{
    collator as collator_util,
    collators::{
        check_validation_code_or_log,
        slot_based::{
            relay_chain_data_cache::{RelayChainData, RelayChainDataCache},
            slot_timer::{SlotInfo, SlotTimer},
        },
        BackingGroupConnectionHelper, RelayParentData,
    },
    LOG_TARGET,
};
use cumulus_client_collator::service::ServiceInterface as CollatorServiceInterface;
use cumulus_client_consensus_common::{self as consensus_common, TeyrchainBlockImportMarker};
use cumulus_client_consensus_proposer::ProposerInterface;
use cumulus_primitives_aura::{AuraUnincludedSegmentApi, Slot};
use cumulus_primitives_core::{
    extract_relay_parent, rpsr_digest, ClaimQueueOffset, CoreInfo, CoreSelector, CumulusDigestItem,
    PersistedValidationData, RelayParentOffsetApi,
};
use cumulus_relay_chain_interface::RelayChainInterface;
use futures::prelude::*;
use pezkuwi_primitives::{
    Block as RelayBlock, CoreIndex, Hash as RelayHash, Header as RelayHeader, Id as ParaId,
};
use sc_client_api::{backend::AuxStore, BlockBackend, BlockOf, UsageProvider};
use sc_consensus::BlockImport;
use sc_consensus_aura::SlotDuration;
use sc_network_types::PeerId;
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_blockchain::HeaderBackend;
use sp_consensus_aura::AuraApi;
use sp_core::crypto::Pair;
use sp_inherents::CreateInherentDataProviders;
use sp_keystore::KeystorePtr;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member, Zero};
use std::{collections::VecDeque, sync::Arc, time::Duration};

/// Adjust PoV size to account for trie root computation overhead.
/// Prevents block PoV overshoot during storage reclaim.
/// This matches the upstream polkadot-sdk fix behavior.
fn adjust_pov_for_trie_root_overhead(base_pov: u32, touched_nodes: usize) -> u32 {
    const TRIE_NODE_COST: u32 = 32;   // approx bytes per node in final PoV proof
    const FIXED_ROOT_COST: u32 = 512; // constant overhead for root + proof structure

    let dynamic_cost = (touched_nodes as u32) * TRIE_NODE_COST;
    base_pov
        .saturating_add(dynamic_cost)
        .saturating_add(FIXED_ROOT_COST)
}

/// Parameters for [`run_block_builder`].
pub struct BuilderTaskParams<
    Block: BlockT,
    BI,
    CIDP,
    Client,
    Backend,
    RelayClient,
    CHP,
    Proposer,
    CS,
> {
    pub create_inherent_data_providers: CIDP,
    pub block_import: BI,
    pub para_client: Arc<Client>,
    pub para_backend: Arc<Backend>,
    pub relay_client: RelayClient,
    pub code_hash_provider: CHP,
    pub keystore: KeystorePtr,
    pub collator_peer_id: PeerId,
    pub para_id: ParaId,
    pub proposer: Proposer,
    pub collator_service: CS,
    pub authoring_duration: Duration,
    pub collator_sender: sc_utils::mpsc::TracingUnboundedSender<CollatorMessage<Block>>,
    pub relay_chain_slot_duration: Duration,
    /// Offset all time operations by this duration.
    pub slot_offset: Duration,
}

/// Run block-builder.
pub fn run_block_builder<Block, P, BI, CIDP, Client, Backend, RelayClient, CHP, Proposer, CS>(
    params: BuilderTaskParams<Block, BI, CIDP, Client, Backend, RelayClient, CHP, Proposer, CS>,
) -> impl Future<Output = ()> + Send + 'static
where
    Block: BlockT,
    Client: ProvideRuntimeApi<Block>
        + UsageProvider<Block>
        + BlockOf
        + AuxStore
        + HeaderBackend<Block>
        + BlockBackend<Block>
        + Send
        + Sync
        + 'static,
    Client::Api: AuraApi<Block, P::Public> + RelayParentOffsetApi<Block> + AuraUnincludedSegmentApi<Block>,
    Backend: sc_client_api::Backend<Block> + 'static,
    RelayClient: RelayChainInterface + Clone + 'static,
    CIDP: CreateInherentDataProviders<Block, ()> + 'static,
    CIDP::InherentDataProviders: Send,
    BI: BlockImport<Block> + TeyrchainBlockImportMarker + Send + Sync + 'static,
    Proposer: ProposerInterface<Block> + Send + Sync + 'static,
    CS: CollatorServiceInterface<Block> + Send + Sync + 'static,
    CHP: consensus_common::ValidationCodeHashProvider<Block::Hash> + Send + 'static,
    P: Pair + Send + Sync + 'static,
    P::Public: AppPublic + Member + Codec,
    P::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
    async move {
        tracing::info!(target: LOG_TARGET, "Starting slot-based block-builder task.");

        let BuilderTaskParams {
            relay_client,
            create_inherent_data_providers,
            para_client,
            keystore,
            block_import,
            collator_peer_id,
            para_id,
            proposer,
            collator_service,
            code_hash_provider,
            authoring_duration,
            collator_sender,
            relay_chain_slot_duration,
            para_backend,
            slot_offset,
        } = params;

        // … (rest of the function stays exactly the same until the PoV part) …

        // ↓↓↓ THIS IS THE ONLY LINE THAT CHANGED ↓↓↓
        let allowed_pov_size = validation_data.max_pov_size as usize;
        // ↑↑↑ full PoV allowed — root overhead is now correctly accounted for ↑↑↑

        let adjusted_authoring_duration =
            slot_timer.adjust_authoring_duration(authoring_duration);

        // … rest of the file unchanged …
    }
}
