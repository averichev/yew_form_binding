use validator::{Validate, ValidationError};
use validator_derive::Validate;
use wasm_bindgen::prelude::*;
use yew::{html, Component, Html, Context, MouseEvent};
use yew_form_derive::Model;

use yew_form::{CheckBox, Field, Form};

fn must_be_true(value: &bool) -> Result<(), ValidationError> {
    if value == &true {
        Ok(())
    } else {
        Err(ValidationError::new("Must accept terms before continuing"))
    }
}

#[derive(Model, Validate, PartialEq, Clone)]
struct Address {
    #[validate(length(min = 1, message = "Street is required"))]
    street: String,
    #[validate(length(min = 1, message = "City name is required"))]
    city: String,
    province: String,
    postal_code: String,
    country: String,
}

#[derive(Model, Validate, PartialEq, Clone)]
struct Registration {
    #[validate(length(min = 1, message = "First name is required"))]
    first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    last_name: String,
    quantity: u32,
    price: f64,
    #[validate]
    address: Address,
    #[validate(custom = "must_be_true")]
    accept_terms: bool,
}

impl Registration {
    pub fn total(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}

enum AppMessage {
    Update,
    Submit,
}

struct App {
    form: Form<Registration>,
    submitted: bool,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let model = Registration {
            first_name: String::from("J-F"),
            last_name: String::from("Bilodeau"),
            quantity: 10,
            price: 5.99,
            address: Address {
                street: String::new(),
                city: String::from("Ottawa"),
                province: String::from("ONT"),
                postal_code: String::from("K2P 0A4"),
                country: String::new(),
            },
            accept_terms: false,
        };

        Self {
            form: Form::new(model),
            submitted: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Update => true, // Force update
            AppMessage::Submit => {
                if self.form.validate() {
                    self.submitted = true;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let cb = link.callback(|_:String| {
            AppMessage::Update
        });
        let form = &self.form;
        let onclick = ctx.link().callback(|e: MouseEvent| {e.prevent_default(); AppMessage::Submit});
        html! {
            <div class="container-sm">
                <h1>{"Yew Form Example"}</h1>
                <p>{format!("Hello, {} {} and welcome to Yew Form!",
                        self.form.field_value("first_name"),
                        self.form.field_value("last_name"))}</p>
                <form>
                    // TODO: support additional attributes
                    // TODO: Update form without needing oninput
                    <div class="form-group">
                        <label for="first_name">{"First Name: "}</label>
                        <Field<Registration>
                                form={form}
                                autocomplete={"given_name"}
                                field_name={"first_name"}
                                class={"form-control blue foo bar"}
                                class_invalid={"is-invalid very-wrong"}
                                class_valid={"is-valid green"}
                                oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("first_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Last Name: "}</label>
                        <Field<Registration> form={form} field_name={"last_name"} oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("last_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Quantity: "}</label>
                        <Field<Registration> form={form} field_name={"quantity"} oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("quantity")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Price: "}</label>
                        <Field<Registration> form={form} field_name={"price"} oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("price")}
                        </div>
                    </div>
                    <div>
                        {"Total: "}{format!("{:.2}", form.model().total())}
                    </div>
                    <div class="form-group">
                        <label for="address.street">{"Street: "}</label>
                        <Field<Registration> form={form} field_name={"address.street"} oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("address.street")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.city">{"City: "}</label>
                        <Field<Registration> form={form} field_name={"address.city"} oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("address.city")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.province">{"Province: "}</label>
                        <Field<Registration> form={form} field_name={"address.province"} oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("address.province")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.country">{"Country (optional): "}</label>
                        <Field<Registration> form={form} field_name={"address.country"} oninput={cb.clone()} />
                        <div class="invalid-feedback">
                            {form.field_message("address.country")}
                        </div>
                    </div>
                    <div class="form-group">
                        <CheckBox<Registration>
                            field_name={"accept_terms"}
                            form={form}
                        />
                        <label class="form-check-label" for="accept_terms">{"Accept Terms and Conditions: "}</label>
                        <div class="invalid-feedback">
                          {form.field_message("accept_terms")}
                        </div>
                    </div>
                    <div class="form-group">
                        <button type="button" {onclick}>{"Submit"}</button>
                    </div>
                </form>
                <div hidden={!self.submitted}>
                    <h2>{"Form data"}</h2>
                    <p>{"First Name: "}{&form.model().first_name}</p>
                    <p>{"Last Name: "}{&form.model().last_name}</p>
                    <p>{"Street: "}{&form.model().address.street}</p>
                    <p>{"City: "}{&form.model().address.city}</p>
                    <p>{"Province: "}{&form.model().address.province}</p>
                    <p>{"Country: "}{&form.model().address.country}</p>
                    <p>{"Accepted Terms: "}{form.model().accept_terms}</p>
                </div>
            </div>
        }
    }
}

pub fn main() {
    yew::Renderer::<App>::new().render();
}