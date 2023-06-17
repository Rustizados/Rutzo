import Container from 'react-bootstrap/Container';
import { ReactComponent as Banner } from 'assets/images/marketplace.svg';

function Marketplace() {
    return (
        <Container className="text-center">
            <Banner style={{ width: '50%', alignSelf: 'center', padding:0 }} />
            <h2>Get ready for the battle with some cool NFTs</h2>
        </Container>
    );

}

export { Marketplace };