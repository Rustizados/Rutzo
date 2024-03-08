import './app.scss';
import './global.css';

import { useAccount, useApi } from '@gear-js/react-hooks';
import {  Footer } from '@/components/layout';
import { withProviders } from '@/app/hocs';

import { Layout} from "@/components/layout";
import { Header } from '@/components/layout';
import {ApiLoader} from "@/components";

import {Home} from "@/pages/home";
import {Play} from "@/pages/play";
import {AboutUs} from "@/pages/resources";
import {Marketplace} from "@/pages/marketplace";
import {Game} from "@/pages/game";
import Match from "@/pages/match/Match";


import { createBrowserRouter, RouterProvider } from 'react-router-dom';

// import { useWalletSync } from '@/features/wallet/hooks';
const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      { path: '/', element: <Home /> },
      { path: '/play', element: <Play /> },
      { path: '/about', element: <AboutUs /> },
      { path: '/marketplace', element: <Marketplace /> },
      { path: '/game', element: <Game /> },
      { path: '/match', element: <Match /> },

    ],
  },


]);


function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  // useWalletSync();

  const isAppReady = isApiReady && isAccountReady;

  return (
    <div className="App">
      {isAppReady ? <RouterProvider router={router} /> : <ApiLoader/>}
    </div>
  );
}

export const App = withProviders(Component);
