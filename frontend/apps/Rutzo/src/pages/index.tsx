import { Route, Routes } from 'react-router-dom';
import { Marketplace } from './marketplace';
import { Home } from './home';
import { Play } from './play';
import { Profile } from './profile';
import { Game } from './game';
import { Select } from './select';
import { Fight } from './fight';
import { TermsAndConditions } from './legal/Terms';
import { PrivacyPolicy } from './legal/PrivacyPolicy';
import { AboutUs } from './resources/AboutUs';
import { Rules } from './resources/Rules';

const routes = [
    { path: '/', Page: Home },
    { path: '/marketplace', Page: Marketplace },
    { path: '/play', Page: Play },
    { path: '/select', Page: Select },
    { path: '/profile', Page: Profile },
    { path: '/game', Page: Game },
    { path: '/fight', Page: Fight },
    { path: '/terms', Page: TermsAndConditions },
    { path: '/privacy', Page: PrivacyPolicy },
    { path: '/about', Page: AboutUs },
    { path: '/rules', Page: Rules},
];

function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return <Routes>{getRoutes()}</Routes>;
}


export { Routing };
