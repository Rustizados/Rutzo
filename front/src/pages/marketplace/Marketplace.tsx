import Container from 'react-bootstrap/Container';
import { ReactComponent as Banner } from 'assets/images/marketplace.svg';
import './Marketplace.scss';
import {Card} from '../../components/card/Card';

function Marketplace() {
    return (
        <Container className="text-center">
            <Banner style={{ width: '50%', alignSelf: 'center', padding:0 }} />
            <h2 style={{marginBottom:80}}>Get ready for the battle with some cool NFTs</h2>

            <div >
                <div className="cards-container">
                    <Card image="/p.jpg" title="SparkPug Blaze" type="fire" value={2} price={0.96}/>
                    <Card image="/pp.jpg" title="AstroPug Nebula" type="water" value={1} price={1.02}/>
                    <Card image="/r.jpg" title="Rusty Robot" type="ice" value={3} price={.56}/>
                    <Card image="/hex.jpg" title="Astonishing Technicolor" type="water" value={1} price={0.35}/>
                    <Card image="/d.jpg" title="Cosmic Inferno" type="fire" value={3} price={1.50}/>
                </div>
            </div>
        </Container>
    );

}

export { Marketplace };