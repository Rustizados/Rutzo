import { ProgramMetadata } from "@gear-js/api";
import { useAccount, useApi } from "@gear-js/react-hooks";
import { MAIN_CONTRACT, NFT_CONTRACT } from "consts";
import { Card, Facedowncard, PlayButton } from "components";
import { useState, useEffect } from "react";
// import "./Collection.scss";

import "./slide-in.css";
import "./fire.css";
import "./selectedCards.css";
import "./MainGame.css";

function BoardGame() {
    const { api } = useApi();
    const { account } = useAccount();
    const [userPressPlayButton, setUserPressPlayButton] = useState(false);
    const [tokensForOwnerState, setTokensForOwnerState] = useState<any>([]);
    const [selectedCards, setSelectedCards] = useState<any>([]);
    const [cardToPlay, setCardToPlay] = useState<any | null>(null);
    const [nftsLoaded, setNftsLoaded] = useState(false);
    const [userInMatch, setUserInMatch] = useState(false);
  
    const mainContractMetadata = ProgramMetadata.from(MAIN_CONTRACT.METADATA);
    const nftContractMetadata = ProgramMetadata.from(NFT_CONTRACT.METADATA);
  
    const checkMatchStatus = async () => {
      if (!api) return;
      const stateResult = await api
        .programState
        .read({ programId: MAIN_CONTRACT.PROGRAM_ID, payload: { UserIsRegister: account?.decodedAddress ?? "0x0" } }, mainContractMetadata);
    
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
      if (userInMatch) return;
      setSelectedCards([card, ...selectedCards]);
      setCardToPlay(null);
    }
  
    const cardSelected = (tokenId: any, selected: boolean) => {
      if (!selected) {
        const nftSelected = tokensForOwnerState.find((token: any) => token[0] === tokenId);
        setTokensForOwnerState(
          tokensForOwnerState.filter((token: any) => token[0] !== tokenId)
        );
        setSelectedCards([nftSelected, ...selectedCards]);
        return;
      }
      const nftSelected = selectedCards.find((token: any) => token[0] === tokenId);
      setSelectedCards(
        selectedCards.filter((token: any) => token[0] !== tokenId)
      );
      setTokensForOwnerState([nftSelected, ...tokensForOwnerState]);
    }
  
    const setData = () => {
      if (!api || nftsLoaded) return;
      
      api.programState
        .read({ programId: NFT_CONTRACT.PROGRAM_ID, payload: { tokensForOwner: account?.decodedAddress ?? "0x0" } }, nftContractMetadata)
        .then((result) => {
  
          const nftStateFormated: any = result.toJSON();
        
          const tokensForOwner: [any] = nftStateFormated.tokensForOwner ?? [];
  
          setStateWithoutSelectedCards(tokensForOwner, selectedCards);
          // setTokensForOwnerState(tokensForOwner);
        })
        .catch(({ message }: Error) => {
          console.log(message);
        });
      setNftsLoaded(true);
    };
  
    setData();
  
    const containerStyles = {
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
    };
  
    useEffect(() => {
      if (!userPressPlayButton) return;
      console.log("Se inicio la partida!");
    }, [userPressPlayButton])
  
  
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
                  <Facedowncard />
                </div>
  
                <div className="buttonArea ">
                  {cardToPlay && (
                    <PlayButton 
                      onJoiningGame={() => {setUserPressPlayButton(true)}}
                      tokenId={cardToPlay[0]}
                    />
                    // <PlayGame
                    //   name={cardToPlay.name}
                    //   reference={cardToPlay.reference}
                    // />
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

