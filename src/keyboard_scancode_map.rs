use winit::event::ScanCode;

// https://github.com/qemu/keycodemapdb/blob/master/data/keymaps.csv

#[cfg(target_os = "linux")]
pub fn translate_scancode(scancode: ScanCode) -> Option<u64> {
    // 0 => undefined
    const TRANSLATION_TABLE: [u64; 194] = [
        0x00070029,
        0x0007001e,
        0x0007001f,
        0x00070020,
        0x00070021,
        0x00070022,
        0x00070023,
        0x00070024,
        0x00070025,
        0x00070026,
        0x00070027,
        0x0007002d,
        0x0007002e,
        0x0007002a,
        0x0007002b,
        0x00070014,
        0x0007001a,
        0x00070008,
        0x00070015,
        0x00070017,
        0x0007001c,
        0x00070018,
        0x0007000c,
        0x00070012,
        0x00070013,
        0x0007002f,
        0x00070030,
        0x00070028,
        0x000700e0,
        0x00070004,
        0x00070016,
        0x00070007,
        0x00070009,
        0x0007000a,
        0x0007000b,
        0x0007000d,
        0x0007000e,
        0x0007000f,
        0x00070033,
        0x00070034,
        0x00070035,
        0x000700e1,
        0x00070031,
        0x0007001d,
        0x0007001b,
        0x00070006,
        0x00070019,
        0x00070005,
        0x00070011,
        0x00070010,
        0x00070036,
        0x00070037,
        0x00070038,
        0x000700e5,
        0x0020000022a,
        0x00200000104,
        0x00000000020,
        0x00100000104,
        0x00100000801,
        0x00100000802,
        0x00100000803,
        0x00100000804,
        0x00100000805,
        0x00100000806,
        0x00100000807,
        0x00100000808,
        0x00100000809,
        0x0010000080a,
        0x00070053,
        0x00070047,
        0x00200000237,
        0x00200000238,
        0x00200000239,
        0x0020000022d,
        0x00200000234,
        0x00200000235,
        0x00200000236,
        0x0020000022b,
        0x00200000231,
        0x00200000232,
        0x00200000233,
        0x00200000230,
        0x0020000022c,
        0x00,
        0x0010000071d,
        0x00,
        0x0010000080b,
        0x0010000080c,
        0x00070087,
        0x0010000071a,
        0x00100000716,
        0x00,
        0x00100000717,
        0x00,
        0x00,
        0x0020000020d,
        0x00200000101,
        0x0020000022f,
        0x00,
        0x000700e6,
        0x00,
        0x00100000306,
        0x00100000304,
        0x00100000308,
        0x00100000302,
        0x00100000303,
        0x00100000305,
        0x00100000301,
        0x00100000307,
        0x00100000407,
        0x0007004c,
        0x00,
        0x0007007f,
        0x00070081,
        0x00070080,
        0x00070066,
        0x00070067,
        0x000700d7,
        0x00100000509,
        0x00,
        0x0020000022c,
        0x00,
        0x00100000712,
        0x00200000022,
        0x00200000106,
        0x00200000107,
        0x00100000704,
        0x000c00b7,
        0x00070079,
        0x000700a3,
        0x0007007a,
        0x00100000d05,
        0x00100000402,
        0x00100000a0b,
        0x00100000408,
        0x00100000507,
        0x00100000404,
        0x00100000508,
        0x00100000505,
        0x00,
        0x00,
        0x00200000002,
        0x00010083,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00100000b09,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00100000b03,
        0x00,
        0x00,
        0x00100000c01,
        0x00100000c03,
        0x00,
        0x00,
        0x00,
        0x000c00b5,
        0x00100000a05,
        0x00100000a09,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00100000c04,
        0x00100000c05,
        0x00100000d15,
        0x00,
        0x00,
        0x00,
        0x00,
        0x000700b6,
        0x000700b7,
        0x000c0201,
        0x000c0279,
        0x00070068,
        0x00070069,
        0x0007006a,
        0x0007006b,
        0x0007006c,
        0x0007006d,
        0x0007006e,
        0x0007006f,
        0x00070070,
        0x00070071,
        0x00070072,
        0x00070073,
    ];

    TRANSLATION_TABLE
        .get((scancode as usize) - 1)
        .filter(|&&hid| hid != 0)
        .copied()
}

// TODO: all other platforms
