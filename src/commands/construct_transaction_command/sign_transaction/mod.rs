use dialoguer::{theme::ColorfulTheme, Input, Select};
use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};
use near_primitives::borsh::BorshSerialize;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod sign_manually;
pub mod sign_with_keychain;
pub mod sign_with_ledger;
pub mod sign_with_private_key;

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum SignTransaction {
    /// Provide arguments to sign a private key transaction
    #[strum_discriminants(strum(
        message = "Yes, I want to sign the transaction with my private key"
    ))]
    SignPrivateKey(self::sign_with_private_key::SignPrivateKey),
    /// Provide arguments to sign a keychain transaction
    #[strum_discriminants(strum(message = "Yes, I want to sign the transaction with keychain"))]
    SignWithKeychain(self::sign_with_keychain::SignKeychain),
    /// Connect your Ledger device and sign transaction with it
    #[strum_discriminants(strum(
        message = "Yes, I want to sign the transaction with Ledger device"
    ))]
    SignWithLedger(self::sign_with_ledger::SignLedger),
    /// Provide arguments to sign a manually transaction
    #[strum_discriminants(strum(
        message = "No, I want to construct the transaction and sign it somewhere else"
    ))]
    SignManually(self::sign_manually::SignManually),
}

impl SignTransaction {
    pub fn from(
        item: CliSignTransaction,
        connection_config: Option<crate::common::ConnectionConfig>,
        sender_account_id: near_primitives::types::AccountId,
    ) -> color_eyre::eyre::Result<Self> {
        match item {
            CliSignTransaction::SignPrivateKey(cli_private_key) => {
                let private_key = self::sign_with_private_key::SignPrivateKey::from(
                    cli_private_key,
                    connection_config,
                );
                Ok(SignTransaction::SignPrivateKey(private_key))
            }
            CliSignTransaction::SignWithKeychain(cli_key_chain) => {
                let key_chain = self::sign_with_keychain::SignKeychain::from(
                    cli_key_chain,
                    connection_config,
                    sender_account_id,
                )?;
                Ok(SignTransaction::SignWithKeychain(key_chain))
            }
            CliSignTransaction::SignWithLedger(cli_ledger) => {
                let ledger =
                    self::sign_with_ledger::SignLedger::from(cli_ledger, connection_config)?;
                Ok(SignTransaction::SignWithLedger(ledger))
            }
            CliSignTransaction::SignManually(cli_manually) => {
                let manually =
                    self::sign_manually::SignManually::from(cli_manually, connection_config);
                Ok(SignTransaction::SignManually(manually))
            }
        }
    }
}

impl SignTransaction {
    pub fn choose_sign_option(
        connection_config: Option<crate::common::ConnectionConfig>,
        sender_account_id: near_primitives::types::AccountId,
    ) -> color_eyre::eyre::Result<Self> {
        println!();
        let variants = SignTransactionDiscriminants::iter().collect::<Vec<_>>();
        let sign_options = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let select_sign_options = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to sign the transaction?")
            .items(&sign_options)
            .default(0)
            .interact()
            .unwrap();
        let cli_sign_option = match variants[select_sign_options] {
            SignTransactionDiscriminants::SignPrivateKey => {
                CliSignTransaction::SignPrivateKey(Default::default())
            }
            SignTransactionDiscriminants::SignWithKeychain => {
                CliSignTransaction::SignWithKeychain(Default::default())
            }
            SignTransactionDiscriminants::SignWithLedger => {
                CliSignTransaction::SignWithLedger(Default::default())
            }
            SignTransactionDiscriminants::SignManually => {
                CliSignTransaction::SignManually(Default::default())
            }
        };
        Self::from(cli_sign_option, connection_config, sender_account_id)
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<Option<near_primitives::views::FinalExecutionOutcomeView>> {
        match self {
            SignTransaction::SignPrivateKey(keys) => {
                keys.process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            SignTransaction::SignWithKeychain(chain) => {
                chain
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            SignTransaction::SignWithLedger(ledger) => {
                ledger
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
            SignTransaction::SignManually(args_manually) => {
                args_manually
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
        }
    }
}

fn input_signer_public_key() -> crate::types::public_key::PublicKey {
    Input::new()
        .with_prompt("To create an unsigned transaction enter sender's public key")
        .interact_text()
        .unwrap()
}

fn input_signer_private_key() -> near_crypto::SecretKey {
    Input::new()
        .with_prompt("Enter sender's private key")
        .interact_text()
        .unwrap()
}

fn input_access_key_nonce(public_key: &str) -> u64 {
    println!("Your public key: `{}`", public_key);
    Input::new()
        .with_prompt(
            "Enter transaction nonce for this public key (query the access key information with \
            `./near-cli view nonce \
                network testnet \
                account 'volodymyr.testnet' \
                public-key ed25519:...` incremented by 1)",
        )
        .interact_text()
        .unwrap()
}

fn input_block_hash() -> crate::types::crypto_hash::CryptoHash {
    let input_block_hash: crate::common::BlockHashAsBase58 = Input::new()
        .with_prompt(
            "Enter recent block hash (query information about the hash of the last block with \
            `./near-cli view recent-block-hash network testnet`)",
        )
        .interact_text()
        .unwrap();
    crate::types::crypto_hash::CryptoHash(input_block_hash.inner)
}

#[derive(Debug, EnumDiscriminants, Clone, clap::Clap, ToCliArgs)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum Submit {
    #[strum_discriminants(strum(
        message = "Do you want send the transaction to the server (it's works only for online mode)"
    ))]
    Send,
    #[strum_discriminants(strum(message = "Do you want show the transaction on display?"))]
    Display,
}

impl Submit {
    pub fn choose_submit(connection_config: Option<crate::common::ConnectionConfig>) -> Self {
        println!();
        let variants = SubmitDiscriminants::iter().collect::<Vec<_>>();

        let submits = if let Some(_) = connection_config {
            variants
                .iter()
                .map(|p| p.get_message().unwrap().to_owned())
                .collect::<Vec<_>>()
        } else {
            vec!["Do you want show the transaction on display?".to_string()]
        };
        let select_submit = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action that you want to add to the action:")
            .items(&submits)
            .default(0)
            .interact()
            .unwrap();
        match variants[select_submit] {
            SubmitDiscriminants::Send => Submit::Send,
            SubmitDiscriminants::Display => Submit::Display,
        }
    }

    pub fn process_offline(
        self,
        serialize_to_base64: String,
    ) -> color_eyre::eyre::Result<Option<near_primitives::views::FinalExecutionOutcomeView>> {
        println!("Serialize_to_base64:\n{}", &serialize_to_base64);
        Ok(None)
    }

    pub async fn process_online(
        self,
        network_connection_config: crate::common::ConnectionConfig,
        signed_transaction: near_primitives::transaction::SignedTransaction,
        serialize_to_base64: String,
    ) -> color_eyre::eyre::Result<Option<near_primitives::views::FinalExecutionOutcomeView>> {
        match self {
            Submit::Send => {
                println!("Transaction sent ...");
                let json_rcp_client =
                    near_jsonrpc_client::new_client(network_connection_config.rpc_url().as_str());
                let transaction_info = loop {
                    let transaction_info_result = json_rcp_client
                        .broadcast_tx_commit(near_primitives::serialize::to_base64(
                            signed_transaction
                                .try_to_vec()
                                .expect("Transaction is not expected to fail on serialization"),
                        ))
                        .await;
                    match transaction_info_result {
                        Ok(response) => {
                            break response;
                        }
                        Err(err) => {
                            if let Some(serde_json::Value::String(data)) = &err.data {
                                if data.contains("Timeout") {
                                    println!("Timeout error transaction.\nPlease wait. The next try to send this transaction is happening right now ...");
                                    continue;
                                } else {
                                    println!("Error transaction: {:#?}", err)
                                }
                            };
                            return Ok(None);
                        }
                    };
                };
                Ok(Some(transaction_info))
            }
            Submit::Display => {
                println!("\nSerialize_to_base64:\n{}", &serialize_to_base64);
                Ok(None)
            }
        }
    }
}
