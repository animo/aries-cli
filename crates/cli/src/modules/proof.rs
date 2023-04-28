use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use clap::{Args, Subcommand};
use siera_agent::modules::proof::{Predicate, ProofModule, ProofRequestOptions};

/// Proof options and flags
#[derive(Args)]
pub struct ProofOptions {
    /// All the subcommands of the proof cli
    #[clap(subcommand)]
    pub commands: ProofSubcommands,
}

/// Proof Subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Proof)]
pub enum ProofSubcommands {
    /// Send a proof request to the connection id
    #[clap(about = HelpStrings::ProofRequest)]
    Request {
        /// Connection id to send the proof request to
        #[clap(short, long, help = HelpStrings::ProofRequestConnectionId)]
        connection_id: String,

        /// The name of the proof request
        #[clap(short, long, default_value = "proof-request", help = HelpStrings::ProofRequestName)]
        name: String,

        /// A list of predicates that are supposed to be in the proof request
        /// e.g. age,>=,18
        #[clap(short, long, help = HelpStrings::ProofRequestPredicate)]
        predicate: Vec<Predicate>,

        /// List of attributes that the receiver must send back to fulfill the request
        #[clap(short, long, help = HelpStrings::ProofRequestAttribute)]
        attribute: Vec<String>,
    },
}

/// Subcoammnd Proof parser
pub async fn parse_proof_args(
    commands: &ProofSubcommands,
    agent: impl ProofModule + Send + Sync,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());
    match commands {
        ProofSubcommands::Request {
            connection_id,
            attribute,
            predicate,
            name,
        } => {
            let predicates: Vec<(String, String, i32)> = predicate
                .iter()
                .map(|p| {
                    (
                        p.0.clone(),
                        p.1.clone(),
                        p.2.parse::<i32>()
                            .map_err(|_| Error::PredicateValueNonNumber(p.0.clone(), p.2.clone()))
                            .unwrap(),
                    )
                })
                .collect();
            let proof_request_options = ProofRequestOptions {
                connection_id: connection_id.clone(),
                name: name.clone(),
                attributes: attribute.clone(),
                predicates,
            };
            agent
                .send_request(proof_request_options)
                .await
                .map(|proof| {
                    debug!({ "proof": proof });
                    info!({ "message": "Successefully requested a proof", "presentation_exchange_id": &proof.presentation_exchange_id});
                    copy!("{}", &proof.presentation_exchange_id);
                })?;
            loader.stop();
            Ok(())
        }
    }
}
