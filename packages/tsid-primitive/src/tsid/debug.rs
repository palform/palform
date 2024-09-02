use crate::TSID;
use std::fmt::{Debug, Formatter};

#[cfg(feature = "debug")]
impl Debug for TSID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("TSID:{}", self).as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::TSID;

    #[test]
    fn should_have_debug_representation() {
        let id1 = TSID::new(496830748901259172);
        assert_eq!("TSID:0DS8RXW6W0DX4", format!("{:?}", id1))
    }
}
