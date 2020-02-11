pub const DATABASE_PATH: &str = "calco-database.db";

pub static INVITATION_EXPIRE_DURATION_SECONDS: u64 = 3600;

pub static USER_TOKEN_EXPIRE_DURATION_SECONDS: u64 = 3600;

pub const ADMIN_HANDLE: &str = "thottou";

pub const SALT_COMPONENT: [u8; 16] = [
  0x3f, 0x12, 0x9b, 0x5c,
  0x22, 0xb0, 0x12, 0x6d,
  0xe5, 0x6a, 0x09, 0x7a,
  0x97, 0xaa, 0xf9, 0x99
];