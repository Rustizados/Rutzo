import {createSlice, PayloadAction} from '@reduxjs/toolkit';

interface cardsState {
	cards: string[];

}

const initialState: cardsState = {
	cards: ["hello", "hey"]
};

const cardsSlice = createSlice({
	name: 'cards',
	initialState,
	reducers: {
		addCard: (state, action: PayloadAction<string>) => {
			state.cards.push(action.payload);
		},
		removeCard: (state, action: PayloadAction<string>) => {
			state.cards = state.cards.filter(card => card !== action.payload);
		}
	}
});

export const {addCard, removeCard} = cardsSlice.actions;
export default cardsSlice.reducer;
