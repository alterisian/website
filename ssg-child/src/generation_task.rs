use std::future::IntoFuture;

use futures::{future::BoxFuture, stream::BoxStream, FutureExt, Stream, StreamExt};

use crate::{
    final_error::{FinalError, FinalErrorBuilder},
    target_error::TargetError,
    target_success::TargetSuccess,
};

pub struct GenerationTask(Box<dyn Stream<Item = Result<TargetSuccess, TargetError>> + Sized >);

impl GenerationTask {
    pub(crate) fn new(stream: Box<dyn Stream<Item = Result<TargetSuccess, TargetError>>>) -> Self {
        Self(stream)
    }
}

impl IntoFuture for GenerationTask {
    type Output = Result<(), FinalError>;
    type IntoFuture = BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        async {
            let final_error = self
                .0
                .fold(FinalErrorBuilder::default(), FinalErrorBuilder::add)
                .await
                .build();
            if let Some(final_error) = final_error {
                Err(final_error)
            } else {
                Ok(())
            }
        }
        .boxed()
    }
}
