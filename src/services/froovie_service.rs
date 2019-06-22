use failure::{format_err, Error};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const BACKEND: &str = "http://localhost:6767";

#[derive(Deserialize, Debug, Clone)]
pub struct Selections {
    pub movies: Vec<Movie>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MovieSearch {
    pub moviedb_id: i32,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Query<'a> {
    pub value: &'a str
}

#[derive(Default)]
pub struct FroovieService {
    web: FetchService,
}

impl FroovieService {
    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn get_user_selection(
        &mut self,
        user_id: &str,
        callback: Callback<Result<Selections, Error>>,
    ) -> FetchTask {
        let url = format!("{}/users/{}/selections", BACKEND, user_id);
        let handler = move |response: Response<Json<Result<Selections, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                callback.emit(Err(format_err!(
                    "{}: error getting user selection from froovie",
                    meta.status
                )))
            }
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }

    pub fn search_movie(
        &mut self,
        query: &str,
        callback: Callback<Result<Vec<MovieSearch>, Error>>,
    ) -> FetchTask {
        let url = format!("{}/movies/search",BACKEND);
        let handler = move |response: Response<Json<Result<Vec<MovieSearch>, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                callback.emit(Err(format_err!(
                    "{}: error getting movie from froovie",
                    meta.status
                )))
            }
        };
        let query = Query{value: query}; 
        let request = Request::post(url.as_str()).body(Json(&query)).unwrap();
        self.web.fetch(request, handler.into())
    }
}
