import { ProgramMetadata } from "@gear-js/api";
import { useAccount, useAlert, useApi } from "@gear-js/react-hooks";
import { MAIN_CONTRACT, NFT_CONTRACT } from "@/app/consts";
import { Card, Facedowncard, PlayButton } from "@/components";
import { useState } from "react";
import { sleepReact } from "@/app/utils";
// import "./Collection.scss";

import "./slide-in.css";
import "./fire.css";
import "./selectedCards.css";
import "./MainGame.css";

function BoardGame() {
    const alert = useAlert();
    const { api } = useApi();
    const { account } = useAccount();
    const [userPressPlayButton, setUserPressPlayButton] = useState(false);  // -----
    const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]); // ----
    const [selectedCards, setSelectedCards] = useState<any>([]); // ----
    const [cardToPlay, setCardToPlay] = useState<any | null>(null); // -----
    const [nftsLoaded, setNftsLoaded] = useState(false);  // -----
    const [userInMatch, setUserInMatch] = useState(false);  // ------
    const [matchInProgress, setMatchInProgress] = useState(false);  // ------
    const [actualUserInMatch, setActualUserInMatch] = useState("0x00");  // ----
    const [enemyCard, setEnemyCard] = useState<any | null>(null);  // ----
    const [userWonTheMatch, setUserWonTheMatch] = useState<boolean | null>(false);  //-- --
  
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
    const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);

    const resetBoard = () => {
      setTokensForOwnerState([]);
      setSelectedCards([]);
      setCardToPlay(null);
      setUserInMatch(false);
      setMatchInProgress(false);
      setNftsLoaded(false);
      setUserPressPlayButton(false);
      setActualUserInMatch(account?.decodedAddress ?? "0x00");
      setUserWonTheMatch(false);
      setEnemyCard(null);
    }

    const ActualMatchOfUser = async (): Promise<number> => {
      if (!api) return -1;
      const stateResult = await api
        .programState
        .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { PlayerIsInMatch: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
      const stateFormated: any = stateResult.toJSON();

      return stateFormated.playerInMatch ?? -1;
    }

    const lastMatchOfUser = async (): Promise<number> => {
      if (!api) return -1;
      const stateResult = await api
        .programState
        .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { PlayerInformation: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
      const stateFormated: any = stateResult.toJSON();

      return stateFormated.playerInformation.recentPastGame ?? -1;
    }

    const setActualSelectedCardFromMatch = async (matchId: number) => {
      if (!api) return;
      const stateResult = await api
        .programState
        .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { GameInformationById: [matchId] } }, mainContractMetadata);
      
      const stateFormated: any = stateResult.toJSON();
      // console.log("Datos de la partida donde esta el usuario:");
      // console.log(stateFormated);

      const { user1 } = stateFormated.gameInformation;
      const tokenId = user1.chosenNft;

      if (tokensForOwnerState.length === 0) return;

      // console.log(tokensForOwnerState);
      const selectedNft = tokensForOwnerState.find((nft: any) => nft[0] === tokenId);

      setCardToPlay(selectedNft);

      setTokensForOwnerState(
        tokensForOwnerState.filter((nft: any) => nft[0] !== tokenId)
      );

      setMatchInProgress(true);
    }




























    const showMatchResults = (userAddress: `0x${string}`, matchData: any) => {
      console.log("INFO RECIBIDA PARA MOSTRAR!!!!!!");
      console.log(matchData);
      
      const matchStateData = matchData.matchState;
      const user1Data = matchData.user1;
      const user2Data = matchData.user2;
      const cardToShow = user1Data.userId === userAddress
        ? user2Data.nftData
        : user1Data.nftData;

      if (Object.keys(matchStateData)[0] !== "draw") {
        const wonTheMatch = matchStateData.finished.winner === userAddress;
        setUserWonTheMatch(wonTheMatch);
      } else {
        setUserWonTheMatch(null);
      }

      setEnemyCard(cardToShow);
    }

    const userWaitingMatch = async (matchId: number) => {
      if (!api) return;

      let matchFinished = false;

      /* eslint-disable no-await-in-loop */
      while (!matchFinished) {
        const stateResult = await api
          .programState
          .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [matchId] } }, mainContractMetadata);
        
        const stateFormated: any = stateResult.toJSON();
        const status = Object.keys(stateFormated)[0];
        if (status === 'matchDoesNotExists') {
          console.log("La partida no existe!!");
          break;
        }

        const matchState = Object.keys(stateFormated.matchState)[0];
        
        if (matchState !== 'inProgress') {
          matchFinished = true;
        }
      }
      
      const matchInformationStateResult = await api
        .programState
        .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { GameInformationById: [matchId] } }, mainContractMetadata);

      const matchInformationState: any = matchInformationStateResult.toJSON();

      showMatchResults(
        account?.decodedAddress ?? "0x00", 
        matchInformationState.gameInformation
      );
      setMatchInProgress(false);
      
      await sleepReact(4000);

      console.log("DESPERTOOOOOOOOOOOOOOO");

      const stateResult = await api
          .programState
          .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [matchId] } }, mainContractMetadata);
      const stateFormated: any = stateResult.toJSON();
      const matchState = Object.keys(stateFormated.matchState)[0];

      if (matchState === 'inProgress') {
        alert.error("Erron in contract, searching match");
        console.log("ERRRRRRRRRRRRRIIIIIIIIIIIIIIOOOOOOOOOOOOOORRRRRRRRRR la partida no concluyo de forma correcta!!");
        
        await userWaitingMatch(matchId);
        return;
      }
      
      resetBoard();
    }









    const handlePlayButton = async () => {
      if (!api) return;

      const matchId = await ActualMatchOfUser();
      console.log("Button El usuario esta en la partida:");
      console.log(matchId);

      setUserPressPlayButton(true);

      if (matchId !== -1) {
        setUserInMatch(true);
        setMatchInProgress(true);
        console.log("CREANDO FUNCION PARA ESPERA EN LA PARTIDA!!!!!!!!!!!!!!!!!!!");
        
        await userWaitingMatch(matchId);
        return;
      }

      console.log("El usuario ingreso a una partida ya comenzada!!!: ");


      setUserInMatch(true);

      const lastMatchId = await lastMatchOfUser();
      console.log("Ultima partida que jugo el jugador: ", lastMatchId);

      const matchInformationStateResponse = await api
        .programState
        .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { MatchStateById: [lastMatchId] } }, mainContractMetadata);

      const matchInformationState: any = matchInformationStateResponse.toJSON();
      console.log("MOSTRANDO INFORMACION DE LA PARTIDA YA EXISTENTE!!");
      console.log(matchInformationState);

      const matchState = Object.keys(matchInformationState.matchState)[0];

      if (matchState === 'inProgress') {
        console.log("El usuario sigue en partida!!!");
        alert.error("Erron in contract!, joined to match");
        resetBoard();
        alert.error("Error joining the game, try again!");
        return;
      }

      const stateResult = await api
          .programState
          .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { GameInformationById: [lastMatchId] } }, mainContractMetadata);
      const stateFormated: any = stateResult.toJSON();
      

      showMatchResults(
        account?.decodedAddress ?? "0x00", 
        stateFormated.gameInformation
      );
      setMatchInProgress(false);

      await sleepReact(4000);

      resetBoard();
    }






















  
    const setStateWithoutSelectedCards = (cards: [any], cardsSelected: [any]) => {
      const cardsLeft = cards.filter((card) => {
        const cardSelected = cardsSelected.find((selectedCard) => selectedCard[0] === card[0]);
        return cardSelected === undefined;
      });
      setTokensForOwnerState(cardsLeft);
    }
  
    const addCardToPlay = (card: any) => {
      if (userInMatch) return;
  
      const cardsSelected = selectedCards.filter((actualCard: any) => actualCard[0] !== card[0]);
  
      if (cardToPlay) cardsSelected.push(cardToPlay);
      setCardToPlay(card);
      setSelectedCards(cardsSelected);
    }
  
    const removeCardToPlay = (card: any) => {
      if (userInMatch || matchInProgress) return;
      setSelectedCards([card, ...selectedCards]);
      setCardToPlay(null);
    }
  
    const cardSelected = (tokenId: any, selected: boolean) => {
      if (!selected) {
        const nftSelected = tokensForOwnerState.find((token: any) => token[0] === tokenId);
        const actualSelectedCards = [nftSelected, ...selectedCards];
        let actualTokensCards = tokensForOwnerState.filter((token: any) => token[0] !== tokenId);
        if (actualSelectedCards.length > 3) {
          actualTokensCards = [actualSelectedCards.pop(), ...actualTokensCards];
        }
        setTokensForOwnerState(
          actualTokensCards
        );
        setSelectedCards(
          actualSelectedCards
        );
        return;
      }
      const nftSelected = selectedCards.find((token: any) => token[0] === tokenId);
      setSelectedCards(
        selectedCards.filter((token: any) => token[0] !== tokenId)
      );
      setTokensForOwnerState([nftSelected, ...tokensForOwnerState]);
    }
  
    const setData = async () => {
      if (!api) return;

      // await checkMatchStatus(0);
      // const lastMatch = await lastMatchOfUser();
      // console.log("USER INFORMATION: ");
      // console.log(lastMatch);
      if (actualUserInMatch !== account?.decodedAddress) {
        console.log("Se va a formatear el tablero este!");
        
        resetBoard();
      }
      
      if (!nftsLoaded) {
        console.log("CARGANDO NFTS");
        
        const resultNfts = await api.programState
          .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, nftContractMetadata);
    
        const nftStateFormated: any = resultNfts.toJSON();
      
        const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];

        setStateWithoutSelectedCards(tokensForOwner, selectedCards);
        // setTokensForOwnerState(tokensForOwner);

        setNftsLoaded(true);
      }

      if (!userInMatch) {
        const matchId = await ActualMatchOfUser();

        if (matchId !== -1) {
          await setActualSelectedCardFromMatch(matchId);
          setUserInMatch(true);
          setMatchInProgress(true);
        }
        setActualUserInMatch(account?.decodedAddress ?? "0x00");
      }
    };
  
    setData();
  
    const containerStyles = {
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
    };
  
    return (
      <div>
        <div className="mainContainer">
          <div style={{ ...containerStyles, flexDirection: "column" }}>
            <h3 className="sectionTitle">My NFT Collection</h3>
            <div className="area areaBorder myCardsArea">
              {tokensForOwnerState.length > 0 ? (
                tokensForOwnerState.map((element: any) => {
                  const [nftId, elemento] = element;
                  return (
                    <Card 
                      image={elemento.media}
                      title={elemento.name}
                      type={elemento.description.toLowerCase()}
                      value={elemento.reference}
                      key={nftId}
                      onCardClick={() => {cardSelected(nftId, false)}}
                    />
                  );
                })
              ) : (
                <h3 style={{ fontSize: "1.5rem" }}>No NFTs</h3>
              )}
  
            </div>
            <h3 className="sectionTitle">My NFT Selection</h3>
            <div className="area selectedCardsArea">
              <div className="selectedCards">
                {
                  selectedCards.map((card: any) => {
                    const [nftId, elemento] = card;
                    return ( 
                      <Card 
                        image={elemento.media}
                        title={elemento.name}
                        type={elemento.description.toLowerCase()}
                        value={elemento.reference}
                        key={nftId}
                        onCardClick={() => {cardSelected(nftId, true)}}
                      />
                    )
                  })
                }
              </div>
  
              {selectedCards.length === 3 && (
                <a className="buttonPrimary" href="#gamearea">
                  Go!
                </a>
              )}
            </div>
  
            <div id="gamearea" className="playArea">
              <div className="grid-row opponentArea areaBorder area">
                <Facedowncard />
                <Facedowncard />
                <Facedowncard />
              </div>
              <div className="versusArea areaBorder area">
                <div className="grid-row">
                  {/* <CardComponent selectedCard={cardToPlay} /> */}
                  {
                    cardToPlay && (
                      <Card 
                        image={cardToPlay[1].media}
                        title={cardToPlay[1].name}
                        type={cardToPlay[1].description.toLowerCase()}
                        value={cardToPlay[1].reference}
                        onCardClick={() => {removeCardToPlay(cardToPlay)}}
                      />
                    )
                  }
                  {
                    enemyCard ? (
                      <Card 
                        image={enemyCard.media}
                        title={enemyCard.name}
                        type={enemyCard.description.toLowerCase()}
                        value={enemyCard.reference}
                      />
                    ) : (
                      <Facedowncard />
                    )
                  }
                  
                </div>
  
                <div className="buttonArea ">
                  {cardToPlay && (
                    <div>
                      {
                        !userPressPlayButton ? (
                          <PlayButton 
                            onJoiningGame={() => handlePlayButton()} // {setUserPressPlayButton(true)}}
                            //onPressed={() => {setUserPressPlayButton(true)}}
                            tokenId={Number(cardToPlay[0])}
                          />
                        ) : (
                          <div>
                            {
                              !matchInProgress ? (
                                <div>
                                  {
                                    userWonTheMatch !== null ? (
                                      <div>
                                        {
                                          userWonTheMatch ? (
                                            <h2>You WON!!</h2>
                                          ) : (
                                            <h2>You lose :c</h2>
                                          )
                                        }
                                      </div>
                                    ) : (
                                      <h2>Draw!</h2>
                                    )
                                  }
                                </div>
                              ) : (
                                <h2>Searching oponent...</h2>
                              )
                            }
                          </div>
                        )
                      }
                      
                    </div>
                  )}
                </div>
              </div>
              <div className="grid-row youArea areaBorder area">
                {selectedCards.map((card: any) => {
                  const [nftId, elemento] = card;
                  return (
                    <Card
                      image={elemento.media}
                      title={elemento.name}
                      type={elemento.description.toLowerCase()}
                      value={elemento.reference}
                      key={nftId}
                      onCardClick={() => addCardToPlay(card)}
                    />
                  );
                })}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
}

export { BoardGame };





/*
    // useEffect(() => {
    //   if (!matchInProgress || userMatchId === -1 || !matchCreated || !cardToPlay) {
    //     console.log("Partida sin iniciar, cancelando use effect //////////////////////////////////////");
    //     console.log("CON CARTA:");
    //     console.log(cardToPlay);
    //     return;        
    //   }

    //   const resetBoardUseEffect = () => {
    //     console.log("SE VA A FORMATEAR TODOOOOOOOOOOOOOOOOOOOOOOOOOO USE EFEEEEEEEEEEEEEECT");
    //     setTokensForOwnerState([]);
    //     setSelectedCards([]);
    //     setCardToPlay(null);
    //     setUserMatchId(-1);
    //     setUserInMatch(false);
    //     setMatchInProgress(false);
    //     setNftsLoaded(false);
    //     setUserPressPlayButton(false);
    //     setActualUserInMatch(account?.decodedAddress ?? "0x00");
    //   }

    //   const checkMatchStatus = async () => {
        
    //   }

    //   console.log("LA PARTIDA VA A COMENZAR!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    //   console.log("PARTIDA A LA QUE SE UNIRA EL VATO: ", userMatchId);
    //   console.log("Se jugara con la carta: ");
    //   console.log(cardToPlay);
      
      
      
    //   checkMatchStatus();


    // }, [matchInProgress, cardToPlay, userMatchId, api, account, mainContractMetadata, matchCreated])
    */