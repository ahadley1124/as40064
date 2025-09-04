use yew::prelude::*;
use crate::site::header::Header;

#[function_component(Pricing)]
pub fn pricing() -> Html {
    html! {
    <>
        <div class="pricing-container">
            <Header />
        </div>
        <div class="pricing">
            <h1>{ "Pricing" }</h1>
            <p>{ "This is the pricing page of our Yew application." }</p>
            <table>
                <thead>
                    <tr>
                        <th>{ "Plan" }</th>
                        <th>{ "Price" }</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{ "VM with 100Mb/s connection" }</td>
                        <td>{ "$5/month" }</td>
                    </tr>
                    <tr>
                        <td>{ "VM with 1Gb/s connection" }</td>
                        <td>{ "$10/month" }</td>
                    </tr>
                    <tr>
                        <td>{ "Colo with 100Mb/s connection" }</td>
                        <td>{ "$60/month" }</td>
                    </tr>
                    <tr>
                        <td>{ "Colo with 1Gb/s connection" }</td>
                        <td>{ "$100/month" }</td>
                    </tr>
                    <tr>
                        <td>{ "Colo with Connection to Ohio-IX" }</td>
                        <td>{ "Contact us for pricing" }</td>
                    </tr>
                </tbody>
            </table>
        </div>
        <style>
            { pricing_style() }
        </style>
    </>
    }
}

fn pricing_style() -> String {
    r#"
    .pricing-container {
        padding: 0;
        width: 100%;
        margin: 0 auto;
    }
    .pricing h1 {
        font-size: 2.5rem;
        margin-bottom: 1rem;
    }
    .pricing p {
        font-size: 1.2rem;
        margin-bottom: 2rem;
    }
    .pricing table {
        width: 100%;
        border-collapse: collapse;
    }
    .pricing th, .pricing td {
        padding: 1rem;
        text-align: left;
        border-bottom: 1px solid #eee;
    }
    .pricing th {
        background-color: #f2f2f2;
    }
    "#.to_string()
}