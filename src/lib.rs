#[macro_use]
extern crate virtual_dom_rs;
use wasm_bindgen::prelude::*;
use virtual_dom_rs::VirtualNode;
use virtual_dom_rs::JsCast;

// HelloWorld component just sys hello along with a global counter value
struct HelloWorld {}

static mut COUNT:i32 = 0;

impl HelloWorld {
    fn new() ->  HelloWorld {
        HelloWorld{}
    }

    fn render(&self) -> VirtualNode {
        // it's unsafe to even read global statics
        let c = unsafe {
            COUNT
        };

        html! {
            <div>
                { format!("Hello World!! {}",c)}
            </div>
        }
    }
}


pub fn render_to_dom(root_element:web_sys::Element,previous_vdom:&Option<VirtualNode>,new_vdom:&mut VirtualNode) {
    if let Some(p_vd) = previous_vdom {
        // If its not the first time, calculate the DOM dif and apply to root dom contents
        let patches = virtual_dom_rs::diff(&p_vd, new_vdom);
        virtual_dom_rs::patch(root_element, &patches);
    }  else {
        // If its the first time just set the contents of the DOM to string
        root_element.set_inner_html(&new_vdom.to_string());
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Let's first get the body since this is going to be our root node
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let hello_world = HelloWorld::new();

    // Originally we have no DOM and count is 0, let's do our first render
    let previous_vdom = None;
    let mut new_vdom = hello_world.render();
    render_to_dom(virtual_dom_rs::Element::from(body), &previous_vdom,&mut new_vdom);

    // Now we are going to increment the counter and render every second
    // So, this looks really complicated, but basically we are just
    // 1. creating a complicated reference of a Closure
    // 2. saying we want it called ever 1000 ms (1 second)
    // 3. then forgetting it so that it doesn't get dropped and cause an error the next timer
    let a = Closure::wrap(Box::new(move || {
        unsafe {
            COUNT += 1;
        }
        let mut new_vdom = hello_world.render();
        let body = document.body().unwrap();
        render_to_dom(virtual_dom_rs::Element::from(body), &previous_vdom,&mut new_vdom);
    }) as Box<dyn Fn()>);
    window.set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
    a.forget();

    Ok(())
}
