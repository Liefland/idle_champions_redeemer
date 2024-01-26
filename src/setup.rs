use crate::config::Strategy::Local;
use crate::config::{dir, file, write, ConfigFile, Instructions};
use crate::err;
use crate::interaction::{await_enter, Coordinates, Interactor};
use enigo::{KeyboardControllable, MouseControllable};
use std::fmt::Display;
use std::fs::create_dir_all;

pub fn is_setup() -> bool {
    file().exists()
}

const DEFAULT_SLEEP_MS: u64 = 1000;

pub fn setup() -> Result<(), &'static str> {
    let path = dir();

    if !path.exists() {
        create_dir_all(&path).map_err(|_| "Failed to create config directory")?;
    }

    let mut enigo = enigo::Enigo::new();

    println!("Welcome to the setup!");
    println!("Please note, you will have to rerun the setup if you have changed the mouse coordinates (for instance, if you drag the Idle Champions window to a different location).");
    println!("Please navigate to the chest UI in Idle Champions before proceeding. Hit ENTER to continue or CTRL-C to abort.");
    await_enter();

    println!("For the following steps, please move your mouse to the desired location and hit ENTER to proceed, or CTRL-C to abort.");
    println!(
        "The program will send mouse clicks for you during this stage as you complete a step."
    );

    println!("Step 1: Hover your mouse over the chest 'Unlock a Locked Chest' button.");
    println!("This is located in the bottom left corner of the UI");
    await_enter();
    let unlock_chest = get_cursor_position(&enigo)?;
    println!(
        "Step 1: Registered coordinates X:{}, Y:{}\n",
        unlock_chest.x, unlock_chest.y
    );

    enigo.mouse_click(enigo::MouseButton::Left);

    println!("Step 2: Hover your mouse over the chest '12 Characters' button.");
    println!("This is located in the bottom left corner of the UI for unlocking chests");
    await_enter();
    let character_switch = get_cursor_position(&enigo)?;
    println!(
        "Step 2: Registered coordinates X:{}, Y:{}",
        character_switch.x, character_switch.y
    );

    let instructions = Instructions {
        unlock_chest,
        character_switch,
    };

    enigo.key_click(enigo::Key::Escape);
    match demo(&instructions) {
        Ok(_) => {}
        Err(e) => {
            err!("Failed to run demo: {}", e);
        }
    };

    println!("Step 4: Saving config file.");
    println!("{}", instructions);

    write(&ConfigFile {
        default_strategy: Local,
        instructions,
        remote: None,
        sleep_ms: DEFAULT_SLEEP_MS,
    })?;

    Ok(())
}

fn demo(instructions: &Instructions) -> Result<(), String> {
    println!("We will now test a full cycle of the program.");
    println!(
        "This will take about {} seconds, will open the chest UI, unlock a chest, and close the UI.",
        DEFAULT_SLEEP_MS * 5
    );
    let mut interactor = Interactor::new(*instructions, 1000, true);
    interactor.redeem("DEMO-REDE-EMER-IDLE")
}

fn get_cursor_position(enigo: &enigo::Enigo) -> Result<Coordinates, &'static str> {
    let (x, y) = enigo.mouse_location();

    Ok(Coordinates { x, y })
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unlock_chest = {}, character_switch = {}",
            self.unlock_chest, self.character_switch,
        )
    }
}
