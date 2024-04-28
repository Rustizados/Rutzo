import { useDispatch, useSelector } from 'react-redux';
import {increment} from "@/features/counterSlice";
import useGameState from "@/hooks/useGameState";
import useCardsData from "@/hooks/useCardsData";
import{EmptySlot} from "@/components";


import {useEffect, useState} from "react";

import {Card} from "@/components";


const Match = () => {
	const dispatch = useDispatch()
	const count = useSelector((state: any) => state.counter.value)
	const cards = useSelector((state: any) => state.cards.cards);

	const {allUserCards, fetchData, getPlayingCards, playingUserCards} = useCardsData();

	useEffect(() => {

		fetchData();
		getPlayingCards();

	}, [fetchData, getPlayingCards]);



	console.log("userCards:2", allUserCards)

	return(

		<div>
			{/*<h1>Match</h1>*/}
			{/*<div> {playingUserCards }</div>*/}

			{/*<div>*/}
			{/*	<button*/}
			{/*		aria-label="Increment value"*/}
			{/*		onClick={() => dispatch(increment())}*/}
			{/*	>*/}
			{/*		Increment*/}
			{/*	</button>*/}

			{/*	<div>*/}
			{/*		{allUserCards.map((card: any) => {*/}
			{/*			const [nftId, elemento] = card;*/}
			{/*			return (*/}
			{/*				<Card*/}
			{/*					image={elemento.media}*/}
			{/*					title={elemento.name}*/}
			{/*					type={elemento.description.toLowerCase()}*/}
			{/*					value={elemento.reference}*/}
			{/*					key={nftId}*/}
			{/*				/>*/}
			{/*			)*/}
			{/*		})}*/}

			{/*	</div>*/}
			{/*</div>*/}

			<div className="p-8 h-screen">
				<div className="grid grid-cols-3 gap-4">
					{/* User section */}
					<div>
						<div className="flex items-center mb-4">
							<div className="rounded-full bg-blue-300 border-2 border-black w-12 h-12 flex items-center justify-center">🧍</div>
							<div className="ml-2">
								<div className="text-lg  font-semibold">Nombre del usuario</div>
								<div className="flex">
									<div className="w-4 h-4 bg-green-500 rounded-full mr-1"></div>
									<div className="w-4 h-4 bg-green-500 rounded-full mr-1"></div>
									<div className="w-4 h-4 bg-green-500 rounded-full"></div>
								</div>
							</div>
						</div>
						<div className={"flex flex-row"}>
							{/* User cards */}
							<div>
							<EmptySlot/>
							</div>
							<div className="mt-24 -ml-32">
							<EmptySlot/>
							</div>
						</div>
					</div>
					{/* Winner section */}
					<div className="flex-grow text-center">
						<div className="text-2xl mb-4 font-bold text-lime-500 mt-12">Nombre del ganador</div>
						<div className="flex justify-center">
							{/* Current match cards */}
							<div className={"p-4 d-inline"}>
							<EmptySlot/>
							</div>
							<div className={"p-4 d-inline"}>
							<EmptySlot/>
							</div>
						</div>
					</div>
					{/* Opponent section */}
					<div>
						<div className="flex items-center justify-end mb-4">
							<div className="mr-2">
								<div className="text-lg font-semibold">Nombre del oponente</div>
								<div className="flex justify-end">
									<div className="w-4 h-4 bg-green-500 rounded-full mr-1"></div>
									<div className="w-4 h-4 bg-green-500 rounded-full mr-1"></div>
									<div className="w-4 h-4 bg-green-500 rounded-full"></div>
								</div>
							</div>
							<div className="rounded-full bg-pink-300 border-2 border-black w-12 h-12 flex items-center justify-center">🧍</div>
						</div>
						<div className={"flex flex-row-reverse"}>
							{/* Opponent cards */}

							<div className="">
								<EmptySlot/>
							</div>
							<div className="mt-24 -mr-32 z-50">
								<EmptySlot/>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>

	)
}

export default Match;
