mod modules;

fn main() {
    modules::display::show_logo();

    if modules::display::get_user_confirmation() {
        println!("\nProceeding...\n");

        match modules::remove::remove_tailscale_files() {
            Ok(_) => {
                modules::remove::show_completion_message();
            }
            Err(e) => {
                eprintln!("\n{}\n", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("Cancelled.");
        std::process::exit(0);
    }
}
