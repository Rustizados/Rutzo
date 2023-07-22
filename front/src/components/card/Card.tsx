import Identicon from '@polkadot/react-identicon';
import { buttonStyles } from '@gear-js/ui';
import styles from './Card.module.scss';
import {Icon} from './icon'
import {Stars} from './stars'


function Card (props: { image: string; title: string; type: string; value: number; price: number; }) {
    const { image, title, type, value, price } = props;

    return (
    <div className={styles.cards_container}>
        <div className={styles.card}>
            <div className={styles.graphics}>
                <img className={styles.hexagon} src={image} alt="NFTimage" />
            </div>


            <div className={styles.content}>
                <p className={styles.title}>{title}</p>
                <div className={styles.typec}>
                    <Icon name={type} />
                    <p className={styles.type}>{type}</p>
                </div>
                <div>
                    <Stars num={value}/>
                </div>
                <p className={styles.price}>${price} ETH</p>
            </div>
        </div>
    </div>
    );
}

export { Card };