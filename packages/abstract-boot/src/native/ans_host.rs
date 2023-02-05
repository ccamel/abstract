use abstract_os::ans_host::*;
use abstract_os::objects::pool_id::UncheckedPoolAddress;
use abstract_os::objects::{PoolMetadata, UncheckedChannelEntry, UncheckedContractEntry};
use abstract_os::ANS_HOST;
use boot_core::prelude::ContractInstance;
use boot_core::{
    prelude::boot_contract, BootEnvironment, BootError, Contract, IndexResponse, TxResponse,
};
use cosmwasm_std::Addr;
use cw_asset::AssetInfoUnchecked;
use log::info;
use serde_json::from_reader;
use std::collections::HashSet;
use std::{cmp::min, env, fs::File};

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct AnsHost<Chain>;

impl<Chain: BootEnvironment> AnsHost<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: Chain) -> Self {
        let mut contract = Contract::new(name, chain);
        contract = contract.with_wasm_path("ans_host");
        Self(contract)
    }

    pub fn load(chain: Chain, address: &Addr) -> Self {
        Self(Contract::new(ANS_HOST, chain).with_address(Some(address)))
    }
}

/// Implementation for the daemon, which maintains actual state
#[cfg(feature = "daemon")]
use boot_core::Daemon;
#[cfg(feature = "daemon")]
impl AnsHost<Daemon> {
    pub fn update_all(&self) -> Result<(), BootError> {
        self.update_assets()?;
        self.update_contracts()?;
        self.update_pools()?;
        Ok(())
    }

    pub fn update_assets(&self) -> Result<(), BootError> {
        let path = env::var("ANS_HOST_ASSETS").unwrap();
        let file =
            File::open(&path).unwrap_or_else(|_| panic!("file should be present at {}", &path));
        let json: serde_json::Value = from_reader(file)?;
        let chain_id = self.get_chain().state.chain.chain_id.clone();
        info!("{}", chain_id);
        let network_id = self.get_chain().state.id.clone();
        let maybe_assets = json
            .get(chain_id)
            .unwrap()
            .get(network_id)
            .ok_or_else(|| BootError::StdErr("network not found".into()))?;

        /*

        [[
           "junox",
           {
             "native": "ujunox"
           }
         ],
         [
           "crab",
           {
             "cw20": "juno12wwfatmpqnxdqk0ka8rpd4ud4frc97srtv55mmlf7xunux6led8s8wtgx2"
           }
         ]]
            */

        let assets = maybe_assets.as_array().unwrap();

        let assets_to_add: Vec<(String, AssetInfoUnchecked)> = assets
            .iter()
            .map(|value| {
                let asset: (String, AssetInfoUnchecked) =
                    serde_json::from_value(value.clone()).unwrap();
                asset
            })
            .collect();

        self.execute_chunked(&assets_to_add, 25, |chunk| {
            ExecuteMsg::UpdateAssetAddresses {
                to_add: chunk.to_vec(),
                to_remove: vec![],
            }
        })?;

        Ok(())
    }

    pub fn update_channels(&self) -> Result<(), BootError> {
        let path = env::var("ANS_HOST_CHANNELS").unwrap();
        let file =
            File::open(&path).unwrap_or_else(|_| panic!("file should be present at {}", &path));
        let json: serde_json::Value = from_reader(file)?;
        let chain_id = self.get_chain().state.chain.chain_id.clone();
        let network_id = self.get_chain().state.id.clone();
        let channels = json
            .get(chain_id)
            .unwrap()
            .get(network_id)
            .ok_or_else(|| BootError::StdErr("network not found".into()))?;

        let channels = channels.as_object().unwrap();
        let channels_to_add: Vec<(UncheckedChannelEntry, String)> = channels
            .iter()
            .map(|(name, value)| {
                let id = value.as_str().unwrap().to_owned();
                let key = UncheckedChannelEntry::try_from(name.clone()).unwrap();
                (key, id)
            })
            .collect();

        self.execute_chunked(&channels_to_add, 25, |chunk| ExecuteMsg::UpdateChannels {
            to_add: chunk.to_vec(),
            to_remove: vec![],
        })?;

        Ok(())
    }

    pub fn update_contracts(&self) -> Result<(), BootError> {
        let path = env::var("ANS_HOST_CONTRACTS").unwrap();

        let file =
            File::open(&path).unwrap_or_else(|_| panic!("file should be present at {}", &path));
        let json: serde_json::Value = from_reader(file)?;
        let chain_id = self.get_chain().state.chain.chain_id.clone();
        let network_id = self.0.get_chain().state.id.clone();
        let contracts = json
            .get(chain_id)
            .unwrap()
            .get(network_id)
            .ok_or_else(|| BootError::StdErr("network not found".into()))?;

        /*
          [
        [
          {
            "protocol": "junoswap",
            "contract": "staking/crab,junox"
          },
          "juno1vhxnvu0zh6p707auf0ltq6scse3d2fxvp4804s54q45z29vtjleqghne5g"
        ]
        ]
           */

        let contracts = contracts.as_array().unwrap();
        let contracts_to_add: Vec<(UncheckedContractEntry, String)> = contracts
            .iter()
            .map(|value| {
                let contract: (UncheckedContractEntry, String) =
                    serde_json::from_value(value.clone()).unwrap();
                contract
            })
            .collect();

        self.execute_chunked(&contracts_to_add, 25, |chunk| {
            ExecuteMsg::UpdateContractAddresses {
                to_add: chunk.to_vec(),
                to_remove: vec![],
            }
        })?;

        Ok(())
    }

    pub fn update_pools(&self) -> Result<(), BootError> {
        let path = env::var("ANS_HOST_POOLS").unwrap();
        let file =
            File::open(&path).unwrap_or_else(|_| panic!("file should be present at {}", &path));
        let json: serde_json::Value = from_reader(file)?;
        let chain_id = self.get_chain().state.chain.chain_id.clone();
        let network_id = self.0.get_chain().state.id.clone();
        let pools = json
            .get(chain_id)
            .unwrap()
            .get(network_id)
            .ok_or_else(|| BootError::StdErr("network not found".into()))?;

        let mut dexes_to_register: HashSet<String> = HashSet::new();

        let pools = pools.as_array().unwrap();
        let pools_to_add: Vec<(UncheckedPoolAddress, PoolMetadata)> = pools
            .iter()
            .map(|value| {
                let pool: (UncheckedPoolAddress, PoolMetadata) =
                    serde_json::from_value(value.clone()).unwrap();

                dexes_to_register.insert(pool.1.dex.clone());

                pool
            })
            .collect();

        // Register the dexes
        self.0.execute(
            &ExecuteMsg::UpdateDexes {
                to_add: Vec::from_iter(dexes_to_register),
                to_remove: vec![],
            },
            None,
        )?;

        self.execute_chunked(&pools_to_add, 25, |chunk| ExecuteMsg::UpdatePools {
            to_add: chunk.to_vec(),
            to_remove: vec![],
        })?;

        Ok(())
    }

    fn execute_chunked<T, F>(
        &self,
        items: &[T],
        chunk_size: usize,
        mut msg_builder: F,
    ) -> Result<(), BootError>
    where
        F: FnMut(&[T]) -> ExecuteMsg,
    {
        let mut i = 0;
        while i < items.len() {
            let chunk = &items[i..min(i + chunk_size, items.len())];
            i += chunk.len();
            self.0.execute(&msg_builder(chunk), None)?;
        }
        Ok(())
    }
}
