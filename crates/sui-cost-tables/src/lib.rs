// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

pub mod natives_tables;

pub mod bytecode_based;
pub mod tier_based;

pub mod double;
pub use double::tables as bytecode_tables;
pub use double::units_types;
