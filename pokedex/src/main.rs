use std::ops::Add;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use serde_json::Value;

#[function_component(App)]
fn app() -> Html {
    let pokemon_state = use_state_eq::<Option<Pokemon>, _>(|| None);
    let search_state = use_state_eq::<i32, _>(|| 1.max(1));
    let _search_state = search_state.clone();
    let search_state_outer = search_state.clone();
    let _pokemon_state_outer = pokemon_state.clone();
    let pokemon_state_outer = pokemon_state.clone();
    let onclick_prev: Callback<MouseEvent> = Callback::from(move |mouse_event| {
        on_click(search_state_outer.clone(), pokemon_state_outer.clone(), _search_state.clone(), -1, mouse_event);
    });
    let _search_state = search_state.clone();
    let search_state_outer = search_state.clone();
    let pokemon_state_outer = pokemon_state.clone();
    let onclick_next: Callback<MouseEvent> = Callback::from(move |mouse_event| {
        on_click(search_state_outer.clone(), pokemon_state_outer.clone(), _search_state.clone(), 1, mouse_event);
    });
    let _search_state = search_state.clone();
    let onkeydown: Callback<KeyboardEvent> = Callback::from(move |keyboard_event: KeyboardEvent| {
        if keyboard_event.key() == "Enter" {
            keyboard_event.prevent_default();
            let input: HtmlInputElement = keyboard_event.target_unchecked_into();
            let value = input.value();
            if let Ok(number) = value.parse::<i32>() {
                _search_state.set(number);
            }
        }
    });

    html! {
        <main>
            <img src={format!("{}", pokemon_state.as_ref().map_or("", |p| &p.image_src))} alt="pokemon" class="poke__image" />

            <h1 class="pokemon__data">
                <span class="pokemon__number">{format!("{:?} - ", search_state.to_string().parse::<i32>().unwrap())}</span>
                <span class="pokemon__name">{format!("{}", pokemon_state.as_ref().map_or("", |p| &p.name))}</span>
            </h1>

            <form class="form">
                <input
                    type="search"
                    class="input__search"
                    placeholder="Name or number"
                    onkeydown={onkeydown}
                />
            </form>

            <div class="buttons">
                <button class="button btn-prev" onclick={onclick_prev}>{"Prev <"}</button>
                <button class="button btn-next" onclick={onclick_next}>{"Next >"}</button>
            </div>
            <img src="./images/pokedex.png" alt="pokedex" class="pokedex" />
        </main>
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pokemon {
    id: i32,
    name: String,
    image_src: String,
}

fn on_click(
    search_state_outer: UseStateHandle<i32>,
    pokemon_state: UseStateHandle<Option<Pokemon>>,
    search_state: UseStateHandle<i32>,
    increment: i32,
    _mouse_event: MouseEvent,
) {
    search_state.set(search_state_outer.add(increment));

    let pokemon_state = pokemon_state.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", search_state_outer.to_string());
        let response = reqwest::get(url).await.unwrap();

        if response.status().is_success() {
            let text = response.text().await.unwrap();
            let v: Value = serde_json::from_str(&text).unwrap();
            let name = v["name"].as_str().unwrap();
            let id = v["id"].as_i64().unwrap() as i32;
            let image_src = v["sprites"]["versions"]["generation-v"]["black-white"]["animated"]["front_default"]
                .as_str()
                .unwrap();

            let pokemon = Pokemon {
                id,
                name: name.into(),
                image_src: image_src.into(),
            };
            pokemon_state.set(Some(pokemon));
        } else {
            pokemon_state.set(None);
        }
    });
}

fn main() {
    yew::Renderer::<App>::new().render();
}
