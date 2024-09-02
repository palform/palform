use crate::consts::{RANDOM_BITS, RANDOM_MASK, TSID_EPOCH_MILLIS};
use crate::tsid::TSID;
use rand::RngCore;
use std::ops::Add;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct TsidFactory {
    // TODO: Consider if all of those can be generic constants
    node_bits: u8,
    counter_bits: u8,
    counter_mask: u64,
    _node_mask: u32,
    last_time_value: u128,
    counter: u64,
    node: u32,
    pub node_val: u64,
}

impl Default for TsidFactory {
    #[doc = "Create default TsidFactory with `node_bits: 0`"]
    fn default() -> Self {
        TsidFactory::with_node_bits(0, 0)
    }
}

///Example
/// ```rust
/// use tsid::TsidFactory;
/// let factory = TsidFactory::with_node_bits(8,1);
///```
impl TsidFactory {
    /// Create a new TsidFactory with default settings
    /// see [`TsidFactory::default`]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_node_bits(node_bits: u8, node_id: u32) -> Self {
        let counter_bits: u8 = RANDOM_BITS - node_bits;
        let counter_mask = RANDOM_MASK >> node_bits;
        let node_mask = (RANDOM_MASK >> counter_bits) as u32;
        let node = node_id & node_mask;

        let mut rng = rand::thread_rng();
        let counter = rng.next_u64() & counter_mask;
        let last_time_value = Self::get_time_millis_in_tsid_epoch();
        let node_val: u64 = (node << counter_bits) as u64;

        Self {
            node_bits,
            counter_bits,
            counter_mask,
            _node_mask: node_mask,
            last_time_value,
            counter,
            node,
            node_val,
        }
    }

    // naive implementation without thread safety
    pub fn create(&mut self) -> TSID {
        let time = self.get_time_and_advance_counter();
        let time_val: u64 = (time << RANDOM_BITS) as u64;
        let counter_val = self.counter & self.counter_mask;
        let number = time_val | self.node_val | counter_val;
        TSID::new(number)
    }

    pub fn node_bits(&self) -> u8 {
        self.node_bits
    }

    pub fn counter_bits(&self) -> u8 {
        self.counter_bits
    }

    pub fn node(&self) -> u32 {
        self.node
    }

    fn get_time_and_advance_counter(&mut self) -> u128 {
        let mut rng = rand::thread_rng();
        let mut time_millis = Self::get_time_millis_in_tsid_epoch();

        if time_millis <= self.last_time_value {
            self.counter += 1;
            let mut carry = 0;
            if self.counter >> self.counter_bits > 0 {
                carry = 1;
            }
            time_millis = self.last_time_value + carry;
        } else {
            self.counter = rng.next_u64();
        }
        self.counter &= self.counter_mask;
        self.last_time_value = time_millis;

        time_millis
    }

    fn get_time_millis_in_tsid_epoch() -> u128 {
        let tsid_epoch = SystemTime::UNIX_EPOCH.add(Duration::from_millis(TSID_EPOCH_MILLIS));

        SystemTime::now()
            .duration_since(tsid_epoch)
            .expect("UNIX_EPOCH ias after now(), check Your system time")
            .as_millis()
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::TIME_BITS;
    use crate::factory::TsidFactory;
    use crate::TSID;
    use std::collections::HashSet;
    use std::time::{Duration, Instant};

    #[test]
    fn builder_should_set_all_masks_for_8node_bits_version() {
        let factory_under_test = TsidFactory::with_node_bits(8, 0);
        println!("{:?}", factory_under_test);

        assert_eq!(8, factory_under_test.node_bits);
        assert_eq!(14, factory_under_test.counter_bits);
        assert_eq!(0x3fff, factory_under_test.counter_mask);
        assert_eq!(0xff, factory_under_test._node_mask);
        assert_eq!(
            64,
            TIME_BITS + factory_under_test.counter_bits + factory_under_test.node_bits
        )
    }

    #[test]
    fn builder_should_set_all_masks_for_0node_bits_version() {
        let factory_under_test = TsidFactory::with_node_bits(0, 0);
        println!("{:?}", factory_under_test);

        assert_eq!(0, factory_under_test.node_bits);
        assert_eq!(22, factory_under_test.counter_bits);
        assert_eq!(0x3fffff, factory_under_test.counter_mask);
        assert_eq!(0x0, factory_under_test._node_mask);
        assert_eq!(
            64,
            TIME_BITS + factory_under_test.counter_bits + factory_under_test.node_bits
        )
    }

    #[test]
    fn create_tsid() {
        let mut factory_under_test = TsidFactory::with_node_bits(8, 1);
        let _tsid = factory_under_test.create();
    }

    #[test]
    fn check_for_collisions() {
        let mut factory_under_test = TsidFactory::with_node_bits(8, 0);
        let test_duration = Duration::from_millis(100);
        let max_ids_count = 100000;

        let mut uniq_ids: HashSet<TSID> = HashSet::new();

        let start = Instant::now();
        while (start.elapsed() < test_duration) && (uniq_ids.len() < max_ids_count) {
            let id = factory_under_test.create();
            assert!(!uniq_ids.contains(&id), "Set contains duplicates!");
            uniq_ids.insert(id);
        }
    }

    #[test]
    fn id_should_be_incremental() {
        let mut factory_under_test = TsidFactory::with_node_bits(8, 0);
        let test_duration = Duration::from_millis(100);
        let mut max_ids_count = 1000000;
        let mut last = factory_under_test.create();

        let start = Instant::now();
        while (start.elapsed() < test_duration) && (max_ids_count > 0) {
            let id = factory_under_test.create();
            assert!(
                last < id,
                "Id is not incremental! for {}, {}",
                last.number(),
                id.number()
            );
            last = id;
            max_ids_count -= 1;
        }
    }

    #[test]
    fn timer_should_be_incremental() {
        let mut factory_under_test = TsidFactory::with_node_bits(8, 0);
        let mut max_sample_count = 100000;
        let max_duration = Duration::from_millis(100);
        let mut last = factory_under_test.get_time_and_advance_counter();

        let start = Instant::now();
        while max_sample_count > 0 && start.elapsed() < max_duration {
            let next = factory_under_test.get_time_and_advance_counter();
            assert!(
                last <= next,
                "Timer implementation is non monotonic {}, {}",
                last,
                next
            );
            last = next;
            max_sample_count -= 1;
        }
    }
}
