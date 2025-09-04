use yew::prelude::*;
use crate::site::header::Header;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
    <>
        <div class="home-container">
            <Header />
        </div>
        <div class="home">
            <h1>{ "Amateur IX" }</h1>
            <p>{ "Welcome to the beginning of Amateur IX, an Internet Exchange designed for the AMPR network, supporting 44/9 and 44.128/10 address space." }</p>
            <p>{ "This project aims to facilitate peering and connectivity for amateur radio IP networks." }</p>
            <p>{ "Stay tuned for updates and more information as we build out this exchange." }</p>
        </div>
    </>
    }
}