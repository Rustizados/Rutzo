import {useState} from "react";
import {CardProps} from "@/interfaces/Card";
import useCardsData from "@/hooks/useCardsData";
import {useEffect} from "react";

const useLocalBoard = () => {
	const MAX_SELECTED_CARDS = 3;
	const [availableCards, setAvailableCards] = useState<CardProps[]>([]);
	const [selectedCards, setSelectedCards] = useState<CardProps[]>([]);


	const {allUserCards, fetchData, getPlayingCards, } = useCardsData();

	useEffect(() => {
			fetchData();
			getPlayingCards();
		}
		, [fetchData, getPlayingCards]);

	useEffect(() => {
		setAvailableCards(allUserCards);
	}, [allUserCards]);

const pushCard = (card: CardProps) => {
		if (selectedCards.length < MAX_SELECTED_CARDS) {
			setSelectedCards([...selectedCards, card]);
			setAvailableCards(availableCards.filter((c) => c[0] !== card[0]));
		}
	};

	const removeCard = (card: CardProps) => {
		setSelectedCards(selectedCards.filter((c) => c[0] !== card[0]));
		setAvailableCards([...availableCards, card]);
	};

	const clearSelectedCards = () => {
		setAvailableCards([...availableCards, ...selectedCards]);
		setSelectedCards([]);
	}

	const board = {availableCards, selectedCards};




	return {pushCard, removeCard, clearSelectedCards, board};
};

export default useLocalBoard;
