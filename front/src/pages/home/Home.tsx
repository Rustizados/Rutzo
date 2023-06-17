import { ReactComponent as Intro } from 'assets/images/Intro.svg';
import Container from 'react-bootstrap/Container';
import { Features } from './Features';
import { HowToStart } from './HowToStart';

function Home() {
  return (
    <Container>
      <Intro style={{width: '100%'}}/>
      <h1 style={{ textAlign: 'center', margin: '5%'}}>Features</h1>
      <Features />
      <h1 style={{ textAlign: 'center', margin: '5%'}}>How to start</h1>
      <HowToStart />
    </Container>
  );
}

export { Home };
