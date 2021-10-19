use dialoguer::{theme::ColorfulTheme, Select};
use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

pub mod server;

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
///Select NEAR protocol RPC server
pub enum SelectServer {
    /// Provide data for the server https://rpc.testnet.near.org
    #[strum_discriminants(strum(message = "Testnet"))]
    Testnet(self::server::Server),
    /// Provide data for the server https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Mainnet"))]
    Mainnet(self::server::Server),
    /// Provide data for the server https://rpc.betanet.near.org
    #[strum_discriminants(strum(message = "Betanet"))]
    Betanet(self::server::Server),
    /// Provide data for a manually specified server
    #[strum_discriminants(strum(message = "Custom"))]
    Custom(self::server::CustomServer),
}

impl SelectServer {
    pub fn from(
        item: CliSelectServer,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        match item {
            CliSelectServer::Testnet(cli_server) => Ok(Self::Testnet(
                cli_server.into_server(crate::common::ConnectionConfig::Testnet, context)?,
            )),
            CliSelectServer::Mainnet(cli_server) => Ok(Self::Mainnet(
                cli_server.into_server(crate::common::ConnectionConfig::Mainnet, context)?,
            )),
            CliSelectServer::Betanet(cli_server) => Ok(Self::Betanet(
                cli_server.into_server(crate::common::ConnectionConfig::Betanet, context)?,
            )),
            CliSelectServer::Custom(cli_custom_server) => {
                Ok(Self::Custom(cli_custom_server.into_custom_server(context)?))
            }
        }
    }
}

impl SelectServer {
    // pub fn choose_server(context: crate::common::Context) -> color_eyre::eyre::Result<Self> {
    //     println!();
    //     let variants = SelectServerDiscriminants::iter().collect::<Vec<_>>();
    //     let servers = variants
    //         .iter()
    //         .map(|p| p.get_message().unwrap().to_owned())
    //         .collect::<Vec<_>>();
    //     let selected_server = Select::with_theme(&ColorfulTheme::default())
    //         .with_prompt("Select NEAR protocol RPC server:")
    //         .items(&servers)
    //         .default(0)
    //         .interact()
    //         .unwrap();
    //     let cli_select_server = match variants[selected_server] {
    //         SelectServerDiscriminants::Testnet => CliSelectServer::Testnet(Default::default()),
    //         SelectServerDiscriminants::Mainnet => CliSelectServer::Mainnet(Default::default()),
    //         SelectServerDiscriminants::Betanet => CliSelectServer::Betanet(Default::default()),
    //         SelectServerDiscriminants::Custom => CliSelectServer::Custom(Default::default()),
    //     };
    //     Ok(Self::from(cli_select_server, context)?)
    // }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        Ok(match self {
            SelectServer::Testnet(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
            SelectServer::Mainnet(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
            SelectServer::Betanet(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
            SelectServer::Custom(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
        })
    }
}
