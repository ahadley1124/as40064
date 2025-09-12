use yew::prelude::*;
use crate::site::header::Header;

#[function_component(Peers)]
pub fn peers() -> Html {
    html! {
    <>
        <Header />
        <div class="peers">
            <h1>{ "Peers" }</h1>
            <p>{ "Current peering partners connected to Amateur IX." }</p>
            <table>
                <tbody>
                    <tr>
                        <td>{ "Peer Name" }</td>
                        <td>{ "AS Number" }</td>
                    </tr>
                    <tr>
                        <td>{ "IPv4 Address" }</td>
                        <td>{ "IPv6 Address" }</td>
                    </tr>
                </tbody>
            </table>
        </div>
        <style>
            { peers_style() }
        </style>
    </>
    }
}

fn peers_style() -> String {
    r#"
    .peers {
        max-width: 900px;
        margin: 0 auto;
        padding: 2rem;
    }
    .peers h1 {
        font-size: 2.5rem;
        margin-bottom: 1rem;
        color: #333;
    }
    .peers p {
        font-size: 1.2rem;
        margin-bottom: 2rem;
        color: #666;
    }
    .peers table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 1rem;
    }
    .peers td {
        padding: 1rem;
        text-align: left;
        border: 1px solid #ddd;
        background-color: #f9f9f9;
    }
    .peers tr:first-child td {
        background-color: #f2f2f2;
        font-weight: bold;
    }
    "#.to_string()
}
