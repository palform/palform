use crate::{TsidFactory, TSID};
use lazy_static::lazy_static;
use rand::{thread_rng, RngCore};
use std::env;
use std::sync::Mutex;

lazy_static! {
    static ref SHARED_FACTORIES: Factories = Factories::new();
}

const DEFAULT_NODE_BITS: u8 = 8;

struct Factories {
    default_shared_factory: Mutex<TsidFactory>,
    shared_factory_8: Mutex<TsidFactory>,
    shared_factory_10: Mutex<TsidFactory>,
    shared_factory_12: Mutex<TsidFactory>,
    node_id: u32,
    node_bits: u8,
}

impl Factories {
    fn new() -> Self {
        let env = read_env();

        Self {
            default_shared_factory: Mutex::new(TsidFactory::with_node_bits(
                env.node_bits,
                env.node_id,
            )),
            shared_factory_8: Mutex::new(TsidFactory::with_node_bits(8, env.node_id)),
            shared_factory_10: Mutex::new(TsidFactory::with_node_bits(10, env.node_id)),
            shared_factory_12: Mutex::new(TsidFactory::with_node_bits(12, env.node_id)),
            node_id: env.node_id,
            node_bits: env.node_bits,
        }
    }
}

struct EnvConfig {
    node_bits: u8,
    node_id: u32,
}

fn read_env() -> EnvConfig {
    let node_bits_env = env::var("TSID_NODE_BITS");
    let node_id_env = env::var("TSID_NODE_ID");

    let node_bits = match node_bits_env {
        Ok(val) => val.parse().unwrap_or(DEFAULT_NODE_BITS),
        Err(_) => 8u8,
    };

    let random_node_val = thread_rng().next_u32();
    let node_val = match node_id_env {
        Ok(val) => val.parse().unwrap_or(random_node_val),
        Err(_) => random_node_val,
    };

    EnvConfig {
        node_bits,
        node_id: node_val,
    }
}

/// Returns a new TSID
///
/// - Node bits taken from ```TSID_NODE_BITS``` environment variable, by default = 8
/// - Node id is taken from env variable ```TSID_NODE_ID``` or random generated (once)
/// - Counter bits: ```22 - node_bits```
/// - Maximum node: ```2^node_bits```
/// - Maximum counter: ```2^(22 - node_bits)```
pub fn create_tsid() -> TSID {
    let mut guard = SHARED_FACTORIES.default_shared_factory.lock().unwrap();
    guard.create()
}

/// Returns a new TSID
///
/// - Node bits: 8
/// - Counter bits: *14*
/// - Maximum node: *256*
/// - Maximum counter *16,384* (maximum TSIDs mer ms mer node)
pub fn create_tsid_256() -> TSID {
    let mut guard = SHARED_FACTORIES.shared_factory_8.lock().unwrap();
    guard.create()
}

/// Returns a new TSID
///
/// - Node bits: 10
/// - Counter bits: *12*
/// - Maximum node: *1024*
/// - Maximum counter *4,096* (maximum TSIDs mer ms mer node)
///
pub fn create_tsid_1024() -> TSID {
    let mut guard = SHARED_FACTORIES.shared_factory_10.lock().unwrap();
    guard.create()
}

/// Returns a new TSID
///
/// - Node bits: 12
/// - Counter bits: *10*
/// - Maximum node: *4096*
/// - Maximum counter *1024* (maximum TSIDs mer ms mer node)
pub fn create_tsid_4096() -> TSID {
    let mut guard = SHARED_FACTORIES.shared_factory_12.lock().unwrap();
    guard.create()
}

/// Returns the NODE_ID parsed from env variable or the random one
/// which was used to create default factory
pub fn get_node_id() -> u32 {
    SHARED_FACTORIES.node_id
}

/// Returns the NODE_BITS parsed from env variable or the default value = 8
/// which was used to create default factory
pub fn get_node_bits() -> u8 {
    SHARED_FACTORIES.node_bits
}
