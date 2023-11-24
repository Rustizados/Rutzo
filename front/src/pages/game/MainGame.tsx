import { NFT_CONTRACT } from "consts";
import React, { useState, useEffect } from "react";
import { decodeAddress, ProgramMetadata } from "@gear-js/api";
import { useApi, useAccount } from "@gear-js/react-hooks";
import { cloneDeep } from "lodash";
import { MySelectedNFT } from "./MySelectedNFT";
import { Card } from "../../components/card/Card";
import { Facedowncard } from "../../components/facedowncard/facedowncard";
import { PlayGame } from "./PlayGame";
import { GameStatus } from "./GameStatus";
import "./slide-in.css";
import "./fire.css";
import "./selectedCards.css";
import "./MainGame.css";

function InfoNFT({ name, description, media, reference, onCardClick }: any) {
  return (
    <Card
      image={media}
      title={name}
      type={description}
      value={reference}
      price={reference}
      onCardClick={onCardClick}
    />
  );
}

function CardComponent({ selectedCard }: any) {
  if (!selectedCard) {
    return <h3>Select your Card</h3>;
  }
  return (
    <MySelectedNFT
      name={selectedCard.name}
      description={selectedCard.description}
      media={selectedCard.media}
      reference={selectedCard.reference}
    />
  );
}

function MainGame() {
  const [selectedCards, setSelectedCards] = useState<any[]>([]);
  const [selectedCard, setSelectedCard] = useState();
  const [allNFTs, setAllNFTs] = useState([]);
  const [myNFTCollection, setMyNFTCollection] = useState([]);
  const { api } = useApi();
  const { account } = useAccount();

  const [cardToPlay, setCardToPlay] = useState<any>();

  // const [fullState, setFullState] = useState<any>([]);

  // const programIDNFT = "0x23fcd161c9b6c736cfb70fd7837c6dd66ea463c441d18fa7a1031c0af18fb0d0"

  const meta = NFT_CONTRACT.METADATA;

  const metadata = ProgramMetadata.from(meta);
  const currentAccount = account?.address;

  const selectCard = (card: any) => {
    // Crear una copia del estado actual para no mutar el estado directamente
    const updatedSelectedCards = cloneDeep(selectedCards);

    // Buscar si la carta ya ha sido seleccionada
    const cardIndex = updatedSelectedCards.findIndex(
      (item) => item.reference === card.reference
    );

    // Si la carta ya ha sido seleccionada, se elimina de la lista
    if (cardIndex !== -1) {
      updatedSelectedCards.splice(cardIndex, 1);
    } else {
      // Si no ha sido seleccionada y hay menos de 3 cartas, se añade a la lista
      // eslint-disable-next-line no-lonely-if
      if (updatedSelectedCards.length < 3) {
        updatedSelectedCards.push(card);
      } else {
        alert("You can only select 3 cards");
      }
    }

    setSelectedCards(updatedSelectedCards);
    const updatedMyNFTCollection = myNFTCollection.filter(
      (item) => !selectedCards.includes(item)
    );
    setMyNFTCollection(updatedMyNFTCollection);
  };

  const selectCardToPlay = (card: any) => {
    if (cardToPlay) {
      alert("You can only select 1 card to play");
    } else {
      setCardToPlay(card);
    }

    console.log("cardToPlay", cardToPlay);
  };

  useEffect(() => {
    async function getMyNFT() {
      const programIDNFT = NFT_CONTRACT.PROGRAM_ID;
      try {
        const result = await api.programState.read(
          { programId: programIDNFT, payload: "" },
          metadata
        );
        const fullState: any = result.toJSON();
        // Usando encadenamiento opcional y asignando valores predeterminados como arrays vacíos
        const tokensForOwner = fullState.token?.tokensForOwner ?? [];
        console.log("tokensForOwner", tokensForOwner);
        const tokenMetadataById = fullState.token?.tokenMetadataById ?? [];
        console.log("tokenMetadataById", tokenMetadataById);

        const allDataNFTs = tokenMetadataById.map((item: any) => item[1]);
        console.log("allDataNFTs", allDataNFTs);
        setAllNFTs(allDataNFTs);

        // Tomar el primer elemento del resultado de filter, que es un array de arrays con los tokens del owner
        const myNFTsArray = tokensForOwner.filter(
          (obj: any) => obj[0] === decodeAddress(currentAccount ?? "")
        );
        console.log("myNFTsArray", myNFTsArray);

        // Verificar si myNFTsArray tiene elementos antes de intentar acceder a ellos
        const myNFTs = myNFTsArray[0][1].length > 0 ? myNFTsArray[0][1] : [];
        console.log("myNFTs", myNFTs);

        const myNFTCollectionH = tokenMetadataById
          .filter((item: any) => myNFTs.includes(item[0]))
          .map((item: any) => item[1]);

        setMyNFTCollection(myNFTCollectionH);
      } catch (error: any) {
        console.error("Error fetching NFTs:", error);
      }
    }

    getMyNFT();
  }, [api.programState, currentAccount, metadata]);

  const cardStyles = { border: "none", background: "transparent" };
  const containerStyles = {
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
  };

  // @ts-ignore
  return (
    <div>
      <div className="mainContainer">
        <div style={{ ...containerStyles, flexDirection: "column" }}>
          <h3 className="sectionTitle">My NFT Collection</h3>
          <div className="area areaBorder myCardsArea">
            {myNFTCollection.length ? (
              myNFTCollection.map((elemento: any, index: any) => (
                <InfoNFT
                  key={elemento.name} // considera usar otro identificador único en lugar de randomUUID para una mejor performance
                  name={elemento.name}
                  description={elemento.description}
                  media={elemento.media}
                  reference={elemento.reference}
                  onCardClick={() => selectCard(elemento)}
                />
              ))
            ) : (
              <h3 style={{ fontSize: "1.5rem" }}>No NFTs</h3>
            )}
          </div>
          <h3 className="sectionTitle">My NFT Selection</h3>
          <div className="area selectedCardsArea">
            <div className="selectedCards">
              {selectedCards.map((card, index) => (
                // eslint-disable-next-line react/no-array-index-key
                <CardComponent key={index} selectedCard={card} />
              ))}
            </div>

            {selectedCards.length === 3 && (
              <a className="buttonPrimary" href="#gamearea">
                Go!
              </a>
            )}
          </div>

          <div id="gamearea" className="playArea">
            <div className="grid-row opponentArea areaBorder area">
              {[1, 2, 3].map((item: number) => (
                <Facedowncard key={item} />
              ))}
            </div>
            <div className="versusArea areaBorder area">
              <div className="grid-row">
                <CardComponent selectedCard={cardToPlay} />
                <Facedowncard />
              </div>

              <div className="buttonArea ">
                {cardToPlay && (
                  <PlayGame
                    name={cardToPlay.name}
                    reference={cardToPlay.reference}
                  />
                )}
                <GameStatus />
              </div>
            </div>
            <div className="grid-row youArea areaBorder area">
              {selectedCards.map((card, index) => (
                <InfoNFT
                  key={card.name}
                  name={card.name}
                  description={card.description}
                  media={card.media}
                  reference={card.reference}
                  onCardClick={() => selectCardToPlay(card)}
                />
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export { MainGame };
