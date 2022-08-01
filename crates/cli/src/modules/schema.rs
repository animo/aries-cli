use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use agent::modules::schema::{SchemaCreateOptions, SchemaModule};
use clap::{Args, Subcommand};
use logger::pretty_stringify_obj;

/// Schema options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Schema)]
pub struct SchemaOptions {
    /// All the subcommands of the schema cli
    #[clap(subcommand)]
    pub commands: SchemaSubcommands,
}

/// Schema subcommands
#[derive(Subcommand, Debug)]
pub enum SchemaSubcommands {
    /// Register a new schema on the ledger
    #[clap(about = HelpStrings::SchemaCreate)]
    Create {
        /// The name of schema
        #[clap(short, long, help=HelpStrings::SchemaCreateName)]
        name: String,

        /// The version of the schema
        #[clap(short, long, help=HelpStrings::SchemaCreateVersion, default_value = "1.0")]
        version: String,

        /// A list of strings of the attributes that must be met when issuing a credential
        /// with this schema
        #[clap(short, long, help=HelpStrings::SchemaCreateAttributes, required = true)]
        attribute: Vec<String>,
    },

    /// List all the registered schemas
    #[clap(about = HelpStrings::SchemaList)]
    List {
        /// Filter by id to fetch a specific schema
        #[clap(long, short, help=HelpStrings::SchemaId)]
        id: Option<String>,
    },
}

/// Subcommand Schema parser
pub async fn parse_schema_args(options: &SchemaOptions, agent: impl SchemaModule) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());
    match &options.commands {
        SchemaSubcommands::Create {
            name,
            version,
            attribute,
        } => {
            let options = SchemaCreateOptions {
                name: name.to_owned(),
                version: version.to_owned(),
                attributes: attribute.to_vec(),
            };
            if options.attributes.is_empty() {
                return Err(Error::RequiredAttributes.into());
            }
            agent.create(options).await.map(|schema| {
                log_debug!("{}", pretty_stringify_obj(&schema));
                log_info!("Created schema with the following attributes: ",);
                schema
                    .attr_names
                    .into_iter()
                    .for_each(|name| log_info!("- {}", name));
                log_info!("Schema id:");
                log!("{}", schema.id);
                copy!("{}", schema.id);
            })
        }
        SchemaSubcommands::List { id } => match id {
            Some(i) => agent.get_by_id(i.to_owned()).await.map(|schema| {
                loader.stop();
                copy!("{}", pretty_stringify_obj(&schema));
                log!("{}", pretty_stringify_obj(schema));
            }),
            None => agent.get_all().await.map(|schemas| {
                loader.stop();
                schemas.schema_ids.iter().for_each(|x| log!("{}", x));
                log_info!("Successfully fetched schema IDs");
            }),
        },
    }
}
