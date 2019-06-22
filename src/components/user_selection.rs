

use failure::Error;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::fetch::FetchTask;

use crate::services::froovie_service::{FroovieService, Selections, Movie};

pub struct UserSelectionModel {
    froovie: FroovieService,
    callback: Callback<Result<Selections, Error>>,
    pub selections: Option<Selections>,
    task: Option<FetchTask>,
    error: Option<String>,
}

pub enum Msg {
    Selections,
    FroovieReady(Result<Selections, Error>),
}

impl Component for UserSelectionModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        UserSelectionModel {
            froovie: FroovieService::new(),
            callback: link.send_back(Msg::FroovieReady),
            selections: None,
            task: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selections => {
                let task = self.froovie.get_user_selection("1", self.callback.clone());
                self.task = Some(task);
            }
            Msg::FroovieReady(Ok(selections)) => {
                self.selections = Some(selections);
            }
            Msg::FroovieReady(Err(error)) => {
                self.error = Some(error.to_string());
            }
        }
        true
    }
}

impl Renderable<UserSelectionModel> for UserSelectionModel {
    fn view(&self) -> Html<Self> {
        let view_movie = |movie| html! {
            <li> { movie } </li>
        };

        let selections = self.selections.as_ref();
        let movies: Vec<Movie> = selections.iter()
            .flat_map(|user|user.movies.clone())
            .collect(); 

        html! {
            <div>
                <button onclick=|_| Msg::Selections,>{ "Get  " }</button>
                <ul> { for movies.iter().map(|movie| view_movie(movie.title.clone())) } </ul>
                <p> { &format!("Error status : {:?}", &self.error) } </p>
            </div>
        }
    }
}