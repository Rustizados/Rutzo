import { useDispatch, useSelector } from 'react-redux';
import {increment} from "@/features/counterSlice";

import {Card} from "@/components";


const Match = () => {
	const dispatch = useDispatch()
	const count = useSelector((state: any) => state.counter.value)
	const cards = useSelector((state: any) => state.cards.cards);

	return(
		<div>
			<div>
				<button
					aria-label="Increment value"
					onClick={() => dispatch(increment())}
				>
					Increment
				</button>

				<div>
					{cards.map((card: any) => (
						<div key={card}>{card}</div>
					))}

				</div>
				<Card image={"null"} title={"null"} type={"null"} value={23}></Card>


			</div>
		</div>

	)
}

export default Match;
