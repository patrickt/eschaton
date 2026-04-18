use bevy::prelude::*;
use std::{collections::HashMap, fmt};

enum Player {
    Human {
        faction: Faction,
    },
    Cpu {
        faction: Faction,
        hatreds: HashMap<Faction, u32>,
    },
}

impl Player {
    fn total_hatred(&self) -> u32 {
        use Player::*;
        match self {
            Human { .. } => 0,
            Cpu { hatreds, .. } => hatreds.values().sum(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Faction {
    AmNat,
    SovWar,
    RedChi,
    IrLibSyr,
    SouthAf,
    IndPak,
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Faction::*;
        write!(
            f,
            "{}",
            match self {
                AmNat => "AMNAT",
                SovWar => "SOVWAR",
                RedChi => "REDCHI",
                IrLibSyr => "IRLIBSYR",
                SouthAf => "SOUTHAF",
                IndPak => "INDPAK",
            }
        )
    }
}

impl Faction {
    fn available(&self) -> bool {
        use Faction::*;
        matches!(self, AmNat | SovWar | RedChi)
    }

    fn starting_megatons(&self) -> u16 {
        use Faction::*;
        match self {
            AmNat => 400,
            SovWar => 80,
            RedChi => 325,
            IrLibSyr => 300,
            SouthAf => 300,
            IndPak => 275,
        }
    }
}

enum City {
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

#[derive(Debug, Clone, Copy)]
enum Site {
    City {
        owner: Faction,
        ident: City,
        location: Vec2,
    },
    PowerPlant {
        owner: Faction,
        location: Vec2,
    },
    SsTrac {
        owner: Faction,
        location: Vec2,
    },
    Submarine {
        owner: Faction,
        location: Vec2,
    },
}

impl Site {
    fn points(&self) -> u16 {
        use Site::*;
        match self {
            City { .. } => 15,
            PowerPlant { .. } => 10,
            SsTrac { .. } => 10,
            Submarine { .. } => 5,
        }
    }
}

enum Scenario {
    FlightOfGeese,
    Explosions,
    PrinceAlbert,
    MonitoringStation,
    Antimissile,
    BorderDispute,
    Defcon3,
}

impl Scenario {
    fn flavor_text(&self) -> &'static str {
        use Scenario::*;
        match self {
            FlightOfGeese => {
                "AN AMNAT COMPUTRACKER IN THE ALEUTIANS MISREADS A FLIGHT OF GEESE AS THREE SOVWAR SS10S ON REENTRY."
            }
            Explosions => {
                "EXPLOSIONS OF SUSPICIOUS ORIGIN OCCUR AT AMNAT SATELLITE-RECEIVER STATIONS FROM TURKEY TO LABRADOR AS THREE HIGH-LEVEL \
CANADIAN DEFENSE MINISTERS VANISH AND THEN A COUPLE OF DAYS LATER ARE PHOTOGRAPHED AT A VOLGOGRAD BISTRO HOISTING SHOTS OF STOLICHNAYA WITH SLAVIC BIMBOS ON THEIR KNEE."
            }
            PrinceAlbert => {
                "SOVWAR’S BALD AND PORT-WINE-STAINED PREMIER CALLS AMNAT’S WATTLE-CHINNED PRESIDENT ON THE HOT LINE AND ASKS HIM IF HE’S GOT PRINCE ALBERT IN A CAN."
            }
            MonitoringStation => {
                "ANOTHER PRETTY SHADY EXPLOSION LEVELS A SOVWAR BIG EAR MONITORING STATION ON SAKHALIN."
            }
            Antimissile => {
                "AMNAT IS WITHIN 72 HOURS OF PUTTING AN IMPREGNABLE STRING OF ANTIMISSILE SATELLITES ON LINE."
            }
            BorderDispute => "A RUSSO-CHINESE BORDER DISPUTE GOES TACTICAL OVER SINKIANG.",
            Defcon3 => {
                "REDCHI GOES TO DEFCON 3, IN RESPONSE TO WHICH SOVWAR AIRFIELDS AND ANTIMISSILE NETWORKS FROM IRKUTSK TO THE DZHUGDZHUR RANGE GO TO DEFCON 5."
            }
        }
    }
}

enum Choice {
    Wait,
    Attack(Site),
    Ally(Faction),
    SacPop,
}

fn main() {
    println!("Hello, world!");
}
