use std::ops::Add;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use serde_json::Value;

#[function_component(App)]
fn app() -> Html {
    let pokemon_state = use_state_eq::<Option<Pokemon>, _>(|| None);
    let search_state = use_state_eq::<i32, _>(|| 1);

    let search_state_outer = search_state.clone();
    let pokemon_state_outer = pokemon_state.clone();

    let onclick: Callback<MouseEvent> = Callback::from(move |_mouse_event: MouseEvent| {
        search_state.set(search_state_outer.add(1));

        let search_state = search_state.clone();
        let pokemon_state = pokemon_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let url = format!("https://pokeapi.co/api/v2/pokemon/{:?}", search_state_outer);
            let response = reqwest::get(url).await.unwrap();

            if response.status().is_success() {
                let text = response.text().await.unwrap();
                let v: Value = serde_json::from_str(&text).unwrap();
                let name = v["name"].as_str().unwrap();
                let id = v["id"].as_i64().unwrap() as i32;
                let image_src = v["sprites"]["versions"]["generation-v"]["black-white"]["animated"]["front_default"]
                .as_str()
                .unwrap();

                let pokemon = Pokemon  {
                    id,
                    name: name.into(),
                    image_src: image_src.into(),
                };
                pokemon_state.set(Some(pokemon));
            } else {
                pokemon_state.set(None);
            }
        });
    });
    html! {
        <main>
            <img src="" alt="pokemon" class="poke__image" />

            <h1 class="pokemon__data">  
                <span class="pokemon__number">{"6 - "}</span> 
                <span class="pokemon__name">{"chalizard"}</span>
            </h1>

            <form class="form">
                <input  
                    type="search"
                    class="input__search"
                    placeholder="Name or number"
                />
            </form>

            <div class="buttons">
                <button class="button btn-prev">{"Prev <"}</button>
                <button class="button btn-next">{"Next >"}</button>
            </div>
            <img src="./images/pokedex.png" alt="pokedex" class="pokedex" />
        </main>
    }
}

#[derive(PartialEq, Clone)]
struct Pokemon {
    id: i32, 
    name: String,
    image_src: String,
}



fn main() {
    yew::Renderer::<App>::new().render();
}

