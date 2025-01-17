// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Traits used by the Substrate/Ethereum chains bridging pallet.


// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

// Frame, system and frame primitives
use frame_support::weights::Weight;


// ----------------------------------------------------------------------------
// Traits declaration
// ----------------------------------------------------------------------------

/// Weight information for pallet extrinsics
///
/// Weights are calculated using runtime benchmarking features.
/// See [`benchmarking`] module for more information. 
pub trait WeightInfo {

    fn set_threshold() -> Weight;

    fn set_resource() -> Weight;
    
    fn remove_resource() -> Weight;

    fn whitelist_chain() -> Weight;

    fn add_relayer() -> Weight;

    fn remove_relayer() -> Weight;

    fn acknowledge_proposal(dispatch_weight: Weight) -> Weight;

    fn reject_proposal() -> Weight;

    fn eval_vote_state(dispatch_weight: Weight) -> Weight;
}
