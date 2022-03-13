use agent::modules::schema::{SchemaCreateOptions, SchemaModule};
use clap::{Args, Subcommand};
use log::{debug, info};

use crate::{
    copy,
    error::{Error, Result},
    utils::logger::pretty_print_obj,
    utils::{
        loader::{Loader, LoaderVariant},
        logger::pretty_stringify_obj,
    },
};

#[derive(Args)]
pub struct SchemaOptions {
    #[clap(long, short)]
    pub id: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<SchemaSubcommands>,
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
    let loader = Loader::start(LoaderVariant::default());
    if let Some(id) = &options.id {
        return agent.get_by_id(id.to_string()).await.map(|schema| {
            loader.stop();
            copy!("{}", pretty_stringify_obj(&schema.schema));
            pretty_print_obj(schema.schema)
        });
    }

    match &options.commands {
        Some(o) => match o {
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
                agent.create(options).await.map(|schema| {
                    debug!("{}", pretty_stringify_obj(&schema));
                    copy!("{}", schema.schema_id);
                    info!("{}", schema.schema_id);
                })
            }
        },
        None => agent.get_all().await.map(|schemas| {
            loader.stop();
            schemas.schema_ids.iter().for_each(|x| info!("{}", x))
        }),
    }
}
