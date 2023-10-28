import { ReactComponent as Bug } from 'assets/images/class_icons/bug.svg';
import { ReactComponent as Fire } from 'assets/images/class_icons/fire.svg';
import { ReactComponent as Water } from 'assets/images/class_icons/water.svg';
import { ReactComponent as Grass } from 'assets/images/class_icons/grass.svg';
import { ReactComponent as Electric } from 'assets/images/class_icons/electric.svg';
import { ReactComponent as Psychic } from 'assets/images/class_icons/psychic.svg';
import { ReactComponent as Ice } from 'assets/images/class_icons/ice.svg';
import { ReactComponent as Dragon } from 'assets/images/class_icons/dragon.svg';
import { ReactComponent as Fairy } from 'assets/images/class_icons/fairy.svg';
import { ReactComponent as Fighting } from 'assets/images/class_icons/fighting.svg';
import { ReactComponent as Flying } from 'assets/images/class_icons/flying.svg';
import { ReactComponent as Ghost } from 'assets/images/class_icons/ghost.svg';
import { ReactComponent as Ground } from 'assets/images/class_icons/ground.svg';
import { ReactComponent as Normal } from 'assets/images/class_icons/normal.svg';
import { ReactComponent as Poison } from 'assets/images/class_icons/poison.svg';
import { ReactComponent as Rock } from 'assets/images/class_icons/rock.svg';
import './Icons.css';

function Icon({ name }: { name: string }) {
    switch (name) {
        case 'bug':
            return <Bug className="icon bug" />;
        case 'dragon':
            return <Dragon className="icon dragon" />;
        case 'electric':
            return <Electric className="icon electric" />;
        case 'fairy':
            return <Fairy className="icon fairy" />;
        case 'fighting':
            return <Fighting className="icon fighting" />;
        case 'fire':
            return <Fire className="icon fire" />;
        case 'flying':
            return <Flying className="icon flying" />;
        case 'ghost':
            return <Ghost className="icon ghost" />;
        case 'grass':
            return <Grass className="icon grass" />;
        case 'ground':
            return <Ground className="icon ground" />;
        case 'ice':
            return <Ice className="icon ice" />;
        case 'normal':
            return <Normal className="icon normal" />;
        case 'poison':
            return <Poison className="icon poison" />;
        case 'psychic':
            return <Psychic className="icon psychic" />;
        case 'rock':
            return <Rock className="icon rock" />;
        case 'water':
            return <Water className="icon water" />;
        default:
            return <Normal className="icon normal" />;
    }
}

export { Icon };