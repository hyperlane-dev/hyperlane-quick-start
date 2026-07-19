use super::*;

/// Route structure for the euv playground view endpoint.
///
/// `GET /euv-playground` redirects to the static SPA shell
/// `/static/euv-playground/index.html`, which loads the euv playground
/// page from `pkg/euv_page_euv_playground.js` and mounts the editor +
/// preview UI in the browser.
#[route("/euv-playground")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundViewRoute;
