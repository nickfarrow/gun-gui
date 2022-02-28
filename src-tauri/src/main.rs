#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::str::FromStr;

use bdk::{
    bitcoin::{Address, Amount},
    FeeRate,
};
use gun_wallet::{
    cmd::{self, FeeArgs, SendOpt, SpendOpt},
    FeeSpec,
};

#[tauri::command]
fn get_balance() -> String {
    let wallet_dir = dirs::home_dir().unwrap().join(".gun");
    let (wallet, _keychain, _config) = cmd::load_wallet(&wallet_dir).expect("loading wallet");
    wallet.sync().expect("syncing wallet");
    let out = cmd::run_balance(&wallet, true);
    out.unwrap().render_json().to_string()
}

#[tauri::command]
fn send_coins(value: String, to: String) -> String {
    let wallet_dir = dirs::home_dir().unwrap().join(".gun");
    let (wallet, _keychain, _config) = cmd::load_wallet(&wallet_dir).expect("loading wallet");

    // let spend_opt = SpendOpt {
    //     fee_args: FeeArgs {
    //         fee: gun_wallet::FeeSpec::Rate(FeeRate::from_sat_per_vb(1.0)),
    //     },
    //     spend_in_use: false,
    //     no_spend_unclaimed: true,
    //     bump_claiming: false,
    //     yes: true,
    //     print_tx: false,
    // };

    // let mut builder = wallet.bdk_wallet().build_tx();
    // builder.add_recipient(
    //     Address::from_str(&to).unwrap().script_pubkey(),
    //     Amount::from_str(&value).unwrap().as_sat(),
    // );

    // spend_opt.spend_coins(&wallet, builder);

    dbg!(value.clone(), to.clone());

    println!("Pretending to send {} to {}", value, to);
    "SENT".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_balance, send_coins])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
