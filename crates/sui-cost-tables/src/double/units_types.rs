// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::gas_algebra::{GasQuantity, InternalGasUnit, ToUnit, ToUnitFractional};

use crate::bytecode_based::units_types as BU;
use crate::tier_based::units_types as TU;
use serde::{Deserialize, Serialize};

pub enum GasUnit {}

pub type Gas = GasQuantity<GasUnit>;

impl ToUnit<InternalGasUnit> for GasUnit {
    const MULTIPLIER: u64 = 1000;
}

impl ToUnitFractional<GasUnit> for InternalGasUnit {
    const NOMINATOR: u64 = 1;
    const DENOMINATOR: u64 = 1000;
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
pub struct CostTable {
    pub bytecode: BU::CostTable,
    pub tiers: TU::CostTable,
}
