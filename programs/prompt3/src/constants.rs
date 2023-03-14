pub const ADMIN: &str = "8BcT1AVYrqSaPfXtvTKFFB3TZxSq1ccBCoBsUtbUsKGV";

pub const MOD_1: &str = "65TzNjiuotUBY94xgWZkp47AX2gPgrEmYyykMT1VPKBX";
pub const MOD_2: &str = "8Y2u5JkUNvqv22PGKCnFjrh9J8suevKYZ3ePYF4T7v38";

pub const SELLER_SEED: &[u8] = b"seller";
pub const STATE_SEED: &[u8] = b"state";
pub const LISTING_SEED: &[u8] = b"listing";
pub const RECEIPT_SEED: &[u8] = b"receipt";
pub const BUYER_SEED: &[u8] = b"buyer";

pub const OFFSET_SIZE: usize = 8;
pub const STATE_PDA_SIZE: usize = OFFSET_SIZE + 8 + 8 + 8 + 8 + 1 + 8;
pub const RECEIPT_PDA_SIZE: usize = OFFSET_SIZE + 32 + 1;
pub const LISTING_PDA_SIZE: usize = OFFSET_SIZE + 100 + 8 + 8 + 32 + 32 + 8 + 8 + 1;
pub const BUYER_PDA_SIZE: usize = OFFSET_SIZE + 32 + 8 + 1;
pub const SELLER_PDA_SIZE: usize = OFFSET_SIZE + 32 + 8 + 8 + 8 + 1;
