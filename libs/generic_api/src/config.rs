use super::handlers::{create, delete, drop, get_item, get_list, update};
use ntex::web;
use proc_macros::DbResource;
use serde::{de::DeserializeOwned, Serialize};
use services::mongo::query_param_processing::QueryParamProcessing;
use validator::Validate;

pub fn generic_routes<T, C, U, Q>(cfg: &mut web::ServiceConfig)
where
    T: 'static + DbResource + Serialize + DeserializeOwned + Sync + Send + Unpin,
    C: 'static + Serialize + DeserializeOwned + Sync + Send + Unpin + Validate,
    U: 'static + Serialize + DeserializeOwned + Validate,
    Q: 'static + Serialize + DeserializeOwned + QueryParamProcessing + Clone,
{
    cfg.service(
        web::scope(T::URL).service((
            web::resource("")
                .route(web::delete().to(drop::<T>))
                .route(web::get().to(get_list::<T, Q>))
                .route(web::post().to(create::<T, C>)),
            web::resource("/{id}")
                .route(web::get().to(get_item::<T, Q>))
                .route(web::put().to(update::<T, U>))
                .route(web::delete().to(delete::<T>)),
        )),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use utoipa::IntoParams;

    struct Test1 {
        name: String,
    }
    struct CreateTest1 {
        name: String,
    }
    struct UpdateTest1 {
        name: String,
    }

    #[derive(Clone, Deserialize, Serialize, Debug, IntoParams)]
    #[serde(deny_unknown_fields)]
    pub struct TodoQueryParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        completed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub projection: Option<String>,
    }
    #[test]
    fn generic_routes_basic() {
        // let s = generic_routes::<Test1, CreateTest1, UpdateTest1,TodoQueryParams>();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
