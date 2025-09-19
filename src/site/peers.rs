use yew::prelude::*;
use crate::site::header::Header;

#[function_component(Peers)]
pub fn peers() -> Html {
    html! {
    <>
        <Header />
        <div class="peers">
            <style>
                { peers_style() }
            </style>
            <h1>{ "Amateur IX Peers" }</h1>
            <table class="peers-table">
                <thead>
                    <tr>
                        <th>{ "Peer Name" }</th>
                        <th>{ "AS" }</th>
                        <th>{ "IPv4 Address" }</th>
                        <th>{ "IPv6 Address" }</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{ "Route Server 1" }</td>
                        <td>{ "AS40064" }</td>
                        <td>{ "149.112.177.1/24" }</td>
                        <td>{ "2602:fa86:fff:ffff::1/64" }</td>
                    </tr>
                    <tr>
                        <td>{ "Route Server 2" }</td>
                        <td>{ "AS40064" }</td>
                        <td>{ "149.112.177.2/24" }</td>
                        <td>{ "2602:fa86:fff:ffff::2/64" }</td>
                    </tr>
                    <tr>
                        <td>{ "Austin Hadley" }</td>
                        <td>{ "AS17290" }</td>
                        <td>{ "149.112.177.3/24" }</td>
                        <td>{ "2602:fa86:fff:ffff::3/64" }</td>
                    </tr>
                    <tr>
                        <td>{ "Bastiaan Brink" }</td>
                        <td>{ "AS215248" }</td>
                        <td>{ "149.112.177.4/24" }</td>
                        <td>{ "2602:fa86:fff:ffff::4/64" }</td>
                    </tr>
                    <tr>
                        <td>{ "Ashland Area Amateur Radio Club" }</td>
                        <td>{ "AS40419" }</td>
                        <td>{ "149.112.177.5/24" }</td>
                        <td>{ "2602:fa86:fff:ffff::5/64" }</td>
                    </tr>
                    <tr>
                        <td>{ "Raoul Brounds" }</td>
                        <td>{ "AS215296" }</td>
                        <td>{ "149.112.177.6/24" }</td>
                        <td>{ "2602:fa86:fff:ffff::6/64" }</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </>
    }
}

fn peers_style() -> String {
    r#"
        .peers-table {
            border-collapse: collapse;
            width: 100%;
            /* Only top and bottom outer borders */
            border-top: 2px solid #ccc;
            border-bottom: 2px solid #ccc;
            /* Center the text */
            text-align: center;
        }
        /* Remove any cell borders so there are no side lines */
        .peers-table td {
            border: none;
            padding: 8px 12px;
        }
        /* Alternate row backgrounds */
        .peers-table tr:nth-child(odd) td {
            background: #f2f2f2;
        }
        .peers-table tr:nth-child(even) td {
            background: #ffffff;
        }
    "#.to_string()
}
