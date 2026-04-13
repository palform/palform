use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::submissions::{
    assets::download::submissions_assets_download, assets::upload::submissions_assets_upload,
    crypto::submissions_crypto, delete::submissions_delete, list::submissions_list,
    num_since::submissions_num_since,
};

pub fn submissions_org_scope() -> Scope {
    scope("submissions").service(resource("").route(post().to(submissions_num_since)))
}

pub fn submissions_form_scope() -> Scope {
    scope("submissions")
        .service(resource("").route(get().to(submissions_list)))
        .service(
            scope("assets")
                .service(resource("{file_id}").route(get().to(submissions_assets_download))),
        )
        .service(
            scope("{submission_id}")
                .service(resource("crypto").route(get().to(submissions_crypto)))
                .service(resource("").route(delete().to(submissions_delete))),
        )
}

pub fn submissions_fill_assets_scope() -> Scope {
    scope("submission_assets").service(resource("").route(post().to(submissions_assets_upload)))
}
