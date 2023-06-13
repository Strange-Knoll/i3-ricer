use std::{fs::File, io::Write};

use crate::theme::Theme;

fn bytes_to_hex(bytes:&[u8; 4]) -> String{
    let mut out = String::new();
    for byte in bytes{
        out.push_str(&format!("{:02x}", byte));
    }
    out
}

pub fn compile(theme:Theme) -> String{
    let mut out = String::new();
    out.push_str("# --- Generated by Ricer --- # 
    # Ricer is not a complete i3 config generator.
    # It only generates the colorscheme and wallpaper. 
    # You will need to add your own keybindings and other settings.
    # Users are encouraged to read the official I3 documentation.\n\n");

    //wallpaper
    out.push_str(format!("exec_always feh --bg-fill {}\n\n", theme.wallpaper.path).as_str());
    
    //border and gaps
    out.push_str(format!("default_border {}px\n", theme.border).as_str());
    out.push_str(format!("default_floating_border {}px\n\n", theme.border).as_str());
    out.push_str(format!("gaps inner {}px\n", theme.gaps).as_str());
    out.push_str(format!("gaps outer {}px\n\n", theme.gaps).as_str());
    
    //color palette
    out.push_str(&format!("client.focused           #{} #{} #{} #{} #{}\n",
        bytes_to_hex(&theme.focused.border as &[u8; 4]),
        bytes_to_hex(&theme.focused.background as &[u8; 4]),
        bytes_to_hex(&theme.focused.text as &[u8; 4]),
        bytes_to_hex(&theme.focused.indicator as &[u8; 4]),
        bytes_to_hex(&theme.focused.child_border as &[u8; 4]),
    ));
    out.push_str(&format!("client.focused_inactive  #{} #{} #{} #{} #{}\n",
        bytes_to_hex(&theme.focused_inactive.border as &[u8; 4]),
        bytes_to_hex(&theme.focused_inactive.background as &[u8; 4]),
        bytes_to_hex(&theme.focused_inactive.text as &[u8; 4]),
        bytes_to_hex(&theme.focused_inactive.indicator as &[u8; 4]),
        bytes_to_hex(&theme.focused_inactive.child_border as &[u8; 4]),
    ));
    out.push_str(&format!("client.unfocused         #{} #{} #{} #{} #{}\n",
        bytes_to_hex(&theme.unfocused.border as &[u8; 4]),
        bytes_to_hex(&theme.unfocused.background as &[u8; 4]),
        bytes_to_hex(&theme.unfocused.text as &[u8; 4]),
        bytes_to_hex(&theme.unfocused.indicator as &[u8; 4]),
        bytes_to_hex(&theme.unfocused.child_border as &[u8; 4]),
    ));
    out.push_str(&format!("client.urgent            #{} #{} #{} #{} #{}\n",
        bytes_to_hex(&theme.urgent.border as &[u8; 4]),
        bytes_to_hex(&theme.urgent.background as &[u8; 4]),
        bytes_to_hex(&theme.urgent.text as &[u8; 4]),
        bytes_to_hex(&theme.urgent.indicator as &[u8; 4]),
        bytes_to_hex(&theme.urgent.child_border as &[u8; 4]),
    ));
    out.push_str(&format!("client.placeholder       #{} #{} #{} #{} #{}\n",
        bytes_to_hex(&theme.placeholder.border as &[u8; 4]),
        bytes_to_hex(&theme.placeholder.background as &[u8; 4]),
        bytes_to_hex(&theme.placeholder.text as &[u8; 4]),
        bytes_to_hex(&theme.placeholder.indicator as &[u8; 4]),
        bytes_to_hex(&theme.placeholder.child_border as &[u8; 4]),
    ));
    out 
}

pub fn save_config(theme:Theme, path:&str){
    let mut file = File::create(path).unwrap();
    file.write_all(compile(theme).as_bytes()).unwrap();
}