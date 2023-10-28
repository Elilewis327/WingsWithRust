use glfw::{Key, Action, *};
use gl;
use core::panic;
use std::fs::File;
use std::io::{ErrorKind, Read, Write, stdout};
use std::sync::mpsc::Receiver;

static CONFIG_PATH: &str = "config.txt";
const DEBUG: bool = true;
type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] =
  [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

fn main() {
    let config: Config = load_config_file();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(config.hoizontal_resolution, config.veritcal_resolution,
        "hello world", glfw::WindowMode::Windowed).expect("Failed to create glfw window");
    
    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));
    
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            core::mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
          );
    }


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

        unsafe {
            gl::Clear(1);
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
        ErrorKind::NotFound => match File::options().write(true).create(true).open(CONFIG_PATH) {
            Ok(mut file) => {
                            print!("Failed to open file {} for read \n", CONFIG_PATH);
                            print!("Created new config file {} \n", CONFIG_PATH);
                            file.write_all(b"vertical_resolution=1080\nhorizontal_resolution=1920\nfps_max=60").expect("Failed to write to new config file.");
                            File::open(CONFIG_PATH).expect("Error opening newly created config file for write. Try starting the program again.")
                        },
            Err(e) => panic!("Problem creating new file {CONFIG_PATH} {:?}", e),
        }
        error => {
            panic!("Failed to open file {} for read: {:?}", CONFIG_PATH, error);
        }
    }   

}

fn load_config_file() -> Config{
    let mut file = match File::open(CONFIG_PATH) {
        Ok(file) => file,
        Err(error) => generate_new_config(error),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("Failed to read file {}", CONFIG_PATH).as_str());

    let lines = contents.split("\n");

    let mut config = Config {
        hoizontal_resolution: 10,
        veritcal_resolution: 10,
        fps_max: 0,
        clown_cakes: 0,
    };   

    let mut line_number : u16 = 1;
    for line in lines{
        let error = format!("Config file error, please make sure your config file is properly formatted. \n The proper format is: \n \t\tkey=value\n\t\tkey=value\n\t\tkey=value\n\n\n and so on. Error occurred on line {line_number}");

        let left = line.split("=").nth(0).expect(error.as_str());
        let right = line.split("=").last().expect(error.as_str());

        debug(format!("{left} = {right}\n"));
        
        match left{
            "horizontal_resolution" => config.hoizontal_resolution = right.parse::<u32>().expect(error.as_str()),
            "vertical_resolution" => config.veritcal_resolution = right.parse::<u32>().expect(error.as_str()),
            "clown_cakes" => config.clown_cakes = right.parse::<u32>().expect(error.as_str()),
            "fps_max" => config.fps_max = right.parse::<u32>().expect(error.as_str()),
            _ => {print!("Unkown config \"{line}\" on line {line_number} of the config file\n"); stdout().flush().unwrap();}

        };

        line_number += 1;
    }
   
   config
}

fn debug(message: String){
    if !DEBUG {return;}
    print!("{}", message);
    stdout().flush().unwrap();
}

struct Config {
    hoizontal_resolution: u32,
    veritcal_resolution: u32,
    fps_max: u32,
    clown_cakes: u32,

}