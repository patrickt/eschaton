use bevy::prelude::*;
use std::{collections::HashMap, fmt};

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

#[derive(Resource, Default)]
struct Aggressions {
    hatreds: Vec<(Entity, Entity)>,
}

fn populate_players(mut commands: Commands) {
    commands.spawn((Player, Human, Faction::AmNat));
    commands.spawn((Player, Cpu, Faction::SovWar));
    commands.spawn((Player, Cpu, Faction::RedChi));
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

fn do_turn(turn: Res<TurnOrder>, factions: Query<&Faction>) {
    let current_player: &Faction = factions.get(turn.players[turn.current]).unwrap();
    println!("Current player is {:?}", current_player);
    println!("Press enter to continue");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "exit" {
        std::process::exit(0);
    }
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TurnOrder::default())
        .add_systems(
            Startup,
            (
                populate_players,
                establish_turn_order.after(populate_players),
                populate_mamas.after(establish_turn_order),
            ),
        )
        .add_systems(Update, (do_turn, next_turn))
        .run();
}
