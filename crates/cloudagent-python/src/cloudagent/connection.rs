use crate::agent::CloudAgentPython;
use crate::fill_query;
use agent::error::Result;
use agent::modules::connection::{
    Connection, ConnectionCreateInvitationOptions, ConnectionCreateInvitationResponse,
    ConnectionGetAllOptions, ConnectionGetAllResponse, ConnectionModule,
    ConnectionReceiveInvitationOptions,
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
impl ConnectionModule for CloudAgentPython {
    async fn get_all(&self, options: ConnectionGetAllOptions) -> Result<ConnectionGetAllResponse> {
        let url = self.create_url(&["connections"])?;

        let query = fill_query!(
            options,
            alias,
            connection_protocol,
            invitation_key,
            my_did,
            state,
            their_did,
            their_role
        );

        self.get(url, Some(query)).await
    }

    async fn get_by_id(&self, id: String) -> Result<Connection> {
        let url = self.create_url(&["connections", &id])?;
        self.get::<Connection>(url, None).await
    }

    async fn create_invitation(
        &self,
        options: ConnectionCreateInvitationOptions,
    ) -> Result<ConnectionCreateInvitationResponse> {
        let url = self.create_url(&["connections", "create-invitation"])?;
        let mut query: Vec<(&str, String)> = vec![];

        let body = if options.toolbox {
            query.push(("multi_use", false.to_string()));
            query.push(("auto_accept", true.to_string()));
            query.push(("alias", String::from("toolbox")));

            Some(json!({
                "metadata": {
                    "group": "admin"
                }
            }))
        } else {
            if options.multi_use {
                query.push(("multi_use", true.to_string()));
            }
            if options.auto_accept {
                query.push(("auto_accept", true.to_string()));
            }
            if let Some(alias) = &options.alias {
                query.push(("alias", alias.to_string()));
            }
            None
        };

        self.post::<ConnectionCreateInvitationResponse>(url, Some(query), body)
            .await
    }
    async fn receive_invitation(
        &self,
        invitation: ConnectionReceiveInvitationOptions,
    ) -> Result<Connection> {
        let url = self.create_url(&["connections", "receive-invitation"])?;

        self.post(url, None, Some(serde_json::to_value(invitation)?))
            .await
    }
}
