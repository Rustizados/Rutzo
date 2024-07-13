import React, { useState, useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { Card, Facedowncard, PlayButton, EmptySlot } from "@/components";
import "./slide-in.css";
import "./fire.css";
import "./selectedCards.css";
import "./MainGame.css";
import useGameState from "@/hooks/useGameState";
import "slick-carousel/slick/slick.css";
import "slick-carousel/slick/slick-theme.css";
import "./BoardGame.css";
import CardsContainer from "@/components/deck-container/CardsContainer";

function BoardGame2() {
  const location = useLocation();
  const navigate = useNavigate();
  const selectedCards = location.state?.selectedCards || [];

  const {
    userPressPlayButton,
    tokensForOwnerState,
    cardToPlay,
    nftsLoaded,
    userInMatch,
    matchInProgress,
    actualUserInMatch,
    enemyCard,
    enemyCardCount,
    userWonTheMatch,
    handlePlayButton,
    cardSelected,
    enemyName,
    addCardToPlay,
    removeCardToPlay,
    setUserWonTheMatch,
    resetBoard,
  } = useGameState();

  const [isPlayerTurn, setIsPlayerTurn] = useState(true);
  const [timeLeft, setTimeLeft] = useState(180); // 3 minutos en segundos
  const [showDialog, setShowDialog] = useState(false);

  const handleNewGame = () => {
    resetBoard();
    setIsPlayerTurn(true);
    setUserWonTheMatch(null);
    navigate("/selection");
  };

  const handleGoHome = () => {
    resetBoard();
    setIsPlayerTurn(true);
    setUserWonTheMatch(null);
    navigate("/");
  };

  let timer: NodeJS.Timeout;

  useEffect(() => {
    if (showDialog && timeLeft > 0) {
      timer = setInterval(() => {
        setTimeLeft((prev) => prev - 1);
      }, 1000);
    } else if (timeLeft === 0) {
      clearInterval(timer);
      setShowDialog(false);
      navigate("/play");
    }
    return () => clearInterval(timer);
  }, [showDialog, timeLeft, navigate]);

  const cancelMatch = () => {
    setShowDialog(false);
    setTimeLeft(180);
  };

  const isButtonDisabled = selectedCards.length !== 3;

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white p-2">
      <div className="w-full max-w-5xl flex justify-between items-center mb-4">
        <div className="flex items-center mt-10">
          <img
            src="https://www.rutzo.tech/NFT/lightning/nova_lighting.jpg"
            alt="Player Avatar"
            className="w-10 h-10 rounded-full mr-2"
          />
          <p className="text-sm w-96 overflow-hidden whitespace-nowrap truncate">
            {actualUserInMatch}
          </p>
        </div>
        <div className="flex items-center">
          <p className="text-sm w-96 overflow-hidden whitespace-nowrap truncate mr-2">
            {enemyName}
          </p>
          <img
            src="https://www.rutzo.tech/NFT/poison/angel_of_death_poison.jpg"
            alt="Enemy Avatar"
            className="w-10 h-10 rounded-full"
          />
        </div>
      </div>

      <div className="flex justify-center mb-2 bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">
        <p className="text-sm p-2 font-bold">
          {isPlayerTurn ? "Your Turn" : "Enemy's Turn"}
        </p>
      </div>

      <div className="w-full max-w-5xl grid grid-cols-2 gap-4 mb-8">
        <div className="flex flex-col items-center">
          <div className="bg-slate-900 rounded-lg mb-4 p-2 flex items-center justify-center">
            {cardToPlay ? (
              <Card
                image={cardToPlay[1].media}
                title={cardToPlay[1].name}
                type={cardToPlay[1].description.toLowerCase()}
                value={cardToPlay[1].reference}
                onCardClick={() => removeCardToPlay(cardToPlay)}
                scale={1}
              />
            ) : (
              <Facedowncard scale={1.0} />
            )}
          </div>
          <div className="flex justify-center space-x-2">
            {selectedCards.map((card: [string, any]) => {
              const [nftId, elemento] = card;
              return (
                <Card
                  key={nftId}
                  image={elemento.media}
                  title={elemento.name}
                  type={elemento.description.toLowerCase()}
                  value={elemento.reference}
                  onCardClick={() => addCardToPlay(card)}
                  scale={0.6}
                />
              );
            })}
            {selectedCards.length < 3 &&
              Array.from(Array(3 - selectedCards.length).keys()).map(
                (index) => <EmptySlot key={`empty-${index}`} />
              )}
          </div>
        </div>
        <div className="flex flex-col items-center">
          <div className="bg-slate-900 rounded-lg mb-4 p-2 flex items-center justify-center">
            {enemyCard ? (
              <Card
                image={enemyCard.media}
                title={enemyCard.name}
                type={enemyCard.description.toLowerCase()}
                value={enemyCard.reference}
                scale={1}
              />
            ) : (
              <Facedowncard scale={1} />
            )}
          </div>
          <div className="flex justify-center space-x-2">
            {Array.from({ length: enemyCardCount }).map((_, index) => (
              <Facedowncard key={`facedown-${index}`} scale={0.6} />
            ))}
          </div>
        </div>
      </div>

      {userWonTheMatch !== null && (
        <div className="fixed inset-0 flex flex-col items-center justify-center bg-black bg-opacity-75">
          <div className="bg-black text-white p-8 rounded-lg text-center mb-4 border-2 border-violet-800">
            {userWonTheMatch ? (
              <h2 className="text-lg">You won! 🎉</h2>
            ) : (
              <h2>You lose ):</h2>
            )}
            <div className="flex space-x-4 mt-8">
              <button
                onClick={handleNewGame}
                className="px-6 py-2 bg-gradient-to-r from-purple-800 to-green-500 text-white rounded-full shadow-md hover:shadow-lg"
              >
                New Game
              </button>
              <button
                onClick={handleGoHome}
                className="px-6 py-2 bg-gradient-to-r from-purple-800 to-green-500 text-white rounded-full shadow-md hover:shadow-lg"
              >
                Home
              </button>
            </div>
          </div>
        </div>
      )}

      {showDialog && (
        <div className="fixed inset-0 flex items-center justify-center bg-black bg-opacity-75">
          <div className="bg-black text-white p-8 rounded-lg text-center border-2 border-violet-800">
            <h2 className="text-lg mb-3">Searching for an opponent...</h2>
            <p>
              Time left: {Math.floor(timeLeft / 60)}:
              {(timeLeft % 60).toString().padStart(2, "0")}
            </p>
            <div className="flex justify-center mt-8">
              <button
                onClick={cancelMatch}
                className="px-6 py-2 bg-gradient-to-r from-red-800 to-red-500 text-white rounded-full shadow-md hover:shadow-lg"
              >
                Cancel Match
              </button>
            </div>
          </div>
        </div>
      )}

      <div className="fixed bottom-4 right-4">
        <button
          className={`px-6 py-2 rounded-full shadow-md ${
            isButtonDisabled ? "bg-gray-500" : "bg-green-500 hover:bg-green-700"
          }`}
          onClick={(e) => {
            if (isButtonDisabled) {
              e.preventDefault();
            } else {
              handlePlayButton();
            }
          }}
          disabled={isButtonDisabled}
        >
          Let's Go!
        </button>
      </div>
    </div>
  );
}

export { BoardGame2 };
