import Identicon from '@polkadot/react-identicon';
import { buttonStyles } from '@gear-js/ui';
import styles from './Card.module.scss';
import {Icon} from './icon'
import {Stars} from './stars'


function Card (props: { image: string; title: string; type: string; value: number; }) {
    const { image, title, type, value } = props;

    return (
    <div className={styles.container}>
        <div className={styles.card}>
            <div className={styles.graphics}>
                <img className={styles.hexagon} src={image} alt="NFTimage" />
                <Icon name={type} />
            </div>


            <div className={styles.content}>
                <h3 className={styles.title}>{title}</h3>
                <div className={styles.typec}>
                    <p className={styles.type}>{type}</p>
                </div>
                <div>
                    <Stars num={value}/>

                </div>
            </div>
        </div>
    </div>
    );
}

export { Card };