// This file has been auto-generated by scripts/generate_versions.ts
// DO NOT EDIT THIS FILE
// To add versions, add a version to the resources/versions.json file!

/// The current minecraft patch version (1.10.2.x)
pub const CURRENT_PATCH: u32 = 563;
/// The current minecraft minor version (1.10.x)
pub const CURRENT_MINOR: u32 = 560;
/// The current minecraft major version (1.x.0)
pub const CURRENT_MAJOR: u32 = 526;

/// This is a helper enum to get a version number from a version string.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum Versions {
    V1_19_60_24 = 563,
    V1_19_60_23 = 562,
    V1_19_60_20 = 561,
    V1_19_51 = 560,
    V1_19_50 = 559,
    V1_19_50_20 = 558,
    V1_19_41 = 557,
    V1_19_40_22 = 555,
    V1_19_40_20 = 554,
    V1_19_30_25 = 553,
    V1_19_30_22 = 552,
    V1_19_30_20 = 551,
    V1_19_22 = 545,
    V1_19_20 = 544,
    V1_19_20_23 = 543,
    V1_19_20_22 = 542,
    V1_19_11 = 534,
    V1_19_10_22 = 533,
    V1_19_10_21 = 532,
    V1_19_10_20 = 530,
    V1_19_2 = 527,
    V1_19_0 = 526,
    V1_19_0_34 = 524,
    V1_19_0_26 = 516,
    V1_19_0_24 = 514,
    V1_19_0_20 = 512,
    V1_18_20_24 = 491,
    V1_18_20_23 = 490,
    V1_18_10_28 = 486,
    V1_18_10_27 = 485,
    V1_18_10_24 = 479,
    V1_18_10_22 = 477,
    V1_18_10_21 = 476,
    V1_18_0 = 475,
    V1_18_0_21 = 474,
    V1_17_41 = 471,
    V1_17_34 = 465,
    V1_17_30_24 = 464,
    V1_17_30_21 = 462,
    V1_17_20_23 = 459,
    V1_17_20_22 = 456,
    V1_17_20_21 = 455,
    V1_17_20_20 = 453,
    V1_17_11 = 448,
    V1_17_10_21 = 441,
    V1_17_0 = 440,
    V1_17_0_52 = 437,
    V1_16_230_56 = 435,
    V1_16_230_52 = 434,
    V1_16_230_50 = 433,
    V1_16_220 = 431,
    V1_16_220_51 = 430,
    V1_16_220_50 = 429,
    V1_16_210 = 428,
    V1_16_210_57 = 427,
    V1_16_210_55 = 425,
    V1_16_210_53 = 424,
    V1_16_210_51 = 423,
    V1_16_201 = 422,
    V1_16_200_52 = 421,
    V1_16_200_51 = 420,
    V1_16_101 = 419,
    V1_16_100_59 = 418,
    V1_16_100_58 = 417,
    V1_16_100_57 = 416,
    V1_16_100_56 = 415,
    V1_16_100_55 = 414,
    V1_16_100_54 = 413,
    V1_16_100_53 = 412,
    V1_16_100_52 = 411,
    V1_16_100_51 = 410,
    V1_16_100_50 = 409,
    V1_16_61 = 408,
    V1_16_0 = 407,
}

pub fn version_within_current_patch(version: u32) -> bool {
    version >= CURRENT_MAJOR && version <= CURRENT_PATCH
}

pub fn version_within_current_minor(version: u32) -> bool {
    version >= CURRENT_MAJOR && version <= CURRENT_MINOR
}