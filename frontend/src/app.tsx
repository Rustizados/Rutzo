import "./app.scss";
import "./global.css";

import { useAccount, useApi } from "@gear-js/react-hooks";
import { Footer } from "@/components/layout";
import { withProviders } from "@/app/hocs";

import { Layout } from "@/components/layout";
import { Header } from "@/components/layout";
import { ApiLoader, BoardGame } from "@/components";

import { Home } from "@/pages/home";
import { Play } from "@/pages/play";
import { AboutUs, Rules } from "@/pages/resources";
import { Marketplace } from "@/pages/marketplace";
import { Game } from "@/pages/game";
import Match from "@/pages/match/Match";
import { Select } from "@/pages/select";
import Selection from "@/pages/selection/Selection";
import { BoardGame2 } from "@/components/board-game/BoardGame2";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { AllNFTs } from "./pages/nfts/AllNFTs";
import { PrivacyPolicy, TermsAndConditions } from "./pages/legal";

// import { useWalletSync } from '@/features/wallet/hooks';
const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "/", element: <Home /> },
      {
        path: "play",
        element: <Play />,
        children: [
          { path: "select", element: <Select /> },
          { path: "fight", element: <Match /> },
        ],
      },
      { path: "/marketplace", element: <Marketplace /> },
      { path: "/game", element: <Game /> },
      { path: "/rules", element: <Rules /> },
      { path: "/all", element: <AllNFTs /> },
      { path: "/terms", element: <TermsAndConditions /> },
      { path: "/privacy", element: <PrivacyPolicy /> },
      { path: "/about", element: <AboutUs /> },
      { path: "/select", element: <Select /> },
      { path: "/selection", element: <Selection /> },
      { path: '/match', element: <Match /> },
      { path: "/fight", element: <BoardGame2 /> },
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
      {isAppReady ? <RouterProvider router={router} /> : <ApiLoader />}
    </div>
  );
}

export const App = withProviders(Component);
