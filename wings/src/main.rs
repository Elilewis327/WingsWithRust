use glfw::{Key, Action, *};
use core::panic;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::sync::mpsc::Receiver;


const WINDOWHEIGHT: u32 = 1080;
const WINDOWWIDTH: u32 = 1920;
static CONFIG_PATH: &str = "config.txt";

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(WINDOWWIDTH, WINDOWHEIGHT,
        "hello world", glfw::WindowMode::Windowed).expect("Failed to create glfw window");
    
    let config: String = load_config_file();

    window.set_key_polling(true);
    window.make_current();

    event_loop(&mut glfw, &mut window, events);

}

fn event_loop(mut glfw: &mut glfw::Glfw, mut window: &mut glfw::Window, events: Receiver<(f64, glfw::WindowEvent)>){
    loop {
        if window.should_close(){
            break;
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events){
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent){
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}    


fn generate_new_config(error: std::io::Error) -> File{
    match error.kind() {
        ErrorKind::NotFound => match File::create(CONFIG_PATH) {
            Ok(file) => {
                            print!("Failed to open file {} for read \n", CONFIG_PATH);
                            print!("Created new config file {} \n", CONFIG_PATH);
                            file
                        },
            Err(e) => panic!("Problem creating new file {CONFIG_PATH} {:?}", e),
        }
        error => {
            panic!("Failed to open file {} for read: {:?}", CONFIG_PATH, error);
        }
    }   

}

fn load_config_file() -> String{
    let mut file = match File::open(CONFIG_PATH) {
        Ok(file) => file,
        Err(error) => generate_new_config(error),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("Failed to read file {}", CONFIG_PATH).as_str());

   contents
}