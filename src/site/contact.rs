use yew::prelude::*;
use crate::site::header::Header;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
    <>
        <Header />
        <div class="contact">
            <h1>{ "Contact" }</h1>
            <p>{ "We will add a contact page shortly. For now, please check " }
                <a href="https://peeringdb.com" target="_blank">{ "PeeringDB" }</a>
                { " for more information." }
            </p>
        </div>
    </>
    }
}