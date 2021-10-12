use dialoguer::{theme::ColorfulTheme, Select};
use interactive_clap::ToCli;
use interactive_clap_derive::{InteractiveClap, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

pub mod operation_mode;
mod receiver;
mod sender;
pub mod transfer_near_tokens_type;

#[derive(Debug, Clone, InteractiveClap)]
pub struct Currency {
    #[interactive_clap(subcommand)]
    currency_selection: CurrencySelection,
}

impl Currency {
    pub fn from(item: CliCurrency) -> color_eyre::eyre::Result<Self> {
        let currency_selection = match item.currency_selection {
            Some(cli_currency_selection) => CurrencySelection::from(cli_currency_selection)?,
            None => CurrencySelection::choose_currency()?,
        };
        Ok(Self { currency_selection })
    }
}

impl Currency {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.currency_selection
            .process(prepopulated_unsigned_transaction)
            .await
    }
}

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
enum CurrencySelection {
    /// The transfer is carried out in NEAR tokens
    #[strum_discriminants(strum(message = "NEAR tokens"))]
    Near(self::operation_mode::OperationMode),
}

impl CurrencySelection {
    fn from(item: CliCurrencySelection) -> color_eyre::eyre::Result<Self> {
        match item {
            CliCurrencySelection::Near(cli_operation_mode) => Ok(Self::Near(
                self::operation_mode::OperationMode::from(cli_operation_mode)?,
            )),
        }
    }
}

impl CurrencySelection {
    fn choose_currency() -> color_eyre::eyre::Result<Self> {
        println!();
        let variants = CurrencySelectionDiscriminants::iter().collect::<Vec<_>>();
        let currencies = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let selected_currency = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to transfer?")
            .items(&currencies)
            .default(0)
            .interact()
            .unwrap();
        let cli_currency = match variants[selected_currency] {
            CurrencySelectionDiscriminants::Near => CliCurrencySelection::Near(Default::default()),
        };
        Ok(Self::from(cli_currency)?)
    }

    async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::Near(operation_mode) => {
                operation_mode
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}
