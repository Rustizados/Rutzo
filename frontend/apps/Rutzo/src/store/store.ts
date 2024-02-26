import {configureStore} from '@reduxjs/toolkit';

import cardsReducer from '../features/cardsSlice';

export const store = configureStore({
	reducer: {
		cards: cardsReducer
	}
});


