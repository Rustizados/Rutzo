import './play-styles.css'
import { Selector } from './Selector';

function Play() {
    return (
        <div style={{ height: '100vh', color:'#7834CF' }} className="play-title">
            <h3 style={{ marginBottom: '50px'}}>Select your better cards and may the force be with you</h3>
            <br/>
            <Selector />
        </div>
    );
}

export { Play };