import { ReactComponent as Intro } from 'assets/images/Intro.svg';
import Container from 'react-bootstrap/Container';
import { Features } from './Features';
import { HowToStart } from './HowToStart';
import { FAQ } from './FAQ';

function Home() {
  return (
    <Container className='About'>
      <Intro style={{width: '100%'}}/>
      <h1 style={{ textAlign: 'center', margin: '5%', color: '#7834CF'}} id='features'>Features</h1>
      <Features />
      <h1 style={{ textAlign: 'center', margin: '5%', color: '#7834CF'}} id='how-to-start'>How to start</h1>
      <HowToStart />
      <h1 style={{ textAlign: 'center', margin: '5%', color: '#7834CF'}} id='faq'>FAQ</h1>
      <FAQ />
    </Container>
  );
}

export { Home };
