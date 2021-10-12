use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};

#[derive(Debug, Clone, InteractiveClap)]
pub enum SendTo {
    /// Specify a receiver
    Receiver(Receiver),
}

impl SendTo {
    pub fn from(
        item: CliSendTo,
        connection_config: Option<crate::common::ConnectionConfig>,
        sender_account_id: crate::types::account_id::AccountId,
    ) -> color_eyre::eyre::Result<Self> {
        match item {
            CliSendTo::Receiver(cli_receiver) => {
                let receiver = Receiver::from(cli_receiver, connection_config, sender_account_id)?;
                Ok(Self::Receiver(receiver))
            }
        }
    }
}

impl SendTo {
    pub fn send_to(
        connection_config: Option<crate::common::ConnectionConfig>,
        sender_account_id: crate::types::account_id::AccountId,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self::from(
            CliSendTo::Receiver(Default::default()),
            connection_config,
            sender_account_id,
        )?)
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        match self {
            SendTo::Receiver(receiver) => {
                receiver
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
        }
    }
}

#[derive(Debug, Clone, InteractiveClap)]
pub struct Receiver {
    pub receiver_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    pub transfer: super::transfer_near_tokens_type::Transfer,
}

impl ToCli for crate::types::account_id::AccountId {
    type CliVariant = crate::types::account_id::AccountId;
}

impl Receiver {
    fn from(
        item: CliReceiver,
        connection_config: Option<crate::common::ConnectionConfig>,
        sender_account_id: crate::types::account_id::AccountId,
    ) -> color_eyre::eyre::Result<Self> {
        let receiver_account_id: crate::types::account_id::AccountId =
            match item.receiver_account_id {
                Some(cli_receiver_account_id) => match &connection_config {
                    Some(network_connection_config) => match crate::common::check_account_id(
                        network_connection_config.clone(),
                        cli_receiver_account_id.clone().into(),
                    )? {
                        Some(_) => cli_receiver_account_id,
                        None => {
                            if !crate::common::is_64_len_hex(&cli_receiver_account_id) {
                                println!("Account <{}> doesn't exist", cli_receiver_account_id);
                                Receiver::input_receiver_account_id(connection_config.clone())?
                            } else {
                                cli_receiver_account_id
                            }
                        }
                    },
                    None => cli_receiver_account_id,
                },
                None => Receiver::input_receiver_account_id(connection_config.clone())?,
            };
        let transfer: super::transfer_near_tokens_type::Transfer = match item.transfer {
            Some(cli_transfer) => super::transfer_near_tokens_type::Transfer::from(
                cli_transfer,
                connection_config,
                sender_account_id.into(),
            )?,
            None => super::transfer_near_tokens_type::Transfer::choose_transfer_near(
                connection_config,
                sender_account_id.into(),
            )?,
        };
        Ok(Self {
            receiver_account_id,
            transfer,
        })
    }
}

impl Receiver {
    fn input_receiver_account_id(
        connection_config: Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        loop {
            let account_id: crate::types::account_id::AccountId = Input::new()
                .with_prompt("What is the account ID of the receiver?")
                .interact_text()
                .unwrap();
            if let Some(connection_config) = &connection_config {
                if let Some(_) = crate::common::check_account_id(
                    connection_config.clone(),
                    account_id.clone().into(),
                )? {
                    break Ok(account_id);
                } else {
                    if !crate::common::is_64_len_hex(&account_id) {
                        println!("Account <{}> doesn't exist", account_id.to_string());
                    } else {
                        break Ok(account_id);
                    }
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
            receiver_id: self.receiver_account_id.clone().into(),
            ..prepopulated_unsigned_transaction
        };
        self.transfer
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
