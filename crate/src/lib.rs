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

// Called by js
#[wasm_bindgen]
pub fn search_hamiltonian(matrix: String, dimension: i32) -> Vec<i32>{
    let mut graph = parse_matrix(matrix, dimension);
    
    return hamiltonian::run(&mut graph);
}

// Used to parse a matrix in string format to vector of vector of bool
pub fn parse_matrix(matrix: String, dimension: i32) -> Vec<Vec<bool>> {
    
    // Instantiate adjacency matrix
    let mut graph: Vec<Vec<bool>> = vec![];

    // Fill with empty vectors
    for row in 0..dimension {
        graph.push(vec![]);
    }

    let mut row = 0;
    let mut col = 0;
    // Traverse each character previously splitted by whitespace
    for i in matrix.split_whitespace() {

        // Parse to boolean
        let is_connected = if (i == "1") { true } else { false };
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
