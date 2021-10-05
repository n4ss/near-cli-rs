use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;

// /// данные об отправителе транзакции
// #[derive(Debug, Default, Clone, clap::Clap)]
// #[clap(
//     setting(clap::AppSettings::ColoredHelp),
//     setting(clap::AppSettings::DisableHelpSubcommand),
//     setting(clap::AppSettings::VersionlessSubcommands)
// )]
// pub struct CliSender {
//     pub sender_account_id: Option<crate::account_id::AccountId>,
//     #[clap(subcommand)]
//     send_to: Option<super::receiver::CliSendTo>,
// }

#[derive(Debug, Clone, InteractiveClap)]
pub struct Sender {
    pub sender_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    pub send_to: super::receiver::SendTo,
}

// impl ToCli for crate::account_id::AccountId {
//     type CliVariant = crate::account_id::AccountId;
// }

impl CliSender {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        let mut args = self
            .send_to
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default();
        if let Some(sender_account_id) = &self.sender_account_id {
            args.push_front(sender_account_id.to_string());
        }
        args
    }
}

// impl From<Sender> for CliSender {
//     fn from(sender: Sender) -> Self {
//         Self {
//             sender_account_id: Some(sender.sender_account_id),
//             send_to: Some(super::receiver::CliSendTo::from(sender.send_to)),
//         }
//     }
// }

impl Sender {
    pub fn from(
        item: CliSender,
        connection_config: Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<Self> {
        let sender_account_id: crate::types::account_id::AccountId = match item.sender_account_id {
            Some(cli_sender_account_id) => match &connection_config {
                Some(network_connection_config) => match crate::common::check_account_id(
                    network_connection_config.clone(),
                    cli_sender_account_id.clone().into(),
                )? {
                    Some(_) => cli_sender_account_id,
                    None => {
                        println!("Account <{}> doesn't exist", cli_sender_account_id);
                        Sender::input_sender_account_id(connection_config.clone())?
                    }
                },
                None => cli_sender_account_id,
            },
            None => Sender::input_sender_account_id(connection_config.clone())?,
        };
        let send_to: super::receiver::SendTo = match item.send_to {
            Some(cli_send_to) => super::receiver::SendTo::from(
                cli_send_to,
                connection_config,
                sender_account_id.clone(),
            )?,
            None => super::receiver::SendTo::send_to(connection_config, sender_account_id.clone())?,
        };
        Ok(Self {
            sender_account_id,
            send_to,
        })
    }
}

impl Sender {
    fn input_sender_account_id(
        connection_config: Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        loop {
            let account_id: crate::types::account_id::AccountId = Input::new()
                .with_prompt("What is the account ID of the sender?")
                .interact_text()
                .unwrap();
            if let Some(connection_config) = &connection_config {
                if let Some(_) = crate::common::check_account_id(
                    connection_config.clone(),
                    account_id.clone().into(),
                )? {
                    break Ok(account_id);
                } else {
                    println!("Account <{}> doesn't exist", account_id.to_string());
                }
            } else {
                break Ok(account_id);
            }
        }
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.sender_account_id.0.clone(),
            ..prepopulated_unsigned_transaction
        };
        self.send_to
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
