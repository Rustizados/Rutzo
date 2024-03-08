import {configureStore} from '@reduxjs/toolkit';

import cardsReducer from '../features/cardsSlice';
import counterReducer from '../features/counterSlice';


export const store = configureStore({
	reducer: {
		cards: cardsReducer,
		counter: counterReducer
	}
});


