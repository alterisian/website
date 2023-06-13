use std::future::IntoFuture;

use futures::{Stream, StreamExt};

use crate::{
    final_error::{FinalError, FinalErrorBuilder},
    target_error::TargetError,
    target_success::TargetSuccess,
};

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
        async {
            self.0
                .fold(FinalErrorBuilder::default(), FinalErrorBuilder::add)
                .await
                .build()
        }
    }
}
