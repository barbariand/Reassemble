#[repr(C)]
pub struct HeaderNDS {
   pub title: [u8; 12],
   pub tid: u32,
   pub developer: u16,
   pub unit: u8,
   pub encryption_seed: u8,
   pub device_capacity: u8,
   pub _reserved: [u8; 7],
   pub revision: u16,
   pub rom_version: u8,
   pub flags: u8,

   pub arm9_offset: u32,
   pub arm9_entry: u32,
   pub arm9_load: u32,
   pub arm9_size: u32,

   pub arm7_offset: u32,
   pub arm7_entry: u32,
   pub arm7_load: u32,
   pub arm7_size: u32,

   pub fnt_offset: u32,
   pub fnt_len: u32,

   pub fat_offset: u32,
   pub fat_len: u32,

   pub arm9_overlay_offset: u32,
   pub arm9_overlay_len: u32,

   pub arm7_overlay_offset: u32,
   pub arm7_overlay_len: u32,

   pub card_cnt: u32,
   pub card_cnt_secure: u32,
   pub icon_offset: u32,
   pub secure_area_crc: u16,
   pub secure_area_timeout: u16,

   pub arm9_autoload: u32,
   pub arm7_autoload: u32,

   pub secure_disable: [u8; 8],

   pub ntr_rom_size: u32,
   pub header_size: u32,

   pub unknown: u32,
   pub _reserved2: [u32; 13],

   pub logo: [u8; 156],
   pub logo_crc: u16,

   pub header_crc: u16,

   pub debugger: [u8; 32],
   pub global_mbks: [u32; 5],
   pub arm9_mbks: [u32; 3],
   pub arm7_mbks: [u32; 3],
   pub mbk9: u32,

   pub region: u32,
   pub access_control: u32,
   pub arm7_scfg: u32,
   pub dsi_flags: u32,

   pub arm9i_offset: u32,
   pub _reservedi: u32,
   pub arm9i_load: u32,
   pub arm9i_size: u32,

   pub arm7i_offset: u32,
   pub _reservedi2: u32,
   pub arm7i_load: u32,
   pub arm7i_size: u32,

   pub digest_ntr_offset: u32,
   pub digest_ntr_len: u32,
   pub digest_twl_offset: u32,
   pub digest_twl_len: u32,
   pub sector_hashtable_offset: u32,
   pub sector_hashtable_len: u32,
   pub block_hashtable_offset: u32,
   pub block_hashtable_len: u32,
   pub sector_size: u32,
   pub block_sectorcount: u32,
   pub icon_banner_size: u32,
   pub unknown2: u32,
   pub total_rom_size: u32,
   pub unknown3: [u32; 3],
   pub modcrypt1_offset: u32,
   pub modcrypt1_len: u32,
   pub modcrypt2_offset: u32,
   pub modcrypt2_len: u32,
   pub title_id: [u8; 8],
   pub public_save_size: u32,
   pub private_save_size: u32,
   pub _reserved3: [u8; 176],
   pub unknown4: [u32; 4],

   pub arm9_sha1: [u32; 5],
   pub arm7_sha1: [u32; 5],
   pub digest_sha1: [u32; 5],
   pub banner_sha1: [u32; 5],
   pub arm9i_sha1: [u32; 5],
   pub arm7i_sha1: [u32; 5],
   pub _reserved4: [u8; 40],
   pub arm9_sha1_unsecure: [u32; 5],
   pub _reserved5: [u8; 2636],
   pub debug: [u8; 0x180],
   pub rsa_signature: [u8; 0x80],
}
