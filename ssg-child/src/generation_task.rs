use std::{
    future::IntoFuture,
    pin::{pin, Pin},
};

use futures::{future::BoxFuture, stream::BoxStream, Future, FutureExt, Stream, StreamExt};

use crate::{
    final_error::{FinalError, FinalErrorBuilder},
    target_error::TargetError,
    target_success::TargetSuccess,
};

pub struct GenerationTask(Pin<Box<dyn Stream<Item = Result<TargetSuccess, TargetError>>>>);

impl GenerationTask {
    pub(crate) fn new(stream: impl Stream<Item = Result<TargetSuccess, TargetError>> + 'static) -> Self {
        Self(Box::pin(stream))
    }
}

impl IntoFuture for GenerationTask {
    type Output = Result<(), FinalError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(self) -> Self::IntoFuture {
        let future = async {
            let final_error = self
                .0
                .fold(FinalErrorBuilder::default(), |builder, result| async {
                    builder.add(result);
                    builder
                })
                .await
                .build();
            if let Some(final_error) = final_error {
                Err(final_error)
            } else {
                Ok(())
            }
        };

        Box::pin(future)
    }
}
