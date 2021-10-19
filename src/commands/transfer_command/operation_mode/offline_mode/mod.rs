use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};

#[derive(Debug, Clone, InteractiveClap)]
pub struct OfflineArgs {
    #[interactive_clap(subcommand)]
    send_from: super::online_mode::select_server::server::SendFrom,
}

impl OfflineArgs {
    pub fn from(
        item: CliOfflineArgs,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        let send_from = match item.send_from {
            Some(cli_send_from) => {
                super::online_mode::select_server::server::SendFrom::from(cli_send_from, context)?
            }
            None => super::online_mode::select_server::server::SendFrom::choose_variant(context)?,
        };
        Ok(Self { send_from })
    }
}

impl OfflineArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let selected_server_url = None;
        self.send_from
            .process(prepopulated_unsigned_transaction, selected_server_url)
            .await
    }
}
