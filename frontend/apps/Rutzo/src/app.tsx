import './app.scss';
import './global.css';
import "slick-carousel/slick/slick.css";
import "slick-carousel/slick/slick-theme.css";
import { useAccount, useApi } from '@gear-js/react-hooks';
import { Routing } from './pages';
import { ApiLoader } from '@/components';
import { Header, Footer } from '@/components/layout';
import { withProviders } from '@/app/hocs';

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