import './play-styles.css'
import { Selector } from './Selector';
import { MainGame } from "./MainGame";

function Play() {
    return (
        <div style={{ height: '100vh', color:'white' }} className="play-title">
            <h3 style={{ marginBottom: '50px'}}>Select your better cards and may the force be with you</h3>
            <br/>
            <MainGame />
        </div>
    );
}

export { Play };