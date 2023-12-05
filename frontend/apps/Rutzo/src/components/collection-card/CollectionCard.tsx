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

        <div className={styles.content}>
          <p className={styles.title}>{title}</p>
          <div className={styles.typec}>
            <Icon name={type} />
            <p className={styles.type}>{type}</p>
          </div>
          <div>
            <PowerBar progress={value} />
          </div>
        </div>
      </div>
    </div>
  );
}

export { CollectionCard };