import React, { useState, useCallback } from 'react';
import styles from './Card.module.scss';
import { Icon } from './icon';
import { PowerBar } from './power';
import { CardDialog } from './CardDialog';
import { Modal } from './Modal';

interface CardProps {
  image: string;
  title: string;
  type: string;
  value: number;
  price: number;
  onCardClick?: () => void;
}

function Card({ image, title, type, value, price, onCardClick }: CardProps) {
  const [dialogOpen, setDialogOpen] = useState(false);

  const handleClick = useCallback(() => {
    if (onCardClick) {
      onCardClick();
    } else {
      setDialogOpen(true);
    }
  }, [onCardClick]);
  const handleKeyDown = useCallback((event: React.KeyboardEvent<HTMLDivElement>) => {
    if (event.key === 'Enter' || event.key === ' ') {
      setDialogOpen(true);
    }
  }, []);

  const handleClose = useCallback(() => {
    setDialogOpen(false);
  }, []);

  return (
      <div className={styles.cards_container}>
        <div
            className={styles.card}
            onClick={handleClick}
            onKeyDown={handleKeyDown}
            role="button"
            tabIndex={0}
            aria-label={`Open ${title} card details`}  // For better accessibility
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
        {dialogOpen && (
            <Modal onClose={handleClose}>
              <CardDialog
                  isOpen={dialogOpen}
                  onClose={handleClose}
                  image={image}
                  title={title}
                  type={type}
                  value={value}
                  price={price}
              />
            </Modal>
        )}
      </div>
  );
}

export { Card };
