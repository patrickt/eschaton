use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin, reply};
use clap::{Parser, Subcommand, ValueEnum};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .add_systems(Startup, setup_camera_system)
        .add_console_command::<WaitCommand, _>(wait_command)
        .add_console_command::<AttackCommand, _>(attack_command)
        .add_console_command::<AllianceCommand, _>(alliance_command)
        .add_console_command::<SacpopCommand, _>(sacpop_command)
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "wait")]
struct WaitCommand;

fn wait_command(mut log: ConsoleCommand<WaitCommand>) {
    if let Some(Ok(_)) = log.take() {
        reply!(log, "waiting");
    }
}

#[derive(Parser, Debug, ConsoleCommand)]
#[command(name = "attack")]
struct AttackCommand {
    #[command(subcommand)]
    target: AttackTarget,
}

#[derive(Clone, Copy, Debug, Subcommand)]
enum AttackTarget {
    Mama {
        #[arg(value_enum)]
        target: Mama,
        warheads: u16,
    },
    Satcom {
        warheads: u16,
    },
    Powerplant {
        warheads: u16,
    },
    Submarine {
        warheads: u16,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Mama {
    NewYork,
    LosAngeles,
    Moscow,
    StPetersburg,
    Beijing,
    Shanghai,
    Tehran,
    Damascus,
    Johannesburg,
    Lagos,
    Delhi,
    Karachi,
}

fn attack_command(mut log: ConsoleCommand<AttackCommand>) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
    }
}

#[derive(Parser, Debug, ConsoleCommand)]
#[command(name = "alliance")]
struct AllianceCommand {
    faction: Faction,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Faction {
    Amnat,
    Sovwar,
    Redchi,
    Irlibsyr,
    Southaf,
    Indpak,
}

fn alliance_command(mut log: ConsoleCommand<AllianceCommand>) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
    }
}

#[derive(Parser, Debug, ConsoleCommand)]
#[command(name = "sacpop")]
struct SacpopCommand {
    warheads: u16,
}

fn sacpop_command(mut log: ConsoleCommand<SacpopCommand>) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
    }
}
