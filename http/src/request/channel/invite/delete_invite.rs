use crate::request::prelude::*;

/// Delete an invite by its code.
pub struct DeleteInvite<'a> {
    code: String,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> DeleteInvite<'a> {
    pub(crate) fn new(http: &'a Client, code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            fut: None,
            http,
            reason: None,
        }
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                headers,
                Route::DeleteInvite {
                    code: self.code.clone(),
                },
            ))
        } else {
            Request::from(Route::DeleteInvite {
                code: self.code.clone(),
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteInvite<'_>, ());
