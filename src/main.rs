use bevy::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
};

#[derive(Component)]
struct Human;

#[derive(Component)]
struct Cpu;

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
enum Faction {
    AmNat,
    SovWar,
    RedChi,
    IrLibSyr,
    SouthAf,
    IndPak,
}

#[derive(Component, Debug)]
enum MaMa {
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

#[derive(Resource, Default)]
struct TurnOrder {
    current: usize,
    players: Vec<Entity>,
}

#[derive(Debug, Clone)]
enum Choice {
    Wait,
    AttackMaMa,
    AttackSatCom,
    AttackPlants,
    AttackSubmarines,
    ProposeAlliance,
}

#[derive(Resource)]
struct SelectedChoice(Choice);

#[derive(Clone, Copy, Debug, Component)]
struct Missiles(u32);

impl Missiles {
    fn maximum_allowed(&self) -> Self {
        if self.0 > 50 {
            Missiles((self.0 as f64 / 5.0) as u32)
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Choice::*;
        match self {
            Wait => write!(f, "Wait it out"),
            AttackMaMa => write!(f, "Attack Major Metropolitan Area"),
            AttackSatCom => write!(f, "Attack Site of Strategic Command"),
            AttackPlants => write!(f, "Attack power plant"),
            AttackSubmarines => write!(f, "Attack submarines"),
            ProposeAlliance => write!(f, "Propose alliance"),
        }
    }
}

impl Choice {
    fn read_from_stdin() -> Self {
        use Choice::*;
        let choices = &[
            Wait,
            AttackMaMa,
            AttackSatCom,
            AttackPlants,
            AttackSubmarines,
            ProposeAlliance,
        ];
        for (idx, choice) in choices.iter().enumerate() {
            println!("{}) {}", idx + 1, choice)
        }
        loop {
            let result = || {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).ok()?;
                let idx = input.trim().parse::<i32>().ok()?;
                if idx >= 1 && idx <= (choices.len() as i32) {
                    Some(choices[(idx - 1) as usize].clone())
                } else {
                    None
                }
            };
            if let Some(done) = result() {
                return done;
            } else {
                println!("Invalid input")
            }
        }
    }
}

#[derive(Resource, Default)]
struct Aggressions {
    hatreds: BTreeMap<Entity, Entity>,
}

fn populate_players(mut commands: Commands, mut agg: ResMut<Aggressions>) {
    let amnat = commands
        .spawn((Player, Human, Faction::AmNat, Missiles(400)))
        .id();
    let sovwar = commands
        .spawn((Player, Cpu, Faction::SovWar, Missiles(400)))
        .id();
    let _redchi = commands
        .spawn((Player, Cpu, Faction::RedChi, Missiles(325)))
        .id();
    agg.hatreds.insert(amnat, sovwar);
}

fn populate_mamas(mut commands: Commands) {
    use Faction::*;
    use MaMa::*;
    commands.spawn_batch([
        (NewYork, AmNat),
        (LosAngeles, AmNat),
        (Moscow, SovWar),
        (StPetersburg, SovWar),
        (Beijing, RedChi),
        (Shanghai, RedChi),
        (Tehran, IrLibSyr),
        (Damascus, IrLibSyr),
        (Johannesburg, SouthAf),
        (Lagos, SouthAf),
        (Delhi, IndPak),
        (Karachi, IndPak),
    ]);
}

fn establish_turn_order(query: Query<Entity, With<Player>>, mut turn: ResMut<TurnOrder>) {
    turn.current = 0;
    for player in query {
        turn.players.push(player);
    }
}

fn next_turn(mut turn: ResMut<TurnOrder>) {
    turn.current = (turn.current + 1) % turn.players.len();
}

fn do_turn(
    turn: Res<TurnOrder>,
    factions: Query<&Faction>,
    mut player_choice: ResMut<SelectedChoice>,
) {
    let current_player: &Faction = factions.get(turn.players[turn.current]).unwrap();
    println!("Current player is {:?}", current_player);
    println!("Press enter to continue");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "exit" {
        std::process::exit(0);
    }
    let choice = Choice::read_from_stdin();
    *player_choice = SelectedChoice(choice);
}

fn handle_choice(
    turn: Res<TurnOrder>,
    mut factions: Query<(&Faction, &mut Missiles)>,
    player_choice: Res<SelectedChoice>,
) {
    let (player, missiles) = factions.get_mut(turn.players[turn.current]).unwrap();
    match player_choice.0 {
        Choice::Wait => (),
        Choice::AttackMaMa => {}
        Choice::AttackSatCom => todo!(),
        Choice::AttackPlants => todo!(),
        Choice::AttackSubmarines => todo!(),
        Choice::ProposeAlliance => todo!(),
    }
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TurnOrder::default())
        .insert_resource(SelectedChoice(Choice::Wait))
        .insert_resource(Aggressions::default())
        .add_systems(
            Startup,
            (
                populate_players,
                establish_turn_order.after(populate_players),
                populate_mamas.after(establish_turn_order),
            ),
        )
        .add_systems(Update, (do_turn, handle_choice, next_turn))
        .run();
}
