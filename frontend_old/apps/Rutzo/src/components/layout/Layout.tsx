import {Outlet} from 'react-router-dom';
import {Header} from "./header";
import {Footer} from "./footer";
import { useAccount, useApi } from '@gear-js/react-hooks';
// import

const Layout = () => {
	const { isApiReady } = useApi();
	const { isAccountReady } = useAccount();

	const isAppReady = isApiReady && isAccountReady;



	return (
		<div>
			<Header isAccountVisible={isAppReady} />

			<main>
			<Outlet/>


			</main>
			<Footer/>

		</div>
	);
}

export {Layout};
