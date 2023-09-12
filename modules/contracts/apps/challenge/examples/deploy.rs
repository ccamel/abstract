use cw_orch::{
    anyhow,
    prelude::{networks::parse_network, DaemonBuilder},
    tokio::runtime::Runtime,
};

use abstract_challenge_app::{contract::CHALLENGE_APP_ID, ChallengeApp};
use abstract_interface::AppDeployer;
use semver::Version;

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    let chain = parse_network("uni-6");
    use dotenv::dotenv;
    let version: Version = CONTRACT_VERSION.parse().unwrap();
    let rt = Runtime::new()?;
    let chain = DaemonBuilder::default()
        .chain(chain)
        .handle(rt.handle())
        .build()?;
    let app = ChallengeApp::new(CHALLENGE_APP_ID, chain);

    app.deploy(version)?;
    Ok(())
}