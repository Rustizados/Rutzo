import { TbHexagonNumber1, TbHexagonNumber2, TbExternalLink } from 'react-icons/tb';
import extension from 'assets/images/howto/extension.png';
import signin from 'assets/images/howto/signin.png';
import Container from 'react-bootstrap/Container';

function HowToStart() {
  return ( 
    <Container style={{ margin: '5%' }}>
      <Container style={{ alignSelf: 'center'}}>
        <h2 style={{ textAlign: 'center', marginBottom: '20px' }}>
          <TbHexagonNumber1 style={{ fontSize: '300%' }} />
          <br/><br/>
          Download the <a href="https://polkadot.js.org/extension/">polkadot js extension   <TbExternalLink /></a>
        </h2>
        <p style={{marginBottom: '50px', textAlign: 'center'}}>Create and sign transactions with your accounts</p>
        <img src={extension} alt="Polkadot extension" style={{width: '100%', marginBottom: '50px'}}/>
      </Container>
      <Container style={{ alignSelf: 'center'}}>
        <h2 style={{ textAlign: 'center', marginBottom: '20px' }}>
          <TbHexagonNumber2 style={{ fontSize: '300%' }} />
          <br/><br/>
          Connect your account with Rutzo
        </h2>
        <p style={{marginBottom: '50px', textAlign: 'center'}}>Select the account to save your NFTs and start playing</p>
        <img src={signin} alt="Select polkadot account" style={{width: '100%'}}/>
      </Container>
    </Container>
    
  );
}

export { HowToStart };
