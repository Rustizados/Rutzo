import React from "react";
import { useNavigate } from "react-router-dom";
import useLocalBoard from "@/hooks/useLocalBoard";
import { Card } from "@/components";
import { FaTrash } from "react-icons/fa6";

const Selection = () => {
  const { pushCard, removeCard, clearSelectedCards, board } = useLocalBoard();
  const navigate = useNavigate();

  const handleLetsGo = () => {
    navigate("/fight", { state: { selectedCards: board.selectedCards } });
  };

  return (
    <div className="mx-32">
      <h1 className="text-3xl md:text-5xl font-semibold mb-6 text-center">
        Choose{" "}
        <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl p-1">
          your cards
        </span>
      </h1>

      <div className="flex justify-between items-center mb-4">
        <h2 className="text-2xl font-bold text-center bg-gradient-to-r from-purple-800 to-green-500 text-white rounded-lg p-1">
          Available Cards
        </h2>
      </div>

      <div className="flex flex-wrap justify-center">
        {board.availableCards.map((card: any) => {
          const [nftId, elemento] = card;
          return (
            <Card
              image={elemento.media}
              title={elemento.name}
              type={elemento.description.toLowerCase()}
              value={elemento.reference}
              key={nftId}
              onCardClick={() => pushCard(card)}
            />
          );
        })}
      </div>

      <hr className="my-8" />

      <div className="flex justify-between items-center mb-4">
        <h2 className="text-2xl font-bold text-center bg-gradient-to-r from-purple-800 to-green-500 text-white rounded-lg p-1">
          Selected Cards
        </h2>
        {board.selectedCards.length > 0 && (
          <button
            onClick={clearSelectedCards}
            className="flex items-center px-4 py-2 bg-gradient-to-r from-purple-800 to-green-500 text-white rounded-full shadow-md hover:shadow-lg"
          >
            <FaTrash className="w-4 h-4 mr-2" />
            Clear
          </button>
        )}
      </div>

      {board.selectedCards.length === 0 ? (
        <p className="text-center text-white">No cards selected</p>
      ) : (
        <div className="flex flex-wrap justify-center">
          {board.selectedCards.map((card: any) => {
            const [nftId, elemento] = card;
            return (
              <Card
                image={elemento.media || "https://via.placeholder.com/150"}
                title={elemento.name}
                type={elemento.description}
                value={elemento.reference}
                key={nftId}
                onCardClick={() => removeCard(card)}
              />
            );
          })}
        </div>
      )}

      <div className="flex justify-center mt-8">
        <button
          onClick={handleLetsGo}
          className="px-6 py-2 bg-gradient-to-r from-purple-800 to-green-500 text-white rounded-full shadow-md hover:shadow-lg"
          disabled={board.selectedCards.length === 0}
        >
          Let's go
        </button>
      </div>
    </div>
  );
};

export default Selection;
