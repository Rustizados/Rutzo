import './app.scss';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { Routing } from './pages';
import { ApiLoader } from '@/components';
import { Header, Footer } from '@/components/layout';
import { withProviders } from '@/app/hocs';

// import "./global.css";

// import { useWalletSync } from '@/features/wallet/hooks';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  // useWalletSync();

  const isAppReady = isApiReady && isAccountReady;

  return (
    <>
      <Header isAccountVisible={isAccountReady} />
      <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>
      <Footer />
    </>
  );
}

export const App = withProviders(Component);


/*

{
    "name": "Death City Earth",
    "description": "Rock",
    "media": "https://home.rutzo.studio/NFT/death_city_earth.jpg",
    "reference": "20"
},
{
    "name": "Chinampa",
    "description": "Water",
    "media": "https://home.rutzo.studio/NFT/chinampa_water.jpg",
    "reference": "25"
},
{
    "name": "Chile",
    "description": "Fire",
    "media": "https://home.rutzo.studio/NFT/chile_fire.jpg",
    "reference": "55"
},
{
    "name": "peaceful axolotl",
    "description": "Water",
    "media": "https://home.rutzo.studio/NFT/peaceful_axolotl_water.jpg",
    "reference": "33"
},
{
    "name": "ixchel",
    "description": "Rock",
    "media": "https://home.rutzo.studio/NFT/ixchel_wind.jpg",
    "reference": "33"
},
{
    "name": "tlaloc",
    "description": "Water",
    "media": "https://home.rutzo.studio/NFT/tlaloc_water.jpg",
    "reference": "75"
}

*/
