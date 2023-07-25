import { BrowserRouter, Route, Routes } from 'react-router-dom';
import { Marketplace } from './marketplace';
import { Home } from './home';
import { Play } from './play';
import { Profile } from './profile';

export function Routing() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/marketplace" element={<Marketplace />} />
      <Route path="/play" element={<Play />} />
      <Route path="/profile" element={<Profile />} />
    </Routes>
  );
}

export function App() {
  return (
    <BrowserRouter basename="/Rutzo">
      <Routing />
    </BrowserRouter>
  );
}
