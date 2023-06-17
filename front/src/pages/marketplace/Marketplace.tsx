import Container from 'react-bootstrap/Container';
import { ReactComponent as Banner } from 'assets/images/marketplace.svg';
import './Marketplace.scss';
import {Card} from '../../components/card/Card';

function Marketplace() {
    return (
        <Container className="text-center">
            <Banner style={{ width: '50%', alignSelf: 'center', padding:0 }} />
            <h2>Get ready for the battle with some cool NFTs</h2>

            <div >

                <div className="cards">
                    <Card image="/p.jpg" title="SparkPug Blaze" type="fire" value={2}/>
                    <Card image="/pp.jpg" title="AstroPug Nebula" type="water" value={1}/>
                    <Card image="/r.jpg" title="Rusty Robot" type="ice" value={3}/>
                    <Card image="/hex.jpg" title="Astonishing Technicolor" type="water" value={1}/>
                    <Card image="/d.jpg" title="Cosmic Inferno" type="fire" value={3}/>
                </div>
            </div>
        </Container>
    );

}

export { Marketplace };