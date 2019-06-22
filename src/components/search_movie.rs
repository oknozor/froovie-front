
use failure::Error;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::fetch::StatusCode;
use yew::services::fetch::FetchTask;

use crate::services::froovie_service::{FroovieService, MovieSearch};

pub struct MovieSearchModel {
    froovie: FroovieService,
    callback: Callback<Result<Vec<MovieSearch>, Error>>,
    add_selection_callback: Callback<Result<StatusCode, Error>>,
    pub result: Vec<MovieSearch>,
    task: Option<FetchTask>,
    error: Option<String>,
}

pub enum Msg {
    SearchResult(String),
    PickSelection(i32),
    SelectionResult(Result<StatusCode, Error>),
    FroovieReady(Result<Vec<MovieSearch>, Error>),
}

impl Component for MovieSearchModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        MovieSearchModel {
            froovie: FroovieService::new(),
            callback: link.send_back(Msg::FroovieReady),
            add_selection_callback: link.send_back(Msg::SelectionResult),
            result: vec![],
            task: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SearchResult(query) => {
                let task = self.froovie.search_movie(&query, self.callback.clone());
                self.task = Some(task);
            }
            Msg::FroovieReady(Ok(movies)) => {
                self.result = movies;
            }
            Msg::SelectionResult(Ok(StatusCode::OK)) => {
                //
            },
            Msg::SelectionResult(_) => {
                //
            }
            Msg::SelectionResult(Err(error)) => {
                //
            }
            Msg::FroovieReady(Err(error)) => {
                self.result = vec![]; 
                self.error = Some(error.to_string());
            }
            Msg::PickSelection(moviedb_id) => {
                let task = self.froovie.post_user_selection(moviedb_id, 1, self.add_selection_callback.clone());
                self.task = Some(task);
            }
        }
        true
    }
}

impl Renderable<MovieSearchModel> for MovieSearchModel {
    fn view(&self) -> Html<Self> {
        let view_movie = |movie: &MovieSearch, id: i32| html! {
            <div> 
                <p> { &movie.title.clone() } </p> 
                <img src={ &movie.image_url.clone().unwrap_or_else(|| "".to_string()) },
                    style="width: 200px",/>
                <button onclick=|_| Msg::PickSelection(id),> { "Save"} </button>

            </div>
        };

        html! {
            <div>
            <textarea class=("search-movie", "input"),
               placeholder="Search",
               value="brute",
               oninput=|query| Msg::SearchResult(query.value),
               />
                <ul> { for self.result.iter()
                    .map(|movie| (movie, movie.moviedb_id))
                    .map(|(movie, id)| view_movie(movie, id)) } </ul>
                <p> { &format!("Error status: {:?}", &self.error) } </p>
            </div>
        }
    }
}