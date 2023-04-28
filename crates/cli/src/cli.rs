use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::help_strings::HelpStrings;

use crate::modules::automation::AutomationOptions;
use crate::modules::multitenancy::MultitenancyOptions;
use crate::modules::{
    basic_message::BasicMessageOptions, configuration::ConfigurationOptions,
    connection::ConnectionOptions, credential::CredentialOptions,
    credential_definition::CredentialDefinitionOptions, feature::FeaturesOptions, oob::OobOptions,
    proof::ProofOptions, schema::SchemaOptions, wallet::WalletOptions, webhook::WebhookOptions,
};

/// Main command with options, flags and subcommands
#[derive(Parser)]
#[clap(name = "siera", author, version, about = HelpStrings::Cli)]
#[clap(arg_required_else_help = true, disable_help_subcommand = true)]
pub struct Cli {
    /// The agent url used for commandos
    #[clap(long, short='u', help = HelpStrings::AgentURL)]
    pub agent_url: Option<String>,

    /// The api key used for agent authentication
    #[clap(long, short, help = HelpStrings::ApiKey)]
    pub api_key: Option<String>,

    /// The multi tenancy token
    #[clap(long, short = 't', help = HelpStrings::ConfigurationInitializeToken)]
    pub token: Option<String>,

    /// The agent type
    #[clap(long, short = 'f', help = HelpStrings::Agent)]
    pub agent: Option<String>,

    /// Whether specific output should be copied to the clipboard
    #[clap(long, short, help = HelpStrings::Copy)]
    pub copy: bool,

    /// Whether specific output should be copied to the clipboard
    #[clap(long = "json", short = 'j', help = HelpStrings::OutputJson)]
    pub output_json: bool,

    /// Whether the output should be quiet
    #[clap(long, short, help = HelpStrings::Quiet, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Which config path to use instead of the default one
    #[clap(long, short = 'o', help = HelpStrings::Config)]
    pub config: Option<PathBuf>,

    /// The environment which to use
    #[clap(long, short, default_value = "default", help = HelpStrings::Environment)]
    pub environment: String,

    /// Whether more verbose output should be printed
    #[clap(long, short='v', help = HelpStrings::Verbose, action = clap::ArgAction::Count, conflicts_with = "quiet")]
    pub verbose: u8,

    /// The main cli subcommands
    #[clap(subcommand)]
    pub commands: Commands,
}

/// All the subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Connection subcommands
    Connection(ConnectionOptions),

    /// Webhook subcommands
    Webhook(WebhookOptions),

    /// Oob subcommands
    Oob(OobOptions),

    /// Feature subcommands
    Feature(FeaturesOptions),

    /// Schema subcommands
    Schema(SchemaOptions),

    /// Credential definition subcommands
    CredentialDefinition(CredentialDefinitionOptions),

    /// BasicMessage subcommands
    Message(BasicMessageOptions),

    /// Credential subcommands
    Credential(CredentialOptions),

    /// Configuration subcommands
    Configuration(ConfigurationOptions),

    /// Automation subcommands
    Automate(AutomationOptions),

    /// Proof subcommands
    Proof(ProofOptions),

    /// Multitenancy subcommands
    Multitenancy(MultitenancyOptions),

    /// Wallet subcommands
    Wallet(WalletOptions),
}

impl From<Commands> for String {
    fn from(c: Commands) -> Self {
        let s = match c {
            Commands::Automate(_) => "Automate",
            Commands::Connection(_) => "Connection",
            Commands::Webhook(_) => "Webhook",
            Commands::Oob(_) => "Oob",
            Commands::Feature(_) => "Feature",
            Commands::Schema(_) => "Schema",
            Commands::CredentialDefinition(_) => "CredentialDefinition",
            Commands::Message(_) => "Message",
            Commands::Credential(_) => "Credential",
            Commands::Configuration(_) => "Configuration",
            Commands::Proof(_) => "Proof",
            Commands::Multitenancy(_) => "Multitenancy",
            Commands::Wallet(_) => "Wallet",
        };

        Self::from(s)
    }
}
