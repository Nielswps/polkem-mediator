use cumulus_primitives_core::ParaId;
use polkem_mediator_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
sc_service::GenericChainSpec<polkem_mediator_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn mediator_session_keys(keys: AuraId) -> polkem_mediator_runtime::SessionKeys {
    polkem_mediator_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                // initial collators.
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("0x1e507f13e9e11444df9198d7f710ed73938c556909dfa4f610078d2ec3a22c18"),
                        get_collator_keys_from_seed("0x1e507f13e9e11444df9198d7f710ed73938c556909dfa4f610078d2ec3a22c18"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("0x200c2565779f60d10529b14c27ab5c86798c4a26f3c8b4101fa42c7bfc7f9517"),
                        get_collator_keys_from_seed("0x200c2565779f60d10529b14c27ab5c86798c4a26f3c8b4101fa42c7bfc7f9517"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("0x0897414e47d98d8bb453bcac44d79feebb907d40cd7829aa76e2e649c0c6ab0b"),
                        get_collator_keys_from_seed("0x0897414e47d98d8bb453bcac44d79feebb907d40cd7829aa76e2e649c0c6ab0b"),
                    ),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("0x1e507f13e9e11444df9198d7f710ed73938c556909dfa4f610078d2ec3a22c18"),
                    get_account_id_from_seed::<sr25519::Public>("0x200c2565779f60d10529b14c27ab5c86798c4a26f3c8b4101fa42c7bfc7f9517"),
                    get_account_id_from_seed::<sr25519::Public>("0x0897414e47d98d8bb453bcac44d79feebb907d40cd7829aa76e2e649c0c6ab0b"),
                    get_account_id_from_seed::<sr25519::Public>("0x46b31dace2abd1b3da960ce0fef9c868a0d816d4b3277cf530cc4bd21a79ca65"),
                    get_account_id_from_seed::<sr25519::Public>("0xd004ca1704d339cf80b50bc48a55a1a8d407e733295e9142e47d3e59a69b061f"),
                    get_account_id_from_seed::<sr25519::Public>("0x6173d39ae56f44f06ce389c6b65a09fef229214a9591fb3853eb573bb1b21730"),
                ],
                100.into(),
            )
        },
        Vec::new(),
        None,
        Some("mediator-local"),
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: 100,
        },
    )
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "UNIT".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                // initial collators.
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("0x1e507f13e9e11444df9198d7f710ed73938c556909dfa4f610078d2ec3a22c18"),
                        get_collator_keys_from_seed("0x1e507f13e9e11444df9198d7f710ed73938c556909dfa4f610078d2ec3a22c18"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("0x200c2565779f60d10529b14c27ab5c86798c4a26f3c8b4101fa42c7bfc7f9517"),
                        get_collator_keys_from_seed("0x200c2565779f60d10529b14c27ab5c86798c4a26f3c8b4101fa42c7bfc7f9517"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("0x0897414e47d98d8bb453bcac44d79feebb907d40cd7829aa76e2e649c0c6ab0b"),
                        get_collator_keys_from_seed("0x0897414e47d98d8bb453bcac44d79feebb907d40cd7829aa76e2e649c0c6ab0b"),
                    ),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("0x1e507f13e9e11444df9198d7f710ed73938c556909dfa4f610078d2ec3a22c18"),
                    get_account_id_from_seed::<sr25519::Public>("0x200c2565779f60d10529b14c27ab5c86798c4a26f3c8b4101fa42c7bfc7f9517"),
                    get_account_id_from_seed::<sr25519::Public>("0x0897414e47d98d8bb453bcac44d79feebb907d40cd7829aa76e2e649c0c6ab0b"),
                    get_account_id_from_seed::<sr25519::Public>("0x46b31dace2abd1b3da960ce0fef9c868a0d816d4b3277cf530cc4bd21a79ca65"),
                    get_account_id_from_seed::<sr25519::Public>("0xd004ca1704d339cf80b50bc48a55a1a8d407e733295e9142e47d3e59a69b061f"),
                    get_account_id_from_seed::<sr25519::Public>("0x6173d39ae56f44f06ce389c6b65a09fef229214a9591fb3853eb573bb1b21730"),
                ],
                100.into(),
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        Some("mediator-local"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: 100,
        },
    )
}

fn testnet_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
) -> polkem_mediator_runtime::GenesisConfig {
    polkem_mediator_runtime::GenesisConfig {
        system: polkem_mediator_runtime::SystemConfig {
            code: polkem_mediator_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: polkem_mediator_runtime::BalancesConfig {
            balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
        },
        parachain_info: polkem_mediator_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: polkem_mediator_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
            ..Default::default()
        },
        session: polkem_mediator_runtime::SessionConfig {
            keys: invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        mediator_session_keys(aura), // session keys
                    )
                })
                .collect(),
        },
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        aura_ext: Default::default(),
        parachain_system: Default::default(),
        polkadot_xcm: polkem_mediator_runtime::PolkadotXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION),
        },
    }
}
