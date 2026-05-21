use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

use bevy::{log, prelude::*};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin, reply};
use clap::{Parser, Subcommand, ValueEnum};

/// Core structures

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
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

#[derive(Clone, Copy, Debug, Component, ValueEnum, Eq, PartialEq)]
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

impl Mama {
    const INITIAL: &[(Mama, FactionKind)] = &[
        (Mama::NewYork, FactionKind::Amnat),
        (Mama::LosAngeles, FactionKind::Amnat),
        (Mama::Moscow, FactionKind::Sovwar),
        (Mama::StPetersburg, FactionKind::Sovwar),
        (Mama::Beijing, FactionKind::Redchi),
        (Mama::Shanghai, FactionKind::Redchi),
        (Mama::Tehran, FactionKind::Irlibsyr),
        (Mama::Damascus, FactionKind::Irlibsyr),
        (Mama::Johannesburg, FactionKind::Southaf),
        (Mama::Lagos, FactionKind::Southaf),
        (Mama::Delhi, FactionKind::Indpak),
        (Mama::Karachi, FactionKind::Indpak),
    ];
}

/// Components

#[derive(Component)]
struct Polity;

#[derive(Component)]
struct Faction(FactionKind);

#[derive(Clone, Copy, Component, PartialEq, Eq, PartialOrd, Ord)]
struct Warheads(u32);

impl Warheads {
    fn budget(&self) -> Self {
        Self((self.0 as f64 * 0.2) as u32)
    }

    fn fits_in_budget(&self, total: Warheads) -> bool {
        self.0 < ((total.0 as f64 * 0.2) as u32)
    }

    fn decrement(&mut self, amount: Warheads) {
        self.0 = self.0 - amount.0
    }
}

#[derive(Component)]
struct Sufddir(u32);

impl Sufddir {
    fn increment(&mut self) {
        self.0 += 5;
    }
}

#[derive(Component)]
struct Inddir(u32);

impl Inddir {
    fn increment(&mut self) {
        self.0 += 5;
    }
}

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

#[derive(Component)]
struct Damage(u8);

impl Damage {
    fn increment(&mut self) {
        self.0 += 1
    }

    fn is_fatal(&self) -> bool {
        self.0 > 4
    }
}

#[derive(Component)]
struct Destroyed(bool);

impl Destroyed {
    fn toggle(&mut self) {
        self.0 = true
    }
}

/// Resources

#[derive(Resource)]
struct ActivePolity(Entity);

#[derive(Resource, Default)]
struct TurnOrder(VecDeque<Entity>);

#[derive(Resource)]
struct FactionPolities(HashMap<FactionKind, Entity>);

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
    order.0.push_back(redchi);

    let mut faction_map = HashMap::new();
    faction_map.insert(FactionKind::Amnat, amnat);
    faction_map.insert(FactionKind::Sovwar, sovwar);
    faction_map.insert(FactionKind::Redchi, redchi);
    commands.insert_resource(FactionPolities(faction_map));

    for (mama, faction) in Mama::INITIAL {
        commands.spawn((*mama, Faction(*faction), Damage(0), Destroyed(false)));
    }
}

/// Messages

#[derive(Message)]
enum ActionRequested {
    Attack(AttackTarget),
    Wait,
    Alliance(FactionKind),
    Sacpop(Warheads),
}

#[derive(Message)]
struct EndTurn;

/// Message systems

fn on_action_requested(
    mut actions: MessageReader<ActionRequested>,
    mut end_turn: MessageWriter<EndTurn>,
    mut mamas: Query<(Entity, &Mama, &Faction, &mut Damage, &mut Destroyed)>,
    mut polities: Query<(Entity, &Children, &mut Sufddir, &mut Inddir), With<Polity>>,
    faction_polities: Res<FactionPolities>,
    active: Res<ActivePolity>,
) {
    for action in actions.read() {
        match action {
            ActionRequested::Attack(attack_target) => match attack_target {
                AttackTarget::Mama {
                    target,
                    warheads: _,
                } => {
                    let Some((_entity, _mama, faction, mut damage, mut destroyed)) =
                        mamas.iter_mut().find(|(_, m, _, _, _)| **m == *target)
                    else {
                        log::error!("couldn't find Mama");
                        return;
                    };

                    if destroyed.0 {
                        log::info!("can't fire at destroyed target");
                        return;
                    }

                    let mama_faction = faction.0;
                    damage.increment();
                    if damage.is_fatal() {
                        destroyed.toggle();
                    }

                    let target_polity_entity = *faction_polities
                        .0
                        .get(&mama_faction)
                        .expect("couldn't find polity for faction");

                    let mut sufddir = polities.get_mut(target_polity_entity).unwrap().2;
                    sufddir.increment();

                    let mut inddir = polities.get_mut(active.0).unwrap().3;
                    inddir.increment();
                }
                AttackTarget::Satcom { warheads } => log::error!("satcom attack unimplemented"),
                AttackTarget::Powerplant { warheads } => {
                    log::error!("powerplant attack unimplemented")
                }
                AttackTarget::Submarine { warheads } => {
                    log::error!("submarine attack unimplemented")
                }
            },
            ActionRequested::Wait => {}
            ActionRequested::Alliance(faction_kind) => {
                log::error!("alliance unimplemented")
            }
            ActionRequested::Sacpop(warheads) => log::error!("sacpop unimplemented"),
        }
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
        warheads: u32,
    },
    Satcom {
        warheads: u32,
    },
    Powerplant {
        warheads: u32,
    },
    Submarine {
        warheads: u32,
    },
}

impl AttackTarget {
    fn warheads(&self) -> Warheads {
        Warheads(*match self {
            AttackTarget::Mama { warheads, .. } => warheads,
            AttackTarget::Satcom { warheads } => warheads,
            AttackTarget::Powerplant { warheads } => warheads,
            AttackTarget::Submarine { warheads } => warheads,
        })
    }
}

#[derive(Parser, Debug, ConsoleCommand)]
#[command(name = "alliance")]
struct AllianceCommand {
    faction: FactionKind,
}

#[derive(Parser, Debug, ConsoleCommand)]
#[command(name = "sacpop")]
struct SacpopCommand {
    warheads: u32,
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "ls")]
struct ListCommand;

/// Command systems

fn wait_command(mut log: ConsoleCommand<WaitCommand>, mut actions: MessageWriter<ActionRequested>) {
    if let Some(Ok(_)) = log.take() {
        reply!(log, "waiting");
        actions.write(ActionRequested::Wait);
    }
}

fn attack_command(
    mut log: ConsoleCommand<AttackCommand>,
    mut actions: MessageWriter<ActionRequested>,
    active: Res<ActivePolity>,
    mut warheads: Query<&mut Warheads, With<Polity>>,
) {
    if let Some(Ok(cmd)) = log.take() {
        let active_warheads = *warheads.get(active.0).unwrap();
        let budget = active_warheads.budget();
        if cmd.target.warheads() > active_warheads {
            reply!(log, "Too few warheads, you only have {}", active_warheads.0);
        } else if active_warheads < budget {
            reply!(log, "Too many warheads, you may only use {}", budget.0)
        } else {
            reply!(log, "{:?}", cmd);
            let mut hdl = warheads.get_mut(active.0).unwrap();
            hdl.decrement(cmd.target.warheads());
            actions.write(ActionRequested::Attack(cmd.target));
        }
    }
}

fn alliance_command(
    mut log: ConsoleCommand<AllianceCommand>,
    mut actions: MessageWriter<ActionRequested>,
) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
        actions.write(ActionRequested::Alliance(cmd.faction));
    }
}

fn sacpop_command(
    mut log: ConsoleCommand<SacpopCommand>,
    mut actions: MessageWriter<ActionRequested>,
) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "{:?}", cmd);
        actions.write(ActionRequested::Sacpop(Warheads(cmd.warheads)));
    }
}

fn list_command(
    mut log: ConsoleCommand<ListCommand>,
    polities: Query<(&Children, &Warheads, &Sufddir, &Inddir), With<Polity>>,
    mamas: Query<(&Mama, &Damage, &Destroyed)>,
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
        for (mama, damage, destroyed) in mamas {
            reply!(log, "{:?}", mama);
            reply!(log, "Damage: {}", damage.0);
            reply!(log, "Destroyed: {}", destroyed.0);
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
