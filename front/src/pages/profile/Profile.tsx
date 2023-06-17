
import Identicon from "@polkadot/react-identicon";
import './Profile.scss';
import {Card} from '../../components/card/Card';

function Profile(){
    return(
        <div >
            <h1 className="title">Profile</h1>
            <div className="cards">
                <Card image="/p.jpg" title="SparkPug Blaze" type="fire" value={2}/>
                <Card image="/pp.jpg" title="SparkPug Blaze" type="water" value={1}/>
                <Card image="/r.jpg" title="SparkPug Blaze" type="ice" value={3}/>
                <Card image="/hex.jpg" title="SparkPug Blaze" type="water" value={1}/>
                <Card image="/d.jpg" title="SparkPug Blaze" type="fire" value={3}/>
            </div>


        </div>
    );
}

export { Profile };