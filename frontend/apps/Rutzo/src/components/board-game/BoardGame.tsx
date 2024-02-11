import {Card, Facedowncard, PlayButton, EmptySlot} from "@/components";

import "./slide-in.css";
import "./fire.css";
import "./selectedCards.css";
import "./MainGame.css";
import useGameState from "@/hooks/useGameState";



import "./BoardGame.css";

import CardsContainer from "@/components/deck-container/CardsContainer";

function BoardGame() {
    const {
        userPressPlayButton,
        tokensForOwnerState,
        selectedCards,
        cardToPlay,
        //nftsLoaded,
        //userInMatch,
        matchInProgress,
        //actualUserInMatch,
        enemyCard,
        userWonTheMatch,
        handlePlayButton,
        cardSelected,
        addCardToPlay,
        removeCardToPlay,
        //resetBoard,
    } = useGameState();
  
    const containerStyles = {

      justifyContent: "center",
      alignItems: "center",
    };

    const cardsNumber: number = 3;

    const isButtonDisabled = selectedCards.length !== cardsNumber;

    return (
      <div>
        <div className="flex flex-col items-center  ">

            <CardsContainer
                className="bg-acrylic"
                title="My Collection"
            >
                    <div className={"flex flex-row justify-around w-full "}> {/* Añadir un div contenedor o usar Fragment */}
                        {tokensForOwnerState.map((element: any) => {
                            const [nftId, elemento] = element;
                            return (
                                <Card
                                    image={elemento.media}
                                    title={elemento.name}
                                    type={elemento.description.toLowerCase()}
                                    value={elemento.reference}
                                    key={nftId}
                                    onCardClick={() => {cardSelected(nftId, false)}}
                                    children={undefined}
                                />
                            );
                        })}

                        {tokensForOwnerState.length < 3 &&
                            Array.from(Array(3 - tokensForOwnerState.length).keys()).map((index) => (
                                <EmptySlot key={`empty-${index}`} /> // Modificado para tener una clave más única
                            ))
                        }
                    </div>

            </CardsContainer>
            <CardsContainer
                className="bg-green-acrylic"
                title="Selected Cards"
            >
                <div className={"flex flex-row justify-around w-full"}>
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
                    {selectedCards.length < 3 &&
                        Array.from(Array(3 - selectedCards.length).keys()).map((index) => (
                            <EmptySlot key={`empty-${index}`} /> // Modificado para tener una clave más única
                        ))
                    }
                        </div>


                        <div>
                        <br/>
                            <a
                                className={`btn-primary ${isButtonDisabled ? 'btn-disabled' : 'btn-primary'}`}
                                href={!isButtonDisabled ? "#gamearea" : ""}
                                onClick={e => isButtonDisabled && e.preventDefault()}
                            >
                                Let's Go!
                            </a>
                        </div>


            </CardsContainer>


          <div style={{ ...containerStyles, flexDirection: "column" }} className={"w-full flex flex-col items-center "}>

            <div id="gamearea" className="playArea flex flex-col items-center">
                <h2 className="text-2xl my-4 font-semibold ">Play</h2>

                {/*Red tones*/}
                <CardsContainer
                  className = "border-rose-900 from-pink-600 to-rose-900"
                  title="Enemy Cards"
                >
                  <div className={"flex flex-row justify-around w-full"}>
                    <Facedowncard />
                    <Facedowncard />
                    <Facedowncard />
                  </div>
              </CardsContainer>
              <CardsContainer
                className = "border-violet-800 from-blue-800 to-rose-800"
                title="Match"
              >
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
                            onClick={() => {console.log("me clockearon D:");
                            }}
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
              </CardsContainer>

              <CardsContainer
                className = "border-violet-800 from-sky-500 to-blue-800"
                title="Your Cards"
              >
                <div className="grid-row">
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
              </CardsContainer>
            </div>
          </div>
        </div>
      </div>
    );
}

export { BoardGame };
