import Container from 'react-bootstrap/Container';
import Accordion from 'react-bootstrap/Accordion';
import 'bootstrap/dist/css/bootstrap.min.css';
import './home_styles.css';

function FAQ() {
  return (
    <Container>
      <Accordion>
        <Accordion.Item eventKey="0" >
          <Accordion.Header>How do I create an account?</Accordion.Header>
          <Accordion.Body>
          To create an account, click on the &quot;Sign Up&quot; button on the homepage and follow the instructions to provide the required information.
          </Accordion.Body>
        </Accordion.Item>
        <Accordion.Item eventKey="1" >
          <Accordion.Header>How can I purchase NFTs?</Accordion.Header>
          <Accordion.Body>
          To purchase NFTs, navigate to the &quot;Marketplace&quot; section and browse through the available collections. Select the NFT you want to purchase and follow the prompts to complete the transaction.
          </Accordion.Body>
        </Accordion.Item>
        <Accordion.Item eventKey="2" >
          <Accordion.Header>Can I trade or sell my owned NFTs?</Accordion.Header>
          <Accordion.Body>
          Yes, you can trade or sell your owned NFTs on our platform. Visit the &quot;My Collection&quot; section and select the NFT you want to trade or sell. Follow the provided options to initiate the transaction.
          </Accordion.Body>
        </Accordion.Item>
        <Accordion.Item eventKey="3" >
          <Accordion.Header>How do I participate in battles?</Accordion.Header>
          <Accordion.Body>
          Battles are scheduled events where players can compete using their owned NFTs. To participate, navigate to the &quot;Battles&quot; section, select the available battle, and follow the instructions to enter your NFTs into the battle.
          </Accordion.Body>
        </Accordion.Item>
        <Accordion.Item eventKey="4" >
          <Accordion.Header>What happens if I win a battle?</Accordion.Header>
          <Accordion.Body>
          Winning a battle rewards you with additional NFTs and in-game rewards. The conquered NFTs from defeated opponents become part of your collection, symbolizing your victory.
          </Accordion.Body>
        </Accordion.Item>
      </Accordion>
    </Container>
  );
}

export { FAQ };
