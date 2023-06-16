import { ReactComponent as Intro } from 'assets/images/Intro.svg';
import Container from 'react-bootstrap/Container';
import { Features } from './Features';


function Home() {
  return (
    <Container>
      <Intro style={{width: '100%'}}/>
      <h1 style={{ textAlign: 'center', margin: '5%'}}>Features</h1>
      <Features />
    </Container>
  );
}

export { Home };
