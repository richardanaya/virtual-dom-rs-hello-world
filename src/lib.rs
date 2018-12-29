#[macro_use]
extern crate virtual_dom_rs;
use wasm_bindgen::prelude::*;
use virtual_dom_rs::VirtualNode;
use virtual_dom_rs::JsCast;
use web_sys::Element;

// Holds a global counter to show something changing in UI
static mut COUNT:i32 = 0;

// HelloWorld component just sys hello along with a global counter value
struct HelloWorld {}

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

// This object will help us store the previous dom and render to a target element
struct VirtualDomRenderer {
    root_element: Element,
    previous_vdom: Option<VirtualNode>,
}

impl VirtualDomRenderer {
    fn new(root_element:Element) -> VirtualDomRenderer {
        VirtualDomRenderer {
            root_element:root_element,
            previous_vdom:None
        }
    }

    fn render(&mut self, new_vdom:&mut VirtualNode) {
        if let Some(p_vd) = &self.previous_vdom {
            // If its not the first time, calculate the DOM dif and apply to root dom contents
            let patches = virtual_dom_rs::diff(&p_vd, new_vdom);
            virtual_dom_rs::patch(self.root_element.clone(), &patches);
        }  else {
            // If its the first time just set the contents of the DOM to string
            self.root_element.set_inner_html(&new_vdom.to_string());
        }
    }
}


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Let's first get the body since this is going to be our root node
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = virtual_dom_rs::Element::from(document.body().unwrap());

    // This is my root component
    let hello_world = HelloWorld::new();

    // create our renderer
    let mut renderer = VirtualDomRenderer::new(body);
    renderer.render(&mut hello_world.render());

    // Now we are going to increment the counter and render every second
    // So, this looks really complicated, but basically we are just
    // 1. creating a complicated reference of a Closure
    // 2. saying we want it called ever 1000 ms (1 second)
    // 3. then forgetting the closure so that it doesn't get dropped and cause an error the next timer
    let a = Closure::wrap(Box::new(move || {
        // increment our counter
        unsafe {
            COUNT += 1;
        }
        // render it again, this time letting virtual dom update things because we'll have previous dom
        renderer.render(&mut hello_world.render());
    }) as Box<dyn FnMut()>);
    window.set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
    a.forget();

    Ok(())
}
