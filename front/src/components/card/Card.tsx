import React from 'react';
import styles from './Card.module.scss';
import { Icon } from '../icon';
import { PowerBar } from '../power';
import { CardDialog } from './CardDialog';
import { Modal } from './Modal';

interface CardProps {
  image: string;
  title: string;
  type: string;
  value: number;
  price: number;
}

interface CardState {
  dialogOpen: boolean;
}

class Card extends React.Component<CardProps, CardState> {
  constructor(props: CardProps) {
    super(props);
    this.state = {
      dialogOpen: false,
    };
    this.handleClick = this.handleClick.bind(this);
    this.handleClose = this.handleClose.bind(this);
  }

  handleClick() {
    this.setState({ dialogOpen: true });
  }

  handleKeyDown(event: React.KeyboardEvent<HTMLDivElement>) {
    if (event.key === 'Enter' || event.key === ' ') {
      this.setState({ dialogOpen: true });
    }
    }

  handleClose() {
    this.setState({ dialogOpen: false });
  }

  render() {
    const { image, title, type, value, price } = this.props;
    const { dialogOpen } = this.state;

    return (
      <div className={styles.cards_container}>
        <div
          className={styles.card}
          onClick={this.handleClick}
          onKeyDown={(e) => this.handleKeyDown(e)}
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
        {dialogOpen && (
          <Modal onClose={this.handleClose}>
            <CardDialog
              isOpen={dialogOpen}
              onClose={this.handleClose}
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
}

export { Card };
