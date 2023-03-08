pub mod mock_modules;

pub const ROOT_USER: &str = "root_user";
pub const TEST_COIN: &str = "ucoin";
use ::abstract_manager::contract::CONTRACT_VERSION;
use abstract_api::mock::{BootMockApi, MockApiContract, MockInitMsg};
use abstract_boot::{Abstract, AnsHost, Manager, ModuleFactory, OSFactory, Proxy, VersionControl};
use abstract_boot::{ApiDeployer, OS};
use abstract_os::{api::InstantiateMsg, objects::gov_type::GovernanceDetails, PROXY};
use abstract_os::{ANS_HOST, MANAGER, MODULE_FACTORY, OS_FACTORY, VERSION_CONTROL};
use boot_core::ContractWrapper;
use boot_core::{
    boot_contract, Contract, Mock, {BootInstantiate, BootUpload, ContractInstance},
};
use cosmwasm_std::{Addr, Empty};
use semver::Version;

pub fn init_abstract_env(chain: Mock) -> anyhow::Result<(Abstract<Mock>, OS<Mock>)> {
    let mut ans_host = AnsHost::new(ANS_HOST, chain.clone());
    let mut os_factory = OSFactory::new(OS_FACTORY, chain.clone());
    let mut version_control = VersionControl::new(VERSION_CONTROL, chain.clone());
    let mut module_factory = ModuleFactory::new(MODULE_FACTORY, chain.clone());
    let mut manager = Manager::new(MANAGER, chain.clone());
    let mut proxy = Proxy::new(PROXY, chain.clone());

    ans_host.as_instance_mut().set_mock(Box::new(
        ContractWrapper::new_with_empty(
            ::ans_host::contract::execute,
            ::ans_host::contract::instantiate,
            ::ans_host::contract::query,
        )
        .with_migrate_empty(::ans_host::contract::migrate),
    ));

    os_factory.as_instance_mut().set_mock(Box::new(
        ContractWrapper::new_with_empty(
            ::os_factory::contract::execute,
            ::os_factory::contract::instantiate,
            ::os_factory::contract::query,
        )
        .with_migrate_empty(::os_factory::contract::migrate)
        .with_reply_empty(::os_factory::contract::reply),
    ));

    module_factory.as_instance_mut().set_mock(Box::new(
        boot_core::ContractWrapper::new_with_empty(
            ::module_factory::contract::execute,
            ::module_factory::contract::instantiate,
            ::module_factory::contract::query,
        )
        .with_migrate_empty(::module_factory::contract::migrate)
        .with_reply_empty(::module_factory::contract::reply),
    ));

    version_control.as_instance_mut().set_mock(Box::new(
        boot_core::ContractWrapper::new_with_empty(
            ::version_control::contract::execute,
            ::version_control::contract::instantiate,
            ::version_control::contract::query,
        )
        .with_migrate_empty(::version_control::contract::migrate),
    ));

    manager.as_instance_mut().set_mock(Box::new(
        boot_core::ContractWrapper::new_with_empty(
            ::abstract_manager::contract::execute,
            ::abstract_manager::contract::instantiate,
            ::abstract_manager::contract::query,
        )
        .with_migrate_empty(::abstract_manager::contract::migrate),
    ));

    proxy.as_instance_mut().set_mock(Box::new(
        boot_core::ContractWrapper::new_with_empty(
            ::proxy::contract::execute,
            ::proxy::contract::instantiate,
            ::proxy::contract::query,
        )
        .with_migrate_empty(::proxy::contract::migrate),
    ));

    // do as above for the rest of the contracts

    let deployment = Abstract {
        chain,
        version: "1.0.0".parse()?,
        ans_host,
        os_factory,
        version_control,
        module_factory,
    };

    let os_core = OS { manager, proxy };

    Ok((deployment, os_core))
}

pub(crate) type AResult = anyhow::Result<()>; // alias for Result<(), anyhow::Error>

pub(crate) fn create_default_os(factory: &OSFactory<Mock>) -> anyhow::Result<OS<Mock>> {
    let os = factory.create_default_os(GovernanceDetails::Monarchy {
        monarch: Addr::unchecked(ROOT_USER).to_string(),
    })?;
    Ok(os)
}
use abstract_api::{ApiContract, ApiError};
use abstract_os::api::{self, BaseInstantiateMsg};
use abstract_sdk::base::InstantiateEndpoint;
use abstract_sdk::AbstractSdkError;
use abstract_testing::addresses::{
    TEST_ADMIN, TEST_ANS_HOST, TEST_MODULE_ID, TEST_VERSION, TEST_VERSION_CONTROL,
};
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    DepsMut, Env, MessageInfo, Response, StdError,
};
use thiserror::Error;

pub const TEST_METADATA: &str = "test_metadata";
pub const TEST_TRADER: &str = "test_trader";

pub(crate) fn init_mock_api(
    chain: Mock,
    _deployment: &Abstract<Mock>,
    version: Option<String>,
) -> anyhow::Result<BootMockApi<Mock>> {
    let mut staking_api = BootMockApi::new(TEST_MODULE_ID, chain);
    let version: Version = version
        .unwrap_or_else(|| CONTRACT_VERSION.to_string())
        .parse()?;
    staking_api.deploy(version, MockInitMsg)?;
    Ok(staking_api)
}
