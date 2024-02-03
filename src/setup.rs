use crate::config::Strategy::Remote;
use crate::config::{dir, file, write, ConfigFile, Instructions};
use crate::err;
use crate::interaction::{await_enter, Coordinates, Interactor};
use enigo::MouseControllable;
use std::fmt::Display;
use std::fs::create_dir_all;

pub fn is_setup() -> bool {
    file().exists()
}

pub fn setup() -> Result<(), &'static str> {
    let path = dir();

    if !path.exists() {
        create_dir_all(&path).map_err(|_| "Failed to create config directory")?;
    }

    let enigo = enigo::Enigo::new();

    println!("Welcome to the setup!");
    println!("Please note, you will have to rerun the setup if you have changed the mouse coordinates (for instance, if you drag the Idle Champions window to a different location).");
    println!("Please navigate to the chest UI in Idle Champions before proceeding. Hit ENTER to continue or CTRL-C to abort.");
    await_enter();

    println!("Hover your mouse over the chest 'Unlock a Locked Chest' button.");
    println!("  This is located in the bottom left corner of the UI");
    println!("  Hit ENTER to register the coordinates, or CTRL-C to abort.");
    await_enter();
    let unlock_chest = get_cursor_position(&enigo)?;
    println!(
        "Registered coordinates X:{}, Y:{}\n",
        unlock_chest.x, unlock_chest.y
    );
    let instructions = Instructions { unlock_chest };

    match demo(&instructions) {
        Ok(_) => {}
        Err(e) => {
            err!("Failed to run demo: {}", e);
        }
    };

    println!("Saving config file.");
    println!("{}", instructions);

    write(&ConfigFile {
        default_strategy: Remote,
        instructions,
        remote: None,
        slow: false,
    })?;

    Ok(())
}

fn demo(instructions: &Instructions) -> Result<(), String> {
    println!("We will now test a full cycle of the program.");
    println!("This will open the chest UI, unlock a chest, and close the UI. Please avoid using the mouse and keyboard.");
    let mut interactor = Interactor::new(*instructions, false, true);
    interactor.redeem("DEMO-REDE-EMER-IDLE")
}

fn get_cursor_position(enigo: &enigo::Enigo) -> Result<Coordinates, &'static str> {
    let (x, y) = enigo.mouse_location();

    Ok(Coordinates { x, y })
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unlock_chest = {}", self.unlock_chest)
    }
}
