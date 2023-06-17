import { Route, Routes } from 'react-router-dom';
import { Marketplace } from './marketplace';
import { Home } from './home';

const routes = [{ path: '/', Page: Home }, { path: '/marketplace', Page: Marketplace }];

function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
