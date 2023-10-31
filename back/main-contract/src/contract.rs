use gear_lib::non_fungible_token::token;
use gstd::{
    prelude::*, 
    msg, 
    BTreeMap
};
use program_io::*;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut CONTRACT: Option<ContractData> = None;

#[no_mangle]
extern "C" fn init() {
    let config: InitContractData = msg::load()
        .expect("Error in decoding init message 'InitContractData'");
    unsafe {
        CONTRACT = Some(ContractData {
            owner: msg::source(),
            nft_contract: config.nft_contract,
            tokens_metadata_default: config.tokens_metadata_default
                .into_iter()
                .enumerate()
                .map(|(index, data)| (index as u8, data))
                .collect(),
            ..Default::default()
        });
        /*
        CONTRACT = Some(ContractData { 
            owner: msg::source(),
            match_contract: config.match_contract,
            nft_contract: config.nft_contract,
            users: BTreeMap::new(),
            games: BTreeMap::new(),
            tokens_metadata_default: config.tokens_metadata_default,
            approved_minters: Vec::new(),
            transaction_id: 0
        });
        */
    };
}


#[gstd::async_main]
async fn main() {
    let action = msg::load().expect("Error in loading message");
    let state = state_mut();
    match action {
        RutzoAction::PlayGame {
            token_id,
            power
        } => {
            let message = state.play_game(msg::source(), token_id, power.parse::<u8>().expect("Error parsing")).await;
            msg::reply(message, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::MintCard {
            token_id
        } => {
            msg::reply(state.mint_card(token_id).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SetNFTAddress(address) => {
            let user_id = msg::source();
            if user_id != state.owner {
                msg::reply(RutzoEvent::UserIsNotApproved(user_id), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.nft_contract = Some(address);
            
            msg::reply(RutzoEvent::NFTContractSaved, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::Register => {
            msg::reply(state.register_user(msg::source()), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },        
        
        
        /*
        RutzoAction::MintCard {
            token_metadata
        } => {
            msg::reply(state.mint_card(token_metadata).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::AddMinter(user_id) => {
            msg::reply(state.add_minter(user_id), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SetNFTAddress(address) => {
            let user_id = msg::source();
            if user_id != state.owner {
                msg::reply(RutzoEvent::UserIsNotApproved(user_id), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.nft_contract = address;
            
            msg::reply(RutzoEvent::NFTContractSaved, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::Register => {
            msg::reply(state.register_user(msg::source()).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::PlayGame {
            token_id,
            power
        } => {
            let user_id = msg::source();
            let power = power as u8;
            
            msg::reply(state.play_game(user_id).await, 0)
                .expect("Error in sending message 'RutzoState'");
        },
        RutzoAction::QuitGame => {
            msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
        },
        RutzoAction::GetCards => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");

        },
        RutzoAction::BuyCard => {
            msg::reply(RutzoEvent::PurchaseSucces, 0).expect("Error in sending reply message");
        },
        RutzoAction::EndGame => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");
        },
        /*
        RutzoAction::TransferCard {
            from,
            to,
            matchId
        } => {
            msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
        },
        RutzoAction::DeleteMainContract => {
            let current_id = msg::source();
            if format!("{:?}", current_id) == MAINACTORID {
                exec::exit(msg::source());
            } else {
                msg::reply(RutzoEvent::LoginSucces, 0)
                    .expect("Error in reply");
            }
        },
        RutzoAction::DeleteMainContractSecond => {
            exec::exit(msg::source());
        },
        RutzoAction::DeleteContracts => {
            state.games
                .iter()
                .for_each(|game| {
                    let delete_contract_message = MatchContractMessage::DeleteContract;
                    msg::send(game.match_id, delete_contract_message, 0)
                        .expect("Error in sending message to match contract");
                });
            msg::reply(RutzoEvent::LoginSucces, 0)
                .expect("Error in reply message");
        }
        */
        _ => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");
        }
        */
    }
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(state_mut(), 0).expect("Error in sending reply state");
}



pub fn state_mut() -> &'static mut ContractData {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}



/*

[
    {
        "name": "Rusty Robot",
        "description": "Robot nft",
        "media": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMMAAAETCAMAAAC86EIDAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAADSUExURUdwTP///////////////////////////////////////////////xcROnhB9v////HGRMTUu1NeUXhB8Wt+ZTxHOXg//DQwUnh+eWttgV9qYZehkSUeSH6xdEdQQ16DV2eZXF9kdkdOV4TGdnVD6oWQgVJ2S+Li5i8dZ0ZmPzA2MlBPa6iqt4mJm6e1oXd6lG9D3MrTwvHw83ZkO2RBwsLBzEE7YkIqh1o+qLXErtu1Q9TX1piZqFRFjB8YQJZ+P5nfiLGdR7v4qvP36REVEN/KbicokooAAAANdFJOUwDpj0IwCioDIBC9pmT6VjRlAAATb0lEQVR42uyaD2+bvBbGl7Zrd9uFCWOIY4P5E0wTSChMNIBSqmrp9/9M7zkmbdc1u/fqXbYSaUdRVKmJ5F+e85xzbPPhw764GGh8uPjw/8TFxcePH/8z0IClfby4+F8AF7j+T8MNDfJfMFACBDg/PxtonJ+fIwZQ/BwBAc4+n1yORl+GGKPR5cnnM8T4iRSYRZ/OT09Gt3fhxB0PMdx1eLcZXZ72FPtFOD+9+rqajIcdk+U1UuyBACd8OjsZrdzx8GOyGp2cv8knrcLZ1fV6fBwxub46+1EJRDi9XLrjYwn3dnT6GkKrMFqOjymWl6+UuLgAO1/djo8rllfgiReGj5/OT67dI2Nwr8HYT0JoM4wm42OL9XeWwEy6XI2PL1ZXz0KgDNeTI2Rwv57uHIFuuFyOjzFWJ+d9Ml1gXZ0cJcNktCtNwPB5Mz7OuP2sGaAqnZ/cHSnDHSYTWhrsEB4pQ3ipKxNaerQ+UgY0BDJAdxi5R8rg9qbGsvRlfKzx5ewvw1+Gvwx/Gf4yHB/D/byu53VTz4+WYd7ERTeN1pVhFI17jAxu5VhFx3Il11VsOMbvofidDPd17FhVx1qz5WTSxIZlGPX9UTHUhWEAwqo1S1MlZA0QhuEU9fEwzCvDcowqW+UmhOA+yZrYsgzHKdzjYLhvLMeytBdMHZwmvKtikAI4DmyL38IARjBwsUUzFa1GKCmlvsoKw4ljkCeuh84wByM4jmHEVaa0DKVZCmRgHZg8tg5ti8MzgBF2EXerJxnMklAaiLACSziW/m/lDpXhvunXbxVx3IQ7BIhrYKCC1U0TO3EP4RzKFgdm0EbAKJqi6JRACXRsKDKoKIWO/fQZ40AJdVAGNAJUT3gVzSrLQrFzA76BIagneGqvCwfs7uDbgSgOyOBqI1j4ipu7VtlM5E+pZJY51wy2nUHZxUwy+ow6QEIdjOG+7vND/8hFJ0xB2IsdQAxtakpSUle68D4llFO7A2F4NgIUf8uoQs/MIf9fGHRhooImtszws1as1TAOUWcPw/BST5EBUkkFZqv49wybXgd/kfIMK+wLA5p7/t4M7gtBnyXFjQhyMyevGDiammIycZwGXzH8mi1+ncF9TqNdZ4P2tTKVMlvG8vKFQU8b8JYu+LxyoF+/+hbY4v0YsJ6+CjBrswSG1sy5MF8xBNgkbDuB0gQIL7be2cJ9J4b6BwLsDlYHux4qzFwRlbffMehY2HLa6ZHD+vG79bswvEEwcNYDBlPQVqk8V0xXp7J8ZiCphAHWeQPw7yF+jcHdtxCjmMKqW0p3XVowlZclTt/UJ9JOUymiyjFee7pPw3j+5xmKfQhWddOCl9XODLB8wZXOJWgOhBA7VbyJ4z0MgH//pxnqvTIYjdbhyc9laXpBIECHJCALZLBttq724v+76vRLDPvWgWWJQ03Nn+pqWeb+g5coKb1vFBAgnWySras9nsbKfP9nGfbK4MC+gPlQmIBhR5Ens0Dkgj7MFj2DpGHWGMahbP0rDNXedLDihgWeKTaw8ekhWmBQwOB5tmZIbcqz2tqfiNWfZYj3L6Lp7gLK8q0oNzshfEglwUgQPDFIHtX7v23Ef5Rhvn8NTTZRIqfbjdhuyh7C8xOmgEGnEiEpGOKnDIY7AIaOcxua27bMt3met3jK1yacMcYXIIOUEoqrbcs63u8HYz4EBuxkrN2W5ZYJBSHalhHOubSJ5EwIYUuZynX8E0O8P4MFOlDKZb7dbEEHgaFg4YwChFSKEhnZ0KyHzIC55ENL5l+vtyCFWba5opICDGMyJbIvruDquhi0Dj71KRfb7VftaAHbaqYZwM3kGBigLmkGSk3QAfobU21ZKtEzoCVwYEq1pwfLYDSZZkgCb9sT4OESOJtxrnu0lgEZnOEyVBnRY/ZDgBuI3eGSgv6ADDjxSdgG2XZdDJehaGCRgODPAvq8FYXSxHYM0CHUArJpUg03l4paaghuL9qnoe8VAyiRQuCt0FDrUuXCICE5JQkFGAWJ1EJ5xQMNzRDB8gHCRobB6lC5JAqhI9g+JY+Pj3nOKGNgaahLHIpS+viYyjRL7awynMEyrCVMFjRFY8OCOcO9NIbu2BKwYA8U2aQrjAH3aZtQQhl6AoXggumBQ1DGoTjZjza0a9sOh9wfnGoiJZvmdCeEKhn4geUw+qkScgq6nB2lMqusAfe4uMsIYXQKcqAQYANwNFP4jpLA+Cp1Kg2YwbKKOgNLMCITnFXLVpRlnm/gXW2EEtxOIynx6HvAfrDA1hHlK4UzaouTdwt+bhkMGziEw7QEydTF1oD9YOG99DyCBUuSEIWHNHcwteZC9XsJsLTEi0VryH7AJzKaObQ5fah3ewubiBbanOAcKNQ0jSZzF5+hsZxBM4AStbter2HzzHBLneOOtN/RyUldxXjIN2QG/WiMFRdVUVT1epIqTKS2NVuNkNVVgYt3Bp1LVqxXZ2k5iqaehKzXQCgedVXcX07/RIPB5JK1+4lhpU7czNe7cTUi4aTqn/3p76aHuxd1+icAHI1iFV0IHY9z6BScRFlToD4O3ooazsAZ8PnDuKqKuJpEnN1MqZzi8JqtG7QDsGkIjfOm1w3D03qFcTUNw66eRBG8oG1zDn9NuFrj5TQ+EGHF8avnBQbFoFcYNx0JvAAGiyiLIs5ZBGGnicdXlTY8ZpThWG+vg4bgaV1c4+pmJfwkCBY2Lj4jTLOkvs+WNwVGrDn2tIkh+AF/ZaPohMpz4fuejxCc9DrYQaLEXddlkGYNJJWzp08MggHN0NwwPNRoReAlAEH6XAIZsONB+078gN10+lJuWAy66EOHhuiAQd9l5cHDguz8kCaJ2ORiCrbwgyCYtTdNMZDaau0mhxiiaKbQlhW1J7bdP+mQB7OEhxSmWEmogqlDgTHsRRLMZrMHr3t72vdeOlh96yqaVd7mXhB4UEjDkFB80GHm+YvA87zZzAtM2MaRcMpt0GGGECu8qB4AA/a0vhZN8wdYGKx2FpAohP7A8ZhMYOIAWPDgKT6FCENgwE/BR2+qYhgMuukWzQ0S9EE1wzSELn3b+kFyt1y2gisuewaZaB28mZdPi/fMJfc7BkSY9r/swwOg0CgLcbXwkkwRere8XTKGZkB5pjsG/HzevKcOT3e7+DgVJFIIKe/5s28QM4+AH/AHD0EJvNalSjGc/Yi0kS2SyPDwDSluine8232+Y7diHLIVGDcJHgDhQXdncEMEI/ciwfAT30+SxT/tnQ9XozoWwK2OfY6OSxdhIExammrLUoJ4QBinoMic4/f/Su/eAJ3qWlct0PZt0nMskkLvL/dPbigJqgt9A8LpM3SI37+BxL/d5m/sP+vYCtZwy4BhJtTwb9sduhPPg3TbRlfGggQIAa+Rh34iFAG4w8vru6vVK68d3+vwUBoTJtNX91Pw3NkQxbJnl8AARdPsIQZV/DO0scxc1x3pMBpic4xMoLTh5a+71d765rxbhuVNJ5BHAwN0vmDjj2MgGY70ka7qs9l4WFOUBLqr63McEY10iLeX6Dq/cGB0Uaui83t/Hm5qa7ohoAUQ/vFyCH8vx7plzUeuO7zE3BUxxoJBd23b1WHTtHQMTY/gO7/vbq42UsOG95H9p7amW9N2bREuRdB0Z6pt6+4YkOxKEwJCtzUAAf/WIEJhXw1e/cyWaPcMpTXBGOAOGEQKVPbS6L3Q4qURlbY0XiK4GmyakI9j1/1og09/r29g38Y9iSXEBeoBpR5WvTSazUwDhtKbS4/QUAFoTRhmXUtlk9kYHNue3v6rZtjOvaHQW9+IHx1QD3bd97oaEkEMKl16Jm4JtXR9BJ0F2JGtQpUKARZ4Zub9TeXPn77pu4H7vX9+v/h+QzSwpSqBsHXQAbj4dCwoQHwN+rXpRNURdAbRde5C/jqzJtAV1r9pXX1+BmMj96zffb+611xX5Kfgvbo6MlELP8COgAwILAbDCkuduPjz+jzKI4i9w0eA8O5vy2t+t1u9Z13Ep5s7ExngNZvhRT3IjKIIFGOPp2A1JhRiMlXV8Z5Ktyhyy1OhFhLDUg23W587ICiYisY00y3NtCaZPzHzRQQJ0nioWSC/eKkTfQbqmuWL3JxAMmh53iS5vdh4xl9jc2mogYpwVWjxkekHbLoYRNoM8zyNmILBRIcA49KipwJycBYEwb2R3F1sPJW3yXlZEDtHFqar5pRMc8EAHZuFDAgRQOItcj93UZgq84LbwEuMnzs0pwnLfKQnXjLxLNO6jhbFyMa81VZLNbBgMreAwbbmi1wDPZAg8IwmplM3PMePwtgAPDVgPBoUpmv/gP5ZY5pJCP5Ap1su2JKlLgrBcBc0s/JW43MtJ5DvsYAAQw5erFl+xs0czAm6uciaauocRtVFHkH6fe81NKe9+TmvFBgI6iEPfGJmnGfWU47ZU1So4MYaM9UiAj14jc3Kb2PuscGIdY0BNDCvA5956mIxfHwcFgWMpQOGdmWaXoPTwNuZx26wa+0pN7OMe8znPxaDAsZGRTHx4OVPQT9Jk0sjtDQXn5IfUyuLOSc8I6woGXKMRJMgmHrNLu7Q2poIRvYrdjj3CTh4zTABLwimrOl1AFtcm8LjISiCBZzng2J4GT1FOl4ha34lwzbXCKF+yn9xn6nR0xPkecVobkyS/VojBCnAfQkZqVHxBFqAtLsFgvbXzKGQYohrlKO2CLpZf2mON7QaD62dv6M1pB7aPLlcB0sySAbJIBkkg2SQDJJBMkgGySAZJMOuMxiifOoKhzjylUvFidEtA68e4hZ7H/4mpzwy5C9EZkpIO2VwlNBxhDRrIZjz6gM9DKU+9MXjb4iivKoII+ZtMYgTs1BZ+1QqX1HWMBB8DxQlexfDuv1NMaCg9HMM57GS7gZDgAxJKP7JHHySShCHSpoZLAZziWNO01RA+g57wcAFg5E5lWuArIkPx5a1JC43DS7OE9P2GFLFWba42JcpShorCvcrz6WK4q9KvtwyxMeTEPwCPscEgyO2g3NxmhCqUsqq4JG0wZD6vp+FQqAVBipEoCSBBgYbgBAaK6Fo5NBYMnA4lIfCREBq+GiKIQkZVrZ9ikr2zyn89d73WMoPM1TtHJy/ZPCe+4Mn9tR6EwxlST1RSc6rN7KyHaN6z8u3Fv0hJiQIy9CyaktgRykXv5nXPh1D6PL+yIF6IIQroV+6k9iPDU5WtitkETFa9mmvNPZVBhrESmnTNQO6PV+qofaHTKhnBxjQ2GkVnPDLK0GTeMXTz2mo8JWusGKgomdhZUVS2VK1HQjdidDltB9bPdFRld/uCwZWNn0oyLzaMUrzfhaXfKxGEioM0MCzLLd98aFEnNF72Rk2HVtjVAFIokB3AO2NSQ/PuPAT3OuEwAR+XgfWFQaodgSoAz0BmiTBJAS38dgUzpiW6Qh1MM4GzTPEwhFAohAbm+HXkBiFyzBiCX8Ve0VcT1fUgCKxqv8VQdepo5sHEuN5RKMbIqmMjaonWW2EBnPvZSZdukBC633UW5qv2Istnz3LvesN+vzz+H95xMrBZU3SRv/wkRK82yl3dxyXKh09oLFFBj829p5BXhOQDJJBMkgGySAZJINkkAySQTL8YxlG87ZqO2OYDxYP7dR2xxANBlE7td0xLAaDRTu1nTGMBlD0Nmo7YZhHUZQvUI5FHkUv3XOT2u4YhAh/yqK52u4Y8udyRM3VdugP0ZtibFLboU//EWQRNVvbHcND9FZLblLbYWxdmnXedG13DEUtR9F0bXe2JEQQcfKh2druGPTSIaPFq93tJrXdMcwXuWhDmi/mzdbKMZBkkAySQTJIBskgGSSDZJAMkkEySAbJIBkkg2SQDJLh/5OBEu6EygdLysnuMFA/jAPjw7OIExI7/o4wGM6nZwoY3DF2gSF51+SjdSUIk+0zGJsJAU1gbJ3BCTY06cDZ9jrr/pqJP9R5t2ixv2WGdT5JnXfbiLHBwghNMJB4fcR990lislWG9d2Ux0Tf54tJyV7w1nreAd8qQ7o2KIVKNWVSYecZzrL33+hgtsqw/kichuwomZFkBkH5s/XLctBwdxm8auJrqniG4b0xnVjZKsP6kAIMrFpNpFqJYFcZ1qc7wJBUenCUt2NUkm43LgVv+UOIiwww6os53usjE9luXFrfP2Bc8kIMSIymSgjDC7Kj/QNd2xsHqCHD5xmGI5KV702H1obypXjjsRjfdr5EG8hb5fhBjuOWPsk/PZ6Od2M8jY7txOTDzUmNIA79btdUe7uf4OlHLy+FDicbPxnlH3Wdr9+je4pAeyXDMTAYe8qQ9Ponx38dHADDKdtTBnYKDAfA8LV/mO0pQ3bY/1oynHyL95Qh/YbucCAC0546hAHuIBjQIQ79vWTgp2BKJcPXk6NwH6Or4RxVahDR9WwfFeGflpFVMIAieskeegOo4fjgoIboHzr7Zk3UOay9ofKI/hnfM4b4rA5KS2v6crpfEBxTpT9qqF2C7485UX569ByhhOifOfvS1SUOGtJzhIO/BMRhz98HCur3Dv8boYY4Ov382LizkOqHZ0evIZQQgqKXZizZTc+gBsvi3iESvIZQq+Kk3/92eNrrKbtYer3Tw29f+ieohFcRBARSAEb/y24WkOwECY7XEFTxCSwKOXa1gHTHa5WwqgwAQZSdKyjZ/wRYcuxseU3cvwFxbxVU7N/F6QAAAABJRU5ErkJggg==",
        "reference": ""
    },
    {
        "name": "Astonishing",
        "description": "Astonishing",
        "media": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMMAAAEXCAMAAAAneQAVAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAADnUExURUdwTA4ODh0dHf7+/re3txEREf39/bOzs9fX18DAwEJCQmNjY+vq7n5+fnt7e/Pz83l5eXp6ev39/fv7+9/f3+vr65mZmaysrNLS0ru7u7m5uf7G/xcROv7QuMXG//////+t2wMCA/3ngPHGRDUvUYX9wP+k1SUfRdTU1ko7YBEOE1FNbMXEzmpsiomIniEbH+/FsGhZdqimtTMsMGZRSkw7PZiWrn12j9OxoPOozribiap3loFqVbm56eKcxZl/dri2wqan0cqMsN22Qs3xmpV5QGe3kPjXjz1rU3qm6rqZQQACBkdwTJB48PEAAABNdFJOUwAZKS9DDCADEys+UvxiYrw0U5FEqW5tZpyKiv////////////////////////////////////////////////////////////////8A1c0hDAAAD91JREFUeNrsnVlvozoUgEfqqDNXQqrUat5IiCvfOpg9LAGSkEVX0xnx///P9TGQQCA0C4EgcR7SlEDiz2fx8THLt29V8uNB5duPb+cI7PrzgQVIvgJguz0/rvx8/gKDE8CeT48rwAEUJxGA4Onp1+uLID2mCC+vv4DjBMWPb5zg7VVSPQsT8RGFyJY3FV7eEopqJTy9vQg6Fh9b8IxTlCEShFdJJ+LjC9aF16eSPSUIL4Is9kOw8FLWBEN4E2ZE7IuQjfBWhGBqeH4SZmKfZCaAJooILxuxXzIDc9orAhBeBdIzBsId+0feGbDYN5El5hJpbOKWpIv9E31vTeANfVQDWNNbxgBqmIl9FP31KYmvPK7iXjJgKTUmxvBrKvZT1F8Jw8+fT69eTxm8Vz7OcXewespgvTxzRfx8/t6bXK+c+3GHYHr4LpGeMhDp+z+c4Z/vkthXGRgGhoFhYBgY+slARUJInxko1rer8Xi88hTaUwYF2p/K1qY9ZNByBKALjCjpFQOViwQAYSDH6hGD4Y3Lso1pFOGeMFC8GlcJRovJZIF7wEDj+bhaPGQH0cIiD88QV9pRIjGiiElMH5qBInt7EmFso9jAiqZpCnlUBqJQdMIV0tDk7T9tasWsYQZrQZFSh1DksR6PAUcLhJTxBdIIRZMMscO0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",
        "reference": ""
    }
]


[
    {
        "nft_type": "ice",
        "power": "20"
    },
   {
        "nft_type": "water",
        "power": "15"
    }
]


*/