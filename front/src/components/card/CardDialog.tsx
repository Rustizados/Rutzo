import React from 'react';
import styles from './CardDialog.module.scss';
import { Icon } from './icon';
import { Stars } from './stars';

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
    <div className={styles.dialogOverlay}>
      <div className={styles.dialog}>
        <img className={styles.hexagon} src={image} alt="NFTimage" />
        <p className={styles.title}>{title}</p>
        <div className={styles.typec}>
          {/* Assuming Icon and Stars are already defined */}
          <Icon name={type} />
          <p className={styles.type}>{type}</p>
        </div>
        <div>
          <Stars num={value} />
        </div>
        <p className={styles.price}>${price} ETH</p>
        <button type='button' onClick={onClose}>
          Cerrar
        </button>
      </div>
    </div>
  );
}

export { CardDialog };
