use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::*;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

static ADJECTIVES: &[&str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

#[component]
fn Button<'a>(cx: Scope, id: &'a str, text: &'a str) -> Element {
    view! {
        <div class="col-sm-6 smallpad">
            <button id={id} class="btn btn-primary btn-block" type="button">{text}</button>
        </div>
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
struct RowData {
    id: usize,
    label: (ReadSignal<String>, WriteSignal<String>),
}

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn build_data(cx: Scope, count: usize) -> Vec<RowData> {
    let mut thread_rng = thread_rng();

    let mut data = Vec::new();
    data.reserve_exact(count);

    for _i in 0..count {
        let adjective = ADJECTIVES.choose(&mut thread_rng).unwrap();
        let colour = COLOURS.choose(&mut thread_rng).unwrap();
        let noun = NOUNS.choose(&mut thread_rng).unwrap();
        let capacity = adjective.len() + colour.len() + noun.len() + 2;
        let mut label = String::with_capacity(capacity);
        label.push_str(adjective);
        label.push(' ');
        label.push_str(colour);
        label.push(' ');
        label.push_str(noun);

        data.push(RowData {
            id: ID_COUNTER.load(Ordering::Relaxed),
            label: create_signal(cx, label),
        });

        ID_COUNTER.store(ID_COUNTER.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
    }

    data
}

#[component]
fn App(cx: Scope) -> Element {
    let (data, set_data) = create_signal(cx, Vec::<RowData>::new());
    let (selected, set_selected) = create_signal(cx, None::<usize>);

    let remove = move |id| {
        set_data(move |data| data.retain(|row| row.id != id));
    };

    let run = move |_| {
        set_data(move |n| *n = build_data(cx, 1000));
        set_selected(|n| *n = None);
    };

    let run_lots = move |_| {
        set_data(move |n| *n = build_data(cx, 10000));
        set_selected(|n| *n = None);
    };

    let add = move |_| {
        set_data(move |data| data.append(&mut build_data(cx, 1000)));
    };

    let update = move |_| {
        set_data(|data| {
            for row in data.iter_mut().step_by(10) {
                row.label.1.update(|n| *n = format!("{} !!!", n));
            }
        });
    };

    let clear = move |_| {
        set_data(|n| *n = Vec::new());
        set_selected(|n| *n = None);
    };

    let swap_rows = move |_| {
        set_data(|data| {
            if data.len() > 998 {
                data.swap(1, 998);
            }
        });
    };

    view! {
        <div class="container">
            <div class="jumbotron"><div class="row">
            <div class="col-md-6"><h1>"Leptos"</h1></div>
            <div class="col-md-6"><div class="row">
                <Button id="run" text="Create 1,000 rows" on:click=run/>
                <Button id="runlots" text="Create 10,000 rows" on:click=run_lots />
                <Button id="add" text="Append 1,000 rows" on:click=add />
                <Button id="update" text="Update every 10th row" on:click=update />
                <Button id="clear" text="Clear" on:click=clear />
                <Button id="swaprows" text="Swap Rows" on:click=swap_rows />
            </div></div>
            </div></div>
            <table class="table table-hover table-striped test-data">
                <tbody>
                    <For each={ data } key={|row| row.id}>{move |cx, row: &RowData| {
                        let row_id = row.id;
                        let (label, set_label) = row.label;
                        view! {
                            <tr prop:className={move || if selected() == Some(row_id) { Some("danger") } else { None }}>
                                <td class="col-md-1">{row_id.to_string()}</td>
                                <td class="col-md-4"><a on:click=move |_| set_selected(move |n| *n = Some(row_id))>{move || label.get()}</a></td>
                                <td class="col-md-1"><a on:click=move |_| remove(row_id)><span class="glyphicon glyphicon-remove" aria-hidden="true"></span></a></td>
                                <td class="col-md-6"/>
                            </tr>
                        }
                    }}</For>
                </tbody>
            </table>
            <span class="preloadicon glyphicon glyphicon-remove" aria-hidden="true" />
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let mount_el = document().query_selector("#main").unwrap().unwrap();
    leptos::mount(mount_el.unchecked_into(), |cx| {
        view! { <App/> }
    });
}
