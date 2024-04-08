//import {Card, Facedowncard, PlayButton} from "@/components";


const Fight = () => {
    return (
        <div>
            <div>Test</div>
            {/*<div id="gamearea" className="playArea">*/}
            {/*    <div className="grid-row opponentArea areaBorder area">*/}
            {/*        <Facedowncard />*/}
            {/*        <Facedowncard />*/}
            {/*        <Facedowncard />*/}
            {/*    </div>*/}
            {/*    <div className="versusArea areaBorder area">*/}
            {/*        <div className="grid-row">*/}
            {/*            /!* <CardComponent selectedCard={cardToPlay} /> *!/*/}
            {/*            {*/}
            {/*                cardToPlay && (*/}
            {/*                    <Card*/}
            {/*                        image={cardToPlay[1].media}*/}
            {/*                        title={cardToPlay[1].name}*/}
            {/*                        type={cardToPlay[1].description.toLowerCase()}*/}
            {/*                        value={cardToPlay[1].reference}*/}
            {/*                        onCardClick={() => {removeCardToPlay(cardToPlay)}}*/}
            {/*                    />*/}
            {/*                )*/}
            {/*            }*/}
            {/*            {*/}
            {/*                enemyCard ? (*/}
            {/*                    <Card*/}
            {/*                        image={enemyCard.media}*/}
            {/*                        title={enemyCard.name}*/}
            {/*                        type={enemyCard.description.toLowerCase()}*/}
            {/*                        value={enemyCard.reference}*/}
            {/*                    />*/}
            {/*                ) : (*/}
            {/*                    <Facedowncard />*/}
            {/*                )*/}
            {/*            }*/}

            {/*        </div>*/}

            {/*        <div className="buttonArea ">*/}
            {/*            {cardToPlay && (*/}
            {/*                <div>*/}
            {/*                    {*/}
            {/*                        !userPressPlayButton ? (*/}
            {/*                            <PlayButton*/}
            {/*                                onJoiningGame={() => handlePlayButton()} // {setUserPressPlayButton(true)}}*/}
            {/*                                //onPressed={() => {setUserPressPlayButton(true)}}*/}
            {/*                                tokenId={Number(cardToPlay[0])}*/}
            {/*                            />*/}
            {/*                        ) : (*/}
            {/*                            <div>*/}
            {/*                                {*/}
            {/*                                    !matchInProgress ? (*/}
            {/*                                        <div>*/}
            {/*                                            {*/}
            {/*                                                userWonTheMatch !== null ? (*/}
            {/*                                                    <div>*/}
            {/*                                                        {*/}
            {/*                                                            userWonTheMatch ? (*/}
            {/*                                                                <h2>You WON!!</h2>*/}
            {/*                                                            ) : (*/}
            {/*                                                                <h2>You lose :c</h2>*/}
            {/*                                                            )*/}
            {/*                                                        }*/}
            {/*                                                    </div>*/}
            {/*                                                ) : (*/}
            {/*                                                    <h2>Draw!</h2>*/}
            {/*                                                )*/}
            {/*                                            }*/}
            {/*                                        </div>*/}
            {/*                                    ) : (*/}
            {/*                                        <h2>Searching oponent...</h2>*/}
            {/*                                    )*/}
            {/*                                }*/}
            {/*                            </div>*/}
            {/*                        )*/}
            {/*                    }*/}

            {/*                </div>*/}
            {/*            )}*/}
            {/*        </div>*/}
            {/*    </div>*/}
            {/*    <div className="grid-row youArea areaBorder area">*/}
            {/*        {selectedCards.map((card: any) => {*/}
            {/*            const [nftId, elemento] = card;*/}
            {/*            return (*/}
            {/*                <Card*/}
            {/*                    image={elemento.media}*/}
            {/*                    title={elemento.name}*/}
            {/*                    type={elemento.description.toLowerCase()}*/}
            {/*                    value={elemento.reference}*/}
            {/*                    key={nftId}*/}
            {/*                    onCardClick={() => addCardToPlay(card)}*/}
            {/*                />*/}
            {/*            );*/}
            {/*        })}*/}
            {/*    </div>*/}
            {/*</div>*/}
        </div>
    );
}

export {Fight};
