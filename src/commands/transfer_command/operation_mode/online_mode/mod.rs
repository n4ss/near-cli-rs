pub mod select_server;
use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};

#[derive(Debug, Clone, InteractiveClap)]
pub struct NetworkArgs {
    #[interactive_clap(subcommand)]
    selected_server: self::select_server::SelectServer,
}

impl NetworkArgs {
    pub fn from(
        item: CliNetworkArgs,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        let selected_server = match item.selected_server {
            Some(cli_selected_server) => {
                self::select_server::SelectServer::from(cli_selected_server, context)?
            }
            None => self::select_server::SelectServer::choose_variant(context)?,
        };
        Ok(Self { selected_server })
    }
}

impl NetworkArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.selected_server
            .process(prepopulated_unsigned_transaction)
            .await
    }
}
