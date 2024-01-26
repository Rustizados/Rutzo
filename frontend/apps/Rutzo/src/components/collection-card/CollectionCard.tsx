import styles from './CollectionCard.module.scss';
import { Icon } from '../icon';
import { PowerBar } from '../power';

interface CardProps {
  image: string;
  title: string;
  type: string;
  value: number;
}

function CollectionCard(props: CardProps) {
  const { image, title, type, value } = props;

  return (
    <div className={styles.cards_container}>
    <div
      className={styles.card}
      role="button"
      tabIndex={0}
    >
       <div className={styles.graphics}>
        <img className={styles.hexagon} src={image} alt="NFTimage" />
      </div>
      <p className={styles.title}>{title}</p>

      <div className={styles.content}>

        <div className={styles.details}>
         <p>Type: {type}</p>
         <p>Power: {value}%</p>
        </div>
      </div>
      </div>
    </div>
    
  );
}

export { CollectionCard };