use crate::request::prelude::*;
use twilight_model::{channel::PrivateChannel, id::UserId};

#[derive(Serialize)]
struct CreatePrivateChannelFields {
    recipient_id: UserId,
}

pub struct CreatePrivateChannel<'a> {
    fields: CreatePrivateChannelFields,
    fut: Option<Pending<'a, PrivateChannel>>,
    http: &'a Client,
}

impl<'a> CreatePrivateChannel<'a> {
    pub(crate) fn new(http: &'a Client, recipient_id: UserId) -> Self {
        Self {
            fields: CreatePrivateChannelFields {
                recipient_id,
            },
            fut: None,
            http,
        }
    }
    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreatePrivateChannel,
        )))));

        Ok(())
    }
}

poll_req!(CreatePrivateChannel<'_>, PrivateChannel);
