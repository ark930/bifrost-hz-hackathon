// Copyright 2019 Liebi Technologies.
// This file is part of Bifrost.

// Bifrost is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Bifrost is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Bifrost.  If not, see <http://www.gnu.org/licenses/>.

//! A collection of node-specific RPC methods.
//!
//! Since `substrate` core functionality makes no assumptions
//! about the modules used inside the runtime, so do
//! RPC methods defined in `substrate-rpc` crate.
//! It means that `core/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what `SRML` modules
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here.

#![warn(missing_docs)]

use std::sync::Arc;

use node_primitives::{Block, AccountNonceApi, ContractsApi};
use sr_primitives::traits::ProvideRuntimeApi;
use transaction_pool::txpool::{ChainApi, Pool};

pub mod accounts;
pub mod contracts;

mod constants {
	/// A status code indicating an error happened while trying to call into the runtime.
	///
	/// This typically means that the runtime trapped.
	pub const RUNTIME_ERROR: i64 = 1;
}

/// Instantiate all RPC extensions.
pub fn create<C, P, M>(client: Arc<C>, pool: Arc<Pool<P>>) -> jsonrpc_core::IoHandler<M> where
	C: ProvideRuntimeApi,
	C: client::blockchain::HeaderBackend<Block>,
	C: Send + Sync + 'static,
	C::Api: AccountNonceApi<Block> + ContractsApi<Block>,
	P: ChainApi + Sync + Send + 'static,
	M: jsonrpc_core::Metadata + Default,
{
	use self::{
		accounts::{Accounts, AccountsApi},
		contracts::{Contracts, ContractsApi},
	};

	let mut io = jsonrpc_core::IoHandler::default();
	io.extend_with(
		AccountsApi::to_delegate(Accounts::new(client.clone(), pool))
	);
	io.extend_with(
		ContractsApi::to_delegate(Contracts::new(client))
	);
	io
}
