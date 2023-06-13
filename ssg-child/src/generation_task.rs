use std::future::IntoFuture;

use futures::{Stream, StreamExt};

use crate::{target_error::TargetError, target_success::TargetSuccess, final_error::FinalError};

pub struct GenerationTask(Box<dyn Stream<Item = Result<TargetSuccess, TargetError>>>);

impl GenerationTask {
    pub(crate) fn new(
        stream: impl Stream<Item = Result<TargetSuccess, TargetError>> + 'static,
    ) -> Self {
        Self(Box::new(stream))
    }
}

impl IntoFuture for GenerationTask {
    type Output = Result<(), FinalError>;

    type IntoFuture = ();

    fn into_future(self) -> Self::IntoFuture {
        self.0.fold(FinalErrorBuilder, |)
    }
}
