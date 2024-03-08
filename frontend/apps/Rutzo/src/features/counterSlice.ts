import {createSlice, PayloadAction} from '@reduxjs/toolkit';

interface counterState {
	value: number;
}

const initialState: counterState = {
	value: 0
};


const counterSlice = createSlice({
	name: 'counter',
	initialState,
	reducers: {
		increment: (state) => {
			state.value++;
		},
		decrement: (state) => {
			state.value--;
		},
		incrementByAmount: (state, action: PayloadAction<number>) => {
			state.value += action.payload;
		}
	}
});

export const {increment, decrement, incrementByAmount} = counterSlice.actions;

export default counterSlice.reducer;
