import {Card, Facedowncard, PlayButton} from "@/components";
import useGameLogic from "@/hooks/useGameLogic";
import { ReactComponent as GameController } from "@/assets/images/game_controller.svg";
import { useState } from "react";

// import "./Collection.scss";

import "./slide-in.css";
import "./fire.css";
import "./selectedCards.css";
import "./MainGame.css";

function BoardGame() {

    const [isPlayAreaVisible, setIsPlayAreaVisible] = useState(false);

    const {
        userPressPlayButton,
        tokensForOwnerState,
        selectedCards,
        cardToPlay,
        nftsLoaded,
        userInMatch,
        matchInProgress,
        actualUserInMatch,
        enemyCard,
        userWonTheMatch,
        handlePlayButton,
        addCardToPlay,
        removeCardToPlay,
        cardSelected,
        resetBoard
    }  = useGameLogic();
  
    const containerStyles = {
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
    };
  
    return (
      <div>
        <div className="mainContainer">

            <div className={`selection-area ${isPlayAreaVisible ? 'hidden' : ''}`}>

            <h3 className={"title titleGame"}>Time to choose! Pick three cards to start</h3>
          <div style={{ ...containerStyles, flexDirection: "column" }}>

            <h3 className="sectionTitle">My Collection</h3>
            <div className="area areaBorder myCardsArea">

              {tokensForOwnerState.length > 0 ? (

                          tokensForOwnerState.map((card: any) => {
                            const [nftId, elemento] = card;
                            return (
                              <Card
                                image={elemento.media}
                                title={elemento.name}
                                type={elemento.description.toLowerCase()}
                                value={elemento.reference}
                                key={nftId}
                                onCardClick={() => {cardSelected(nftId, false)}}
                              />
                            )
                          })


                  ): (
                <h3 style={{ fontSize: "1.5rem" }}>No NFTs</h3>
              )}

            </div>
            <h3 className="sectionTitle">My Selection</h3>
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
            </div>
              {selectedCards.length === 3 && (
                  <a className="buttonPrimary" href="#gamearea" onClick={() => setIsPlayAreaVisible(true)}>
                      <GameController /> Let's Go!
                  </a>
              )}
          </div>



          </div>

            <div id="gamearea" className={`playArea ${isPlayAreaVisible ? 'flex-display' : 'hidden'}`}>
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
    );
}

export { BoardGame };
