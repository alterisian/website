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

pub struct GenerationTask<T>(T);

impl<T> GenerationTask<T> {
    pub(crate) fn new(stream: T) -> Self {
        Self(stream)
    }
}

impl<T> IntoFuture for GenerationTask<T>
where
    T: Stream<Item = Result<TargetSuccess, TargetError>>,
{
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
