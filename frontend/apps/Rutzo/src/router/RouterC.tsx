import {createBrowserRouter, RouterProvider} from "react-router-dom";

import { Home } from '@/pages/home';


const routes = createBrowserRouter( [
		{ path: '/', element: <Home/> }
]);

function RouterC() {

	return (
			<RouterProvider router={routes}/>

	)
}

export { RouterC};
