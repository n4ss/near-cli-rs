use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(context = crate::common::Context)]
#[interactive_clap(disable_strum_discriminants)]
pub enum Transfer {
    /// Enter an amount to transfer
    Amount(TransferNEARTokensAction),
}

impl Transfer {
    pub fn from(
        item: CliTransfer,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        match item {
            CliTransfer::Amount(cli_transfer_near_action) => Ok(Self::Amount(
                TransferNEARTokensAction::from(cli_transfer_near_action, context)?,
            )),
        }
    }
}

impl Transfer {
    // pub fn choose_transfer_near(context: crate::common::Context) -> color_eyre::eyre::Result<Self> {
    //     Self::from(CliTransfer::Amount(Default::default()), context)
    // }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        match self {
            Transfer::Amount(transfer_near_action) => {
                transfer_near_action
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
        }
    }
}

#[derive(Debug, Clone, InteractiveClap)]
pub struct TransferNEARTokensAction {
    pub amount: crate::common::NearBalance,
    #[interactive_clap(subcommand)]
    pub sign_option:
        crate::commands::construct_transaction_command::sign_transaction::SignTransaction,
}

impl ToCli for crate::common::NearBalance {
    type CliVariant = crate::common::NearBalance;
}

impl TransferNEARTokensAction {
    fn from(
        item: <TransferNEARTokensAction as ToCli>::CliVariant,
        //context: <TransferNEARTokensAction as ToCli>::Context { connection_config: Option<crate::common::ConnectionConfig>, sender_account_id: near_primitives::types::AccountId },
        //context: (Option<crate::common::ConnectionConfig>, sender_account_id: near_primitives::types::AccountId),
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        let sender_account_id = context
            .sender_account_id
            .clone()
            .expect("wrong sender_account_id");
        let amount: crate::common::NearBalance = match context.connection_config.clone() {
            Some(network_connection_config) => {
                let account_balance: crate::common::NearBalance =
                    match crate::common::check_account_id(
                        network_connection_config.clone(),
                        sender_account_id.clone(),
                    )? {
                        Some(account_view) => {
                            crate::common::NearBalance::from_yoctonear(account_view.amount)
                        }
                        None => crate::common::NearBalance::from_yoctonear(0),
                    };
                match item.amount {
                    Some(cli_amount) => {
                        if cli_amount <= account_balance {
                            cli_amount
                        } else {
                            println!(
                                "You need to enter a value of no more than {}",
                                account_balance
                            );
                            TransferNEARTokensAction::input_amount(Some(account_balance))
                        }
                    }
                    None => TransferNEARTokensAction::input_amount(Some(account_balance)),
                }
            }
            None => match item.amount {
                Some(cli_amount) => cli_amount,
                None => TransferNEARTokensAction::input_amount(None),
            },
        };
        let sign_option = match item.sign_option {
            Some(cli_sign_transaction) => crate::commands::construct_transaction_command::sign_transaction::SignTransaction::from(cli_sign_transaction, context)?,
            None => crate::commands::construct_transaction_command::sign_transaction::SignTransaction::choose_variant(context)?,
        };
        Ok(Self {
            amount,
            sign_option,
        })
    }
}

impl TransferNEARTokensAction {
    fn input_amount(
        account_balance: Option<crate::common::NearBalance>,
    ) -> crate::common::NearBalance {
        match account_balance {
            Some(account_balance) => loop {
                let input_amount: crate::common::NearBalance = Input::new()
                            .with_prompt("How many NEAR Tokens do you want to transfer? (example: 10NEAR or 0.5near or 10000yoctonear)")
                            .with_initial_text(format!("{}", account_balance))
                            .interact_text()
                            .unwrap();
                if input_amount <= account_balance {
                    break input_amount;
                } else {
                    println!(
                        "You need to enter a value of no more than {}",
                        account_balance
                    )
                }
            }
            None => Input::new()
                        .with_prompt("How many NEAR Tokens do you want to transfer? (example: 10NEAR or 0.5near or 10000yoctonear)")
                        .interact_text()
                        .unwrap()
        }
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        let action = near_primitives::transaction::Action::Transfer(
            near_primitives::transaction::TransferAction {
                deposit: self.amount.to_yoctonear(),
            },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        match self
            .sign_option
            .process(unsigned_transaction, network_connection_config.clone())
            .await?
        {
            Some(transaction_info) => {
                crate::common::print_transaction_status(
                    transaction_info,
                    network_connection_config,
                )
                .await;
            }
            None => {}
        };
        Ok(())
    }
}
