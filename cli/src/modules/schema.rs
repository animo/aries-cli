use agent_controller::modules::schema::{SchemaCreateOptions, SchemaModule};
use clap::{Args, Subcommand};

use crate::{
    error::{Error, Result},
    utils::logger::pretty_print_obj,
};

#[derive(Args)]
pub struct SchemaOptions {
    #[clap(subcommand)]
    pub commands: Option<SchemaSubcommands>,

    #[clap(long, short, conflicts_with = "all")]
    pub id: Option<String>,

    #[clap(long, short, conflicts_with = "id")]
    pub all: bool,
}

#[derive(Subcommand, Debug)]
pub enum SchemaSubcommands {
    Create {
        #[clap(short, long)]
        name: String,
        #[clap(short, long, default_value = "1.0")]
        version: String,
        #[clap(short, long)]
        attributes: Vec<String>,
    },
}

pub async fn parse_schema_args(options: &SchemaOptions, agent: impl SchemaModule) -> Result<()> {
    if let Some(id) = &options.id {
        return agent
            .get_by_id(id.to_string())
            .await
            .map(|schema| pretty_print_obj(schema.schema));
    }
    if options.all {
        return agent
            .get_all()
            .await
            .map(|schemas| schemas.schema_ids.iter().for_each(|x| println!("{}", x)));
    }
    match options
        .commands
        .as_ref()
        .ok_or_else(|| Error::NoSubcommandSupplied("schema".to_string()))?
    {
        SchemaSubcommands::Create {
            name,
            attributes,
            version,
        } => {
            let options = SchemaCreateOptions {
                name: name.to_string(),
                version: version.to_string(),
                attributes: attributes.to_vec(),
            };
            if options.attributes.is_empty() {
                return Err(Error::RequiredAttributes.into());
            }
            agent
                .create(options)
                .await
                .map(|schema_id| println!("{}", schema_id))
        }
    }
}
