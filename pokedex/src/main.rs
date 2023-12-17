use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
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

fn main() {
    yew::Renderer::<App>::new().render();
}

