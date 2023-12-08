use gstd::prelude::*;
use gear_lib_old::non_fungible_token::token::TokenId;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTDefault {
    pub name: String,
    pub description: String,
    pub media: String,
    pub reference: String,
    pub sale_id: u64
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTOnSale {
    pub token_id: TokenId,
    pub value: u128
}

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub enum MatchResult {
    PlayerOneWins,
    PlayerTwoWins,
    #[default]
    Draw
}

#[derive(Default, Encode, Decode, TypeInfo, Eq, PartialEq, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTCardType {
    #[default]
    Normal, 
    Fire, 
    Water,
    Grass, 
    Electric, 
    Ice, 
    Fight, 
    Poison,
    Ground,
    Flying,
    Psychic, 
    Bug,  
    Rock, 
    Ghost, 
    Dragon, 
    Dark, 
    Steel,
    Fairy
}

impl NFTCardType {
    pub fn batle(user1: Self, user2: Self) -> MatchResult {
        let result1 = user1.is_waek_to(user2.clone());
        let result2 = user2.is_waek_to(user1);
        if (result1 && result2) || !(result1 || result2) {
            MatchResult::Draw
        } else if result1 {
            MatchResult::PlayerTwoWins
        } else {
            MatchResult::PlayerOneWins
        }
    }
    
    pub fn is_waek_to(&self, card2: Self) -> bool {
        match *self {
            Self::Normal => {
                card2 == Self::Fight
            },
            Self::Fire => {
                card2 == Self::Water ||
                card2 == Self::Ground ||
                card2 == Self::Rock
            },
            Self::Water => {
                card2 == Self::Grass ||
                card2 == Self::Electric
            },
            Self::Grass => {
                card2 == Self::Fire ||
                card2 == Self::Ice ||
                card2 == Self::Poison ||
                card2 == Self::Flying ||
                card2 == Self::Bug
            },
            Self::Electric => {
                card2 == Self::Ground
            },
            Self::Ice => {
                card2 == Self::Fire ||
                card2 == Self::Fight ||
                card2 == Self::Rock ||
                card2 == Self::Steel
            },
            Self::Fight => {
                card2 == Self::Flying ||
                card2 == Self::Psychic ||
                card2 == Self::Fairy
            },
            Self::Poison => {
                card2 == Self::Ground ||
                card2 == Self::Psychic
            },
            Self::Ground => {
                card2 == Self::Water ||
                card2 == Self::Grass || 
                card2 == Self::Ice
            },
            Self::Flying => {
                card2 == Self::Electric ||
                card2 == Self::Ice ||
                card2 == Self::Rock
            },
            Self::Psychic => {
                card2 == Self::Bug ||
                card2 == Self::Ghost ||
                card2 == Self::Dark
            },
            Self::Bug => {
                card2 == Self::Flying ||
                card2 == Self::Rock ||
                card2 == Self::Fire
            },
            Self::Rock => {
                card2 == Self::Water ||
                card2 == Self::Grass ||
                card2 == Self::Fight ||
                card2 == Self::Ground ||
                card2 == Self::Steel
            },
            Self::Ghost => {
                card2 == Self::Ghost ||
                card2 == Self::Dark
            },
            Self::Dragon => {
                card2 == Self::Ice ||
                card2 == Self::Dragon ||
                card2 == Self::Fairy
            },
            Self::Dark => {
                card2 == Self::Fight ||
                card2 == Self::Bug ||
                card2 == Self::Fairy
            },
            Self::Steel => {
                card2 == Self::Fire ||
                card2 == Self::Fight ||
                card2 == Self::Ground
            },
            Self::Fairy => {
                card2 == Self::Poison ||
                card2 == Self::Steel
            }
        }
    }
    
    pub fn string_to_type(card_type: &str) -> Result<Self, ()> {
        match card_type {
            "Normal" => {
                Ok(Self::Normal)
            },
            "Fire" => {
                Ok(Self::Fire)
            },
            "Water" => {
                Ok(Self::Water)
            },
            "Grass" => {
                Ok(Self::Grass)
            },
            "Electric" => {
                Ok(Self::Electric)
            },
            "Ice" => {
                Ok(Self::Ice)
            },
            "Fight" => {
                Ok(Self::Fight)
            },
            "Poison" => {
                Ok(Self::Poison)
            },
            "Ground" => {
                Ok(Self::Ground)
            },
            "Flying" => {
                Ok(Self::Flying)
            },
            "Psychic" => {
                Ok(Self::Psychic)
            },
            "Bug" => {
               Ok( Self::Bug)
            },
           "Rock" => {
               Ok(Self::Rock)
           },
           "Ghost" => {
               Ok(Self::Ghost)
           },
           "Dragon" => {
              Ok(Self::Dragon)
           },
           "Dark" => {
               Ok(Self::Dark)
           },
           "Steel" => {
               Ok(Self::Steel)
           },
           "Fairy" => {
               Ok(Self::Fairy)
           },
           _ => {
               Err(())
           }
        }
    }
}