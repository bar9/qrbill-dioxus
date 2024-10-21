#![allow(non_snake_case)]

extern crate core;

mod layout;

use crate::layout::ThemeLayout;
use chrono::{Date, DateTime, Local, NaiveDate, TimeZone, Utc};
use dioxus::prelude::*;
use qrbill::{Address, Iban, QRBill, QRBillOptions, Reference, StructuredAddress};
use std::fmt::Write;

fn main() {
    dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
    // let displayed_data = use_ref(cx, || HashMap::<String, ShoppingListItem>::new());
    let svg = use_state(cx, || String::new());
    let qr_string = use_state(cx, || String::new());
    let name = use_state(cx, || String::from("Roland Brand"));
    let amount = use_state(cx, || None::<f64>);
    let lang = use_state(cx, || qrbill::Language::English);
    let currency = use_state(cx, || qrbill::Currency::SwissFranc);
    let date = use_state(cx, || Some(Utc::today()));

    //
    use_effect(cx, (name, amount, lang, currency), |_| {
        let qrbill = QRBill::new(QRBillOptions {
            account: String::from("CH0409000000303748105")
                .parse::<Iban>()
                .unwrap(),
            creditor: Address::Structured(StructuredAddress {
                name: name.to_string(),
                street: "Tellstrasse".to_string(),
                house_number: "66".to_string(),
                postal_code: "4053".to_string(),
                city: "Basel".to_string(),
                country: isocountry::CountryCode::CHE,
            }),
            amount: *amount.get(), //Some(42.0),
            currency: *currency.get(),
            due_date: *date.get(),
            // due_date: None,
            debtor: None,
            reference: Reference::None,
            extra_infos: None,
            alternative_processes: vec![],
            language: *lang.get(),
            top_line: true,
            payment_line: true,
        })
        .unwrap();

        let rendered_svg = qrbill.create_svg(false).unwrap_or(String::new());
        let rendered_string = qrbill.qr_data();

        to_owned![svg, qr_string];
        async move {
            svg.set(rendered_svg);
            qr_string.set(rendered_string);
        }
    });

    render! {
        ThemeLayout{
            h1 { class: "text-4xl text-center p-8",
                "QR Bill Playground"
            }
            div {
                class: "grid grid-flow-col gap-2",
                form {
                    class: "col-span-1 flex flex-col gap-2",
                    label { class: "input input-bordered flex items-center gap-2",
                        "Language:"
                        select {
                            class: "select-bordered select-sm w-full max-w-xs",
                            oninput: move |evt| match evt.value.as_ref() {
                                "en" => lang.set(qrbill::Language::English),
                                "de" => lang.set(qrbill::Language::German),
                                "fr" => lang.set(qrbill::Language::French),
                                _ => lang.set(qrbill::Language::English)
                            },
                            option { value: "en", "English" }
                            option { value: "de", "Deutsch" }
                            option { value: "fr", "Français" }
                        }
                    }
                    label { class: "input input-bordered flex items-center gap-2",
                        "Currency:"
                        select {
                            class: "select-bordered select-sm w-full max-w-xs",
                            oninput: move |evt| match evt.value.as_ref() {
                                "CHF" => currency.set(qrbill::Currency::SwissFranc),
                                "EUR" => currency.set(qrbill::Currency::Euro),
                                _ => currency.set(qrbill::Currency::SwissFranc)
                            },
                            option { value: "CHF", "CHF" }
                            option { value: "EUR", "EUR" }
                        }
                    }
                    label { class: "input input-bordered flex items-center gap-2",
                        "Name:"
                        input {
                            r#"type"#: "text",
                            class: "grow",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value.clone()),
                        }
                    }
                    label { class: "input input-bordered flex items-center gap-2",
                        "Amount:"
                        input {
                            r#"type"#: "number",
                            class: "grow",
                            value: "{amount.unwrap_or_default()}",
                            oninput: move |evt| amount.set(evt.value.clone().parse().ok()),
                        }
                    }
                    // label { class: "input input-bordered flex items-center gap-2",
                    //     "Due date:"
                    //     input {
                    //         r#"type"#: "date",
                    //         class: "grow",
                    //         value: "{date.unwrap()}",
                    //
                    //         oninput: move |evt| date.set(
                    //             Utc::from_utc_date(
                    //                 NaiveDate::parse_from_str(&evt.value, "%Y-%m-%d").
                    //             ).ok()),
                    //     }
                    // }
                }
                div {
                    class: "col-span-1",
                    div { class: "mockup-code text-xs",
                        for line in qr_string.lines().filter(|line| !line.is_empty()) {
                            pre { r#"data-prefix"#: ">",
                                code {
                                    "{line}"
                                }
                            }
                        }
                    }
                    div { class: "mt-3 rounded",
                        dangerous_inner_html: "{svg}"
                    }
                }
            }
        }
    }
}
