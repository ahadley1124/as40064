use yew::prelude::*;
use crate::site::header::Header;

#[function_component(About)]
pub fn about() -> Html {
    html! {
    <>
        <Header />
        <div class="about">
            <h1>{ "About Amateur IX" }</h1>
            <p>{ "Amateur IX is an Internet Exchange designed for the AMPR network, supporting 44/9 and 44.128/10 address space. Our mission is to facilitate peering and connectivity for amateur radio IP networks, fostering collaboration and technical advancement in the community." }</p>
            <h2>{ "Exchange Policy" }</h2>
            <ul>
                <li>{ "Open peering: All networks with AMPR address space (44/9, 44.128/10) are welcome to peer." }</li>
                <li>{ "No commercial traffic: The exchange is strictly for amateur radio and research purposes." }</li>
                <li>{ "Respect and collaboration: Members are expected to maintain a respectful and collaborative environment." }</li>
                <li>{ "Technical requirements: Participants must maintain secure and reliable connections, and follow best practices for routing and filtering." }</li>
                <li>{ "Policy updates: The exchange policy may be updated as the community grows and needs evolve." }</li>
            </ul>
            <p>{ "For more information or to join, please check PeeringDB or contact us once the contact page is available." }</p>
            <p>
                { "You can also view the full Exchange policy as a PDF: " }
                <a href="/ExchangePolicy.pdf" target="_blank">{ "Download Exchange Policy (PDF)" }</a>
            </p>
        </div>
    </>
    }
}