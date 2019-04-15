use wasm_bindgen::prelude::*;
mod hamiltonian;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();
    let window = web_sys::window().expect("should have a Window"); 
    let document = window.document().expect("should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    // p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;

    Ok(())    
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Called by js
#[wasm_bindgen]
pub fn search_hamiltonian(dimension: i32, matrix: String) -> Vec<i32>{
    // let mut graph = parse_matrix(matrix, dimension);
    
    let mut graph = create_matrix(matrix, dimension);
    
    let result = hamiltonian::run(&mut graph);
    

    return result
}

pub fn create_matrix(edges: String, vertices: i32 ) -> Vec<Vec<bool>> {
    let mut graph: Vec<Vec<bool>> = vec![vec![]; vertices as usize];

    for i in 0..vertices {
        for j in 0..vertices {
            graph[i as usize].push(false);   
        }
    }

    for c in edges.split("\n") {
        let nodes = c.split_whitespace().collect::<Vec<&str>>();
        let a = nodes[0].parse::<usize>().unwrap();;
        let b = nodes[1].parse::<usize>().unwrap();;
        graph[a][b] = true;
        graph[b][a] = true;
    }

    
    return graph;
}

// Used to parse a matrix in string format to vector of vector of bool
pub fn parse_matrix(matrix: String, dimension: i32) -> Vec<Vec<bool>> {
    
    // Instantiate adjacency matrix
    let mut graph: Vec<Vec<bool>> = vec![vec![]; dimension as usize];

    let mut row = 0;
    let mut col = 0;
    // Traverse each character previously splitted by whitespace
    for i in matrix.split_whitespace() {

        // Parse to boolean
        let is_connected = if i == "1" { true } else { false };
        graph[row].push(is_connected);

        if col + 1 == dimension as usize {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    return graph;
}

// "Import" alert function of js to debug code
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
