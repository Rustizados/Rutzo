import { ReactComponent as Intro } from 'assets/images/Intro.svg';
import Container from 'react-bootstrap/Container';
import { Features } from './Features';
import { HowToStart } from './HowToStart';
import { FAQ } from './FAQ';
import './Home.scss';

function Home() {
  return (
    <Container className='About'>
      <Intro style={{width: '100%'}}/>
        <div className="playcontainer">
            <a href="/play" className="play">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={3.5} stroke="currentColor" className="w-6 h-6">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.348a1.125 1.125 0 010 1.971l-11.54 6.347a1.125 1.125 0 01-1.667-.985V5.653z" />
                </svg>
                Play
            </a>
        </div>

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
