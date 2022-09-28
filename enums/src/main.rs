enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x:i64, y:i64},
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let quit = WebEvent::KeyPress('q');
    let paste = WebEvent::Paste(String::from("hello"));
    let load = WebEvent::PageLoad;

    handle(quit);
    handle(paste);
    handle(load);

    // let something = Some(1);
}

fn handle(event : WebEvent) {
    match event {
        WebEvent::PageLoad => {
            println!("page loaded");
        },
        WebEvent::PageUnload => {
            println!("page unloaded");
        },
        WebEvent::KeyPress(pressed) => {
            println!("char pressed {}", pressed);
        },
        WebEvent::Paste(pasted) => {
            println!("pasted value {}", pasted);
        },
        WebEvent::Click { x, y } => {
            println!("pasted value {} {}", x, y);
        },
    }

}
