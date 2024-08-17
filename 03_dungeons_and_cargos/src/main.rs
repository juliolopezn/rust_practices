use std::fs::File;
use std::collections::HashMap;
use csv::{ReaderBuilder, StringRecord};

// TIPO; TAG; TEXTO; VIDA
const FILE_NAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct Scene {
    category: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<Scene>,
}

impl Scene {
    fn new(record: StringRecord) -> Self {
        Scene {
            category: record.get(0).expect("Tipo not found in record").trim().to_string(),
            tag: record.get(1).expect("Tag not found in record").trim().to_string(),
            text: record.get(2).expect("Text not found in record").trim().to_string(),
            life: record.get(3).expect("Vida not found in record").trim().parse().unwrap_or(0),
            options: Vec::new(),
        }
    }
}

fn main() {
    let mut last_tag = String::from(FIRST_TAG);
    let mut story: HashMap<String, Scene> = HashMap::new();
    let history_content = File::open(FILE_NAME).expect("File not found");
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(history_content);

    for record in reader.records() {
        let record = record.expect("Error reading record");
        let scene = Scene::new(record);

        if scene.category == "SITUACION" {
            let scene_tag = scene.tag.clone();
            story.insert(scene_tag.clone(), scene);
            last_tag = scene_tag.clone();
        } else if scene.category == "OPCION" {
            let parent = story.get_mut(&last_tag).expect("Parent not found");
            parent.options.push(scene);
        }
    }

    let mut life: i32 = 100;
    let mut current_tag = String::from(FIRST_TAG);

    loop {
        println!("Tienes {} de vida", life);
        let current_scene = story.get_mut(&current_tag).expect("Tag not found");
        println!("{}", current_scene.text);
        
        life += current_scene.life;
        if life <= 0 {
            println!("Has muerto...");
            break;
        }

        for (index, option) in current_scene.options.iter().enumerate() {
            println!("|{}| {}", index + 1, option.text);
        }

        let mut input_selected_option = String::new();
        std::io::stdin().read_line(&mut input_selected_option).expect("Error reading line");
        let selected_option = input_selected_option.trim().parse().unwrap_or(999);

        if current_scene.options.len() < selected_option {
            println!("Opción no válida");
            continue;
        }

        let option_selected = &current_scene.options[selected_option - 1];
        current_tag = option_selected.tag.clone();
    }
}
