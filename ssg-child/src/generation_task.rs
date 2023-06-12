use std::{future::IntoFuture, pin::Pin, task};

use futures::{Future, Stream, StreamExt};

use crate::{final_error::FinalError, target_error::TargetError, target_success::TargetSuccess};

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

    type IntoFuture = ();

    fn into_future(self) -> Self::IntoFuture {
        final_error::collect_stream_into_result_of_unit_and_final_error(self.0)
    }
}
