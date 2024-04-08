import { useState } from 'react';

const rules = [
  {
    rule: 'Deck Requirement',
    description: 'Each player must have a minimum of 3 cards in their deck to participate in matches.',
  },
  {
    rule: 'Match Objective',
    description: 'The goal of each match is to defeat the opponent by strategically using your cards.',
  },
  {
    rule: 'Card Exchange',
    description:
      'When a player wins a match, they acquire a card from the opponent. Conversely, losing a match results in the loss of a card.',
  },
  {
    rule: 'Tie Resolution',
    description: 'In the event of a tie, neither player gains or loses cards.',
  },
  {
    rule: 'Gameplay Fairness',
    description:
      'Rutzo aims to provide a fair and balanced gaming experience where outcomes are determined by player strategies and card choices.',
  },
  {
    rule: 'Marketplace Transactions',
    description:
      'Players can buy, sell, and trade NFTs on the Rutzo marketplace. Rutzo does not guarantee the value of NFTs, as they are subject to market fluctuations.',
  },
  {
    rule: 'Player Responsibility',
    description:
      'Players are responsible for their actions, including financial transactions and adherence to game rules. Rutzo disclaims liability for unauthorized purchases and encourages players to protect their accounts and data.',
  },
];
function Rules() {
    const [currentRuleIndex, setCurrentRuleIndex] = useState(0);
  
    const handleNextRule = () => {
      setCurrentRuleIndex((prevIndex) => (prevIndex + 1) % rules.length);
    };
  
    const handlePrevRule = () => {
      setCurrentRuleIndex((prevIndex) => (prevIndex - 1 + rules.length) % rules.length);
    };
  
    const { rule, description } = rules[currentRuleIndex];
  
    return (
      <div className="m-2 sm:m-20 justify-center bg-gray-950 rounded-2xl">
        <h1 className="text-3xl md:text-5xl font-semibold p-10 md:p-16 text-center">
          <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">The Rules</span>
        </h1>
        <div className="flex flex-col items-center m-5 sm:m-0">
          <h2 className="text-2xl md:text-5xl font-bold mt-2 text-center">{rule}</h2>
          <p className="mx-5 mt-5 mb-5 lg:mx-52">{description}</p>
          <div className="flex m-10">
            <button
              onClick={handlePrevRule}
              className="bg-gray-700 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded-l"
            >
              Previous
            </button>
            <button
              onClick={handleNextRule}
              className="bg-gray-700 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded-r"
            >
              Next
            </button>
          </div>
        </div>
      </div>
    );
  }
  
  export { Rules };