mod router;
mod routing;
mod components;
use components::user_selection::UserSelectionModel;
use components::search_movie::MovieSearchModel;

pub mod services;

use log::info;
use router::Route;
use yew::{html, Bridge, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::agent::Bridged;


pub enum Child {
    UserSelection,
    MovieSearch,
    PathNotFound(String)
}

pub struct Model {
    child: Child,
    router: Box<dyn Bridge<router::Router<()>>>
}

pub enum Msg {
    NavigateTo(Child),
    HandleRoute(Route<()>)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {

        let callback = link.send_back(Msg::HandleRoute);
        let mut router = router::Router::bridge(callback);

        // TODO Not sure if this is technically correct. This should be sent _after_ the component has been created.
        // I think the `Component` trait should have a hook called `on_mount()`
        // that is called after the component has been attached to the vdom.
        // It seems like this only works because the JS engine decides to activate the
        // router worker logic after the mounting has finished.
        router.send(router::Request::GetCurrentRoute);

        Model {
            child: Child::UserSelection, // This should be quickly overwritten by the actual route.
            router
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(child) => {

                let path_segments = match child {
                    Child::UserSelection => vec!["my_selection".into()],
                    Child::MovieSearch => vec!["movies_search".into()],
                    Child::PathNotFound(_) => vec!["path_not_found".into()]
                };

                let route = router::Route {
                    path_segments,
                    query: None,
                    fragment: None,
                    state: (),
                };

                self.router.send(router::Request::ChangeRoute(route));
                false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                // Instead of each component selecting which parts of the path are important to it,
                // it is also possible to match on the `route.to_route_string().as_str()` once
                // and create enum variants representing the different children and pass them as props.
                self.child = if let Some(first_segment) = route.path_segments.get(0) {
                   match first_segment.as_str() {
                       "my_selection" => Child::UserSelection,
                       "movies_search" => Child::MovieSearch,
                        other => Child::PathNotFound(other.into())
                   }
                } else {
                    Child::PathNotFound("path_not_found".into())
                };

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::NavigateTo(Child::UserSelection),>{ "My Movies" }</button>
                    <button onclick=|_| Msg::NavigateTo(Child::MovieSearch),>{ "Search a Movie" }</button>
                </nav>
                <div>
                    {self.child.view()}
                </div>
            </div>
        }
    }
}

impl Renderable<Model> for Child {
    fn view(&self) -> Html<Model> {
        match *self {
            Child::MovieSearch => html! {
                <>
                    {" Search a movie : "}
                    <MovieSearchModel: />
                </>
            },
            Child::UserSelection => html! {
                <>
                    {"User Selection: "}
                    <UserSelectionModel: />
                </>
            },
            Child::PathNotFound(ref path) => html! {
                <>
                    {format!("Invalid path: '{}'", path)}
                </>
            }
        }
    }
}

