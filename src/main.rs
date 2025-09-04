use yew::prelude::*;
use yew_router::prelude::*;
mod site;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/pricing")]
    Pricing,
    #[at("/ExchangePolicy.pdf")]
    Policy,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <site::home::Home /> },
        Route::About => html! { <site::about::About /> },
        Route::Contact => html! { <site::contact::Contact /> },
        Route::Pricing => html! { <site::pricing::Pricing /> },
        Route::Policy => html! {
            <iframe src="/Amateur_Virtual_Internet_Exchange_Policy_V1.0.pdf" width="100%" height="800px" title="Exchange Policy PDF" />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}