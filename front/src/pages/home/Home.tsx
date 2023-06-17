import { ReactComponent as Intro } from 'assets/images/Intro.svg';
import Container from 'react-bootstrap/Container';
import { Features } from './Features';
import { HowToStart } from './HowToStart';
import { FAQ } from './FAQ';

function Home() {
  return (
    <Container>
      <Intro style={{width: '100%'}}/>
      <h1 style={{ textAlign: 'center', margin: '5%', color: '#7834CF'}}>Features</h1>
      <Features />
      <h1 style={{ textAlign: 'center', margin: '5%', color: '#7834CF'}}>How to start</h1>
      <HowToStart />
      <h1 style={{ textAlign: 'center', margin: '5%', color: '#7834CF'}}>FAQ</h1>
      <FAQ />
    </Container>
  );
}

export { Home };
