use g_rust::extension::extension::{Extension, ExtensionInfo};
use g_rust::extension::parsers::incoming::{CloseConnection, WiredFurniVariable};
use g_rust::misc::connectioninfo::ConnectionInfo;
use g_rust::protocol::hmessage::HMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct GVariables {
    user_name_by_index: HashMap<i32, String>,
    habbo_host: Option<String>,
}

impl GVariables {
    fn clear(&mut self) {
        self.user_name_by_index.clear();
        self.habbo_host = None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VariablesItem {
    name: String,
    gender: String,
    figure: String,
}

pub fn run() {
    println!("G-Variables -> extension.rs -> runned!");
    let mut ext: Extension<GVariables> = Extension::new();
    ext.info = ExtensionInfo {
        name: "G-Variables".to_string(),
        description: "Extended variables tools".to_string(),
        author: "!K2".to_string(),
        version: "0.1.0".to_string(),
    };
    ext.on_init(on_init);
    ext.on_connect(on_connect);
    ext.on_start(on_start);
    ext.on_end(on_end);
    ext.intercept(on_close_connection);
    ext.intercept(on_wired_furni_variable);
    ext.run();
}

fn on_init(_ext: &mut Extension<GVariables>) {
    println!("G-Variables -> extension.rs -> on_init");
}

fn on_connect(ext: &mut Extension<GVariables>, connectioninfo: ConnectionInfo) {
    ext.globals.habbo_host = Some(connectioninfo.host);
    println!(
        "G-Variables -> extension.rs -> on_connect: {}",
        ext.globals.habbo_host.clone().unwrap()
    );
}

fn on_start(_ext: &mut Extension<GVariables>) {
    println!("G-Variables -> extension.rs -> on_start");
}

fn on_end(_ext: &mut Extension<GVariables>) {
    println!("G-Variables -> extension.rs -> on_end");
}

fn on_close_connection(ext: &mut Extension<GVariables>, _: &mut HMessage, _: &mut CloseConnection) {
    ext.globals.clear();
}

fn on_wired_furni_variable(
    ext: &mut Extension<GVariables>,
    _: &mut HMessage,
    variable: &mut WiredFurniVariable,
) {
    println!("G-Variables -> extension.rs -> on_wired_furni_variable");

    println!("Variable: {}", variable.def.string_param);
}
