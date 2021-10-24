use dialoguer::{theme::ColorfulTheme, Select};
use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

// pub mod add_command;
pub mod construct_transaction_command;
// pub mod delete_command;
// pub mod execute_command;
// pub mod generate_shell_completions_command;
// pub mod login;
pub mod transfer_command;
pub mod utils_command;
// pub mod view_command;

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = crate::common::Context)]
///Choose transaction action
pub enum TopLevelCommand {
    // #[strum_discriminants(strum(message = "Login with wallet authorization"))]
    // Login(self::login::operation_mode::OperationMode),
    // #[strum_discriminants(strum(
    //     message = "View account, contract code, contract state, transaction, nonce, recent block hash"
    // ))]
    // View(self::view_command::ViewQueryRequest),
    ///Use these to transfer tokens
    #[strum_discriminants(strum(message = "Transfer tokens"))]
    Transfer(self::transfer_command::Currency),
    // #[strum_discriminants(strum(message = "Execute function (contract method)"))]
    // Execute(self::execute_command::OptionMethod),
    // #[strum_discriminants(strum(
    //     message = "Add access key, contract code, stake proposal, sub-account, implicit-account"
    // ))]
    // Add(self::add_command::AddAction),
    // #[strum_discriminants(strum(message = "Delete access key, account"))]
    // Delete(self::delete_command::DeleteAction),
    // #[strum_discriminants(strum(message = "Construct a new transaction"))]
    // ConstructTransaction(self::construct_transaction_command::operation_mode::OperationMode),
    // #[strum_discriminants(strum(message = "Helpers"))]
    // Utils(self::utils_command::Utils),
}

impl TopLevelCommand {
    pub async fn process(self) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: near_primitives::types::AccountId::test_account(),
            public_key: near_crypto::PublicKey::empty(near_crypto::KeyType::ED25519),
            nonce: 0,
            receiver_id: near_primitives::types::AccountId::test_account(),
            block_hash: Default::default(),
            actions: vec![],
        };
        match self {
            // Self::Add(add_action) => add_action.process(unsigned_transaction).await,
            // Self::ConstructTransaction(mode) => mode.process(unsigned_transaction).await,
            // Self::Delete(delete_action) => delete_action.process(unsigned_transaction).await,
            // Self::Execute(option_method) => option_method.process(unsigned_transaction).await,
            // Self::Login(mode) => mode.process().await,
            Self::Transfer(currency) => currency.process(unsigned_transaction).await,
            // Self::Utils(util_type) => util_type.process().await,
            // Self::View(view_query_request) => view_query_request.process().await,
        }
    }
}
