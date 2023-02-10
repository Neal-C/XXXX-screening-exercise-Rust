use std::collections::HashSet;

#[derive(Debug)]
pub struct Pronostics {
    pub champion: ChessPlayer,
    pub possible_champions: Vec<ChessPlayer>,
}

impl Default for Pronostics {
    fn default() -> Self {
        return Self {
            champion: ChessPlayer::default(),
            possible_champions: Vec::<ChessPlayer>::new(),
        };
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct ChessPlayer {
    pub elo: u16,
    pub age: u8,
    pub name: String,
}

mod helpers {
    use super::ChessPlayer;
    pub fn is_stronger(target: &ChessPlayer, other: &ChessPlayer) -> bool {
        return target.elo > other.elo;
    }

    pub fn is_younger(target: &ChessPlayer, other: &ChessPlayer) -> bool {
        return target.age < other.age;
    }

    pub fn are_equal_players(target: &ChessPlayer, other: &ChessPlayer) -> bool {
        return target.elo == other.elo && target.age == other.age;
    }
}

pub fn get_pronostics(player_list: Vec<ChessPlayer>) -> Pronostics {
    let mut pronostics = Pronostics::default();

    for player in player_list {
        if pronostics
            .possible_champions
            .iter()
            .any(|possible_champion| !helpers::is_stronger(possible_champion, &player))
        {
            pronostics.champion = player.clone();
            pronostics.possible_champions = Vec::new();
            continue;
        }
        if helpers::are_equal_players(&pronostics.champion, &player) {
            pronostics
                .possible_champions
                .push(pronostics.champion.clone());
            pronostics.possible_champions.push(player.clone());
            // pronostics.possible_champions : Vec<ChessPlayer> = BTreeSet::from(pronostics
            //     .possible_champions)
            //     .into_iter().collect()
            //----------
            // pronostics.possible_champions = pronostics.possible_champions.iter().unique();
            //-----------------
            // pronostics.possible_champions = pronostics.possible_champions.sort().dedup();
            //-----------
            pronostics.possible_champions = Vec::from_iter::<HashSet<ChessPlayer>>(HashSet::from_iter(
                pronostics.possible_champions,
            ));

            continue;
        };

        if !helpers::is_stronger(&pronostics.champion, &player)
            && !helpers::is_younger(&pronostics.champion, &player)
        {
            pronostics.champion = player.clone();
            continue;
        }

        if !helpers::is_stronger(&pronostics.champion, &player) {
            pronostics.champion = player.clone();
            continue;
        }
    }

    return pronostics;
} // end of get_pronostics


#[cfg(test)]
mod tests {
    use crate::{get_pronostics, ChessPlayer, Pronostics};

    #[test]
    fn returns_player_with_highest_elo() {
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer {
                elo: 3100,
                age: 33,
                name: String::from("Sherlock"),
            },
            ChessPlayer {
                elo: 3000,
                age: 32,
                name: String::from("Magnus"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Francis"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            ChessPlayer {
                elo: 700,
                age: 30,
                name: String::from("Félix"),
            },
            ChessPlayer {
                elo: 2700,
                age: 31,
                name: String::from("Erik Lehnsherr"),
            },
            ChessPlayer {
                elo: 2800,
                age: 30,
                name: String::from("Charles Xavier"),
            },
        ];

        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.champion.name, "Sherlock");
    }

    #[test]
    fn returns_player_with_highest_elo_and_youngest_age() {
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer {
                elo: 3000,
                age: 33,
                name: String::from("Sherlock"),
            },
            ChessPlayer {
                elo: 3000,
                age: 32,
                name: String::from("Magnus"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Francis"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            ChessPlayer {
                elo: 700,
                age: 30,
                name: String::from("Félix"),
            },
            ChessPlayer {
                elo: 2700,
                age: 31,
                name: String::from("Erik Lehnsherr"),
            },
            ChessPlayer {
                elo: 2800,
                age: 30,
                name: String::from("Charles Xavier"),
            },
        ];

        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.champion.name, "Magnus");
    }
    #[test]
    fn a_list_with_multiple_possible_champions_cant_be_empty() {
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Sherlock"),
            },
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Magnus"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Francis"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            ChessPlayer {
                elo: 700,
                age: 30,
                name: String::from("Félix"),
            },
            ChessPlayer {
                elo: 2700,
                age: 31,
                name: String::from("Erik Lehnsherr"),
            },
            ChessPlayer {
                elo: 2800,
                age: 30,
                name: String::from("Charles Xavier"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert!(!result.possible_champions.is_empty());

    }

    #[test]
    fn cannot_return_a_default_object_with_default_values() {
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer {
                elo: 3000,
                age: 33,
                name: String::from("Sherlock"),
            },
            ChessPlayer {
                elo: 3000,
                age: 32,
                name: String::from("Magnus"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Francis"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            ChessPlayer {
                elo: 700,
                age: 30,
                name: String::from("Félix"),
            },
            ChessPlayer {
                elo: 2700,
                age: 31,
                name: String::from("Erik Lehnsherr"),
            },
            ChessPlayer {
                elo: 2800,
                age: 30,
                name: String::from("Charles Xavier"),
            },
        ];

        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_ne!(result.champion, ChessPlayer::default());
    }
    #[test]
    fn returns_all_players_with_highest_elo_and_same_age() {
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
            ChessPlayer {
                elo: 2900,
                age: 30,
                name: String::from("Boo"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            ChessPlayer {
                elo: 700,
                age: 30,
                name: String::from("Félix"),
            },
            ChessPlayer {
                elo: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            ChessPlayer {
                elo: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];

        let possible_champions = vec![
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.possible_champions, possible_champions);
    }

    #[test]
    fn returns_all_3_players_with_highest_elo_and_same_age() {
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Boo"),
            },
            ChessPlayer {
                elo: 2999,
                age: 24,
                name: String::from("Moriarty"),
            },
            ChessPlayer {
                elo: 700,
                age: 30,
                name: String::from("Félix"),
            },
            ChessPlayer {
                elo: 2700,
                age: 31,
                name: String::from("Michael"),
            },
            ChessPlayer {
                elo: 2800,
                age: 30,
                name: String::from("Karl"),
            },
        ];

        let possible_champions = vec![
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Kareem"),
            },
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Lebron"),
            },
            ChessPlayer {
                elo: 3000,
                age: 30,
                name: String::from("Boo"),
            },
        ];
        //ACT
        let result: Pronostics = get_pronostics(player_list);
        //ASSERT
        assert_eq!(result.possible_champions, possible_champions);
    }
}
