use ssg::Asset;

use crate::{calendar, fonts, graphic_assets, pages};

pub(crate) async fn get() -> impl Iterator<Item = Asset> {
    let fonts = fonts::assets();
    let pages = pages::all().await;

    [calendar::js_library_asset()]
        .into_iter()
        .chain(fonts)
        .chain(graphic_assets::get())
        .chain(pages)
}