use crate::{
    entities::app::parser::BelongToId,
    errors::{ErrorBuilder, WorkspaceError, WsErrCode},
};
use flowy_derive::ProtoBuf;
use std::convert::TryInto;

#[derive(Default, ProtoBuf)]
pub struct DeleteAppRequest {
    #[pb(index = 1)]
    pub app_id: String,
}

pub struct DeleteAppParams {
    pub app_id: String,
}

impl TryInto<DeleteAppParams> for DeleteAppRequest {
    type Error = WorkspaceError;

    fn try_into(self) -> Result<DeleteAppParams, Self::Error> {
        let app_id = BelongToId::parse(self.app_id)
            .map_err(|e| ErrorBuilder::new(WsErrCode::AppIdInvalid).msg(e).build())?
            .0;

        Ok(DeleteAppParams { app_id })
    }
}
