import React, {useEffect} from 'react';

import useLocalBoard from "@/hooks/useLocalBoard";
import {Card} from "@/components";

const Selection = () => {
	const {pushCard, removeCard, clearSelectedCards, board} = useLocalBoard();

		return (
				<div className='mx-32'>
					<h1 className='title text-4xl font-extrabold dark:text-white'>Choose your cards</h1>
					<h2>My Collection</h2>
					<div>
						{
							board.availableCards.map((card: any) => {
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
								)
							})
						}
					</div>
					<hr/>
					<br/>
					<h2>Selected Cards</h2>
					<div>
						{
							board.selectedCards.map((card: any) => {
								const [nftId, elemento] = card;
								console.log(card, "card")
								return (
									<Card
										image={elemento.media || "https://via.placeholder.com/150"}
										title={elemento.name}
										type={elemento.description}
										value={elemento.reference}
										key={nftId}
										onCardClick={() => removeCard(card)}

									/>
								)
							})
						}
					<button onClick={clearSelectedCards}>Clear</button>
					</div>
				</div>
		)
}

export default Selection;
