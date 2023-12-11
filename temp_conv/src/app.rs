use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Window>{
            show_bg: true
            width: Fill
            height: Fill
            draw_bg: {
                shape: Solid
                fn pixel(self) -> vec4 {
                    return mix(#7, #3, self.pos.y);
                }
            }

            body = <View> {
                flow: Right,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.5
                }

                input_celsius = <TextInput> {
                    width: 80, height: 40,
                    draw_bg: {
                        color: #121212
                    }
                    text: "Input"
                }

                label_celsius = <Label> {
                    width: 60, height: 40 
                    align: {
                        x: 0.5,
                        y: 0.5
                    }
                    draw_text: {
                        color: #0f0
                    },
                    text: "Celsius = "
                }

                input_fahrenheit = <TextInput> {
                    width: 80, height: 40,
                    draw_bg: {
                        color: #121212
                    }
                    text: "Input"
                }

                label_fahrenheit = <Label> {
                    width: 70, height: 40 
                    align: {
                        x: 0.5,
                        y: 0.5
                    }
                    draw_text: {
                        color: #0ff
                    },
                    text: "Fahrenheit"
                }
            }
        }
    }
}

// This main_app macro generates the code necessary to initialize and run your application.
//
// This code is almost always the same between different applications, so it is convenient to use a
// macro for it. The two main tasks that this code needs to carry out are: initializing both the
// main application struct (`App`) and the global context object (`Cx`), and setting up event
// handling. On desktop, this means creating and running our own event loop from a fn main(). On web, this means
// creating an event handler function that the browser event loop can call into.
app_main!(App);

// The main application struct.
//
// The #[derive(Live, LiveHook)] attribute implements a bunch of traits for this struct that enable
// it to interact with the Makepad runtime. Among other things, this enables the Makepad runtime to
// initialize the struct from a DSL object.
#[derive(Live, LiveHook)]

pub struct App {
    // A frame widget. Used to contain our button and label.
    #[live] ui: WidgetRef,

    // The value for our counter.
    //
    // The #[rust] attribute here is used to indicate that this field should *not* be initialized
    // from a DSL object, even when a corresponding property exists.
    #[rust] c_value: String,
    #[rust] f_value: String,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    } 
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx:&mut Cx, actions:&Actions){
        let res = self.ui.text_input(id!(input_celsius)).changed(&actions);
        match res {
            Some(s) => match s.parse::<i32>() {
                Ok(number) => {
                    self.f_value = (number * 9/5 + 32).to_string();
                    println!("F={}", self.f_value);
                    let inp_f = self.ui.text_input(id!(input_fahrenheit));
                    inp_f.set_text_and_redraw(cx, &format!("{}", self.f_value));
                }
                Err(e) => {
                    println!("Invalid input. Please enter an integer. {}", e);
                }
            },
            None => {
                // println!("No input to parse")
            }
        }
        let res = self.ui.text_input(id!(input_fahrenheit)).changed(&actions);
        match res {
            Some(s) => match s.parse::<i32>() {
                Ok(number) => {
                    self.c_value = ((number - 32) * 5/9).to_string();
                    println!("C={}",  self.c_value);
                    let inp_c = self.ui.text_input(id!(input_celsius));
                    inp_c.set_text_and_redraw(cx, &format!("{}", self.c_value));
                }
                Err(e) => {
                    println!("Invalid input. Please enter an integer. {}", e);
                }
            },
            None => {
                // println!("No input to parse")
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
