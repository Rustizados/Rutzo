import styles from './CardDialog.module.scss';
import { Icon } from './icon';
import { PowerBar } from './power';
import { MintNFT } from './MintNFT';

interface CardDialogProps {
  isOpen: boolean;
  onClose: () => void;
  image: string;
  title: string;
  type: string;
  value: number;
  price: number;
}

function CardDialog({
  isOpen,
  onClose,
  image,
  title,
  type,
  value,
  price,
}: CardDialogProps) {
  if (!isOpen) return null;

  return (
    <div className={styles.dialog}>
      <img className={styles.hexagon} src={image} alt="NFTimage" />
      <p className={styles.title}>{title}</p>
      <div className={styles.typec}>
        <Icon name={type} />
        <p className={styles.type}>{type}</p>
      </div>
      <div>
        <PowerBar progress={value} />
      </div>
      <MintNFT/>
    </div>
  );
}

export { CardDialog };
