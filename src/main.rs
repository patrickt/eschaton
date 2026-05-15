use std::{collections::VecDeque, fmt};

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin, reply};
use clap::{Parser, Subcommand, ValueEnum};

/// Core structures

#[derive(Clone, Copy, Debug, ValueEnum)]
enum FactionKind {
    Amnat,
    Sovwar,
    Redchi,
    Irlibsyr,
    Southaf,
    Indpak,
}

impl fmt::Display for FactionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FactionKind::*;
        write!(
            f,
            "{}",
            match self {
                Amnat => "AMNAT",
                Sovwar => "SOVWAR",
                Redchi => "REDCHI",
                Irlibsyr => "IRLIBSYR",
                Southaf => "SOUTHAF",
                Indpak => "INDPAK",
            }
        )
    }
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

/// Components

#[derive(Component)]
struct Polity;

#[derive(Component)]
struct Faction(FactionKind);

#[derive(Component)]
struct Warheads(u32);

#[derive(Component)]
struct Sufddir(u32);

#[derive(Component)]
struct Inddir(u32);

#[derive(Component)]
struct Human;

#[derive(Component)]
struct Cpu;

#[derive(Component)]
enum Target {
    Mama(Mama),
    Sstrac,
    Plant,
    Sub,
}

/// Resources

#[derive(Resource)]
struct ActivePolity(Entity);

#[derive(Resource, Default)]
struct TurnOrder(VecDeque<Entity>);

/// Systems

fn setup(mut commands: Commands, mut active: ResMut<ActivePolity>, mut order: ResMut<TurnOrder>) {
    let amnat = commands
        .spawn((Polity, Human, Warheads(400), Sufddir(0), Inddir(0)))
        .with_child(Faction(FactionKind::Amnat))
        .id();

    active.0 = amnat;

    let sovwar = commands
        .spawn((Polity, Cpu, Warheads(400), Sufddir(0), Inddir(0)))
        .with_child(Faction(FactionKind::Sovwar))
        .id();

    let redchi = commands
        .spawn((Polity, Cpu, Warheads(325), Sufddir(0), Inddir(0)))
        .with_child(Faction(FactionKind::Redchi))
        .id();

    order.0.push_back(amnat);
    order.0.push_back(sovwar);
    order.0.push_back(redchi)
}

/// Messages

#[derive(Message)]
enum ActionRequested {
    Attack,
    Wait,
    Alliance,
    Sacpop,
}

#[derive(Message)]
struct EndTurn;

/// Message systems

fn on_action_requested(
    mut actions: MessageReader<ActionRequested>,
    mut end_turn: MessageWriter<EndTurn>,
) {
    for _action in actions.read() {
        end_turn.write(EndTurn);
    }
}

fn on_end_turn(
    mut turns: MessageReader<EndTurn>,
    mut active: ResMut<ActivePolity>,
    mut queue: ResMut<TurnOrder>,
    polities: Query<Entity, With<Polity>>,
) {
    for _turn in turns.read() {
        if queue.0.is_empty() {
            for polity in polities {
                queue.0.push_back(polity);
            }
        }

        let next = queue.0.pop_front().unwrap();
        active.0 = next;
    }
}

/// Command structures

#[derive(Parser, ConsoleCommand)]
#[command(name = "wait")]
struct WaitCommand;

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

#[derive(Parser, Debug, ConsoleCommand)]
#[command(name = "alliance")]
struct AllianceCommand {
    faction: FactionKind,
}

#[derive(Parser, Debug, ConsoleCommand)]
#[command(name = "sacpop")]
struct SacpopCommand {
    warheads: u16,
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "ls")]
struct ListCommand;

/// Command systems

fn wait_command(
    mut log: ConsoleCommand<WaitCommand>,
    mut actions: MessageWriter<ActionRequested>,
) {
    if let Some(Ok(_)) = log.take() {
        reply!(log, "waiting");
        actions.write(ActionRequested::Wait);
    }
}

fn attack_command(mut log: ConsoleCommand<AttackCommand>) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
    }
}

fn alliance_command(mut log: ConsoleCommand<AllianceCommand>) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
    }
}

fn sacpop_command(mut log: ConsoleCommand<SacpopCommand>) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
    }
}

fn list_command(
    mut log: ConsoleCommand<ListCommand>,
    polities: Query<(&Children, &Warheads, &Sufddir, &Inddir), With<Polity>>,
    factions: Query<&Faction>,
) {
    if let Some(Ok(_)) = log.take() {
        for (polity, warheads, sufddir, inddir) in polities {
            let mut buffer = Vec::<String>::new();
            for faction_ent in polity {
                let faction = factions.get(*faction_ent).unwrap();
                buffer.push(faction.0.to_string());
            }
            reply!(log, "{}", buffer.join(", "));
            reply!(log, "Warheads: {}", warheads.0);
            reply!(log, "SUFDDIR: {}", sufddir.0);
            reply!(log, "INDDIR: {}", inddir.0);
            reply!(log, "\n");
        }
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .insert_resource(ActivePolity(Entity::PLACEHOLDER))
        .insert_resource(TurnOrder::default())
        .add_systems(Startup, (setup_camera_system, setup))
        .add_message::<ActionRequested>()
        .add_message::<EndTurn>()
        .add_systems(Update, (on_action_requested, on_end_turn).chain())
        .add_console_command::<WaitCommand, _>(wait_command)
        .add_console_command::<AttackCommand, _>(attack_command)
        .add_console_command::<AllianceCommand, _>(alliance_command)
        .add_console_command::<SacpopCommand, _>(sacpop_command)
        .add_console_command::<ListCommand, _>(list_command)
        .run();
}
