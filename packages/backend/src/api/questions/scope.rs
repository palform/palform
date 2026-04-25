use apistos::web::{get, post, resource, scope, Scope};

use crate::api::questions::{get::questions_get, list::questions_list, save::questions_save};

pub fn questions_scope() -> Scope {
    scope("content")
        .service(resource("save").route(post().to(questions_save)))
        .service(
            scope("groups")
                .service(resource("all/questions").route(get().to(questions_list)))
                .service(
                    scope("{question_group_id}").service(
                        scope("questions")
                            .service(resource("{question_id}").route(get().to(questions_get))),
                    ),
                ),
        )
}
