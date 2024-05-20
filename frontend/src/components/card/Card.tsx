import React from 'react';
import styles from './Card.module.scss';

interface CardProps {
  image: string;
  title: string;
  type: string;
  value: number;
  price?: number;
  onCardClick?: () => void;
  children?: any;
  scale?: number; // Añadida prop para escalar
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
    const { onCardClick } = this.props;

    if (onCardClick) {
      onCardClick();
    } else {
      this.setState({ dialogOpen: true });
    }
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
    const { image, title, type, value, price, children, scale = 1 } = this.props; // Prop scale con valor por defecto 1

    const scaledWidth = 13 * scale; // Base width 13rem
    const scaledHeight = 20 * scale; // Base height 20rem
    const scaledFontSize = 1 * scale; // Base font-size 1rem

    const cardStyle = {
      width: `${scaledWidth}rem`,
      height: `${scaledHeight}rem`,
      margin: `${0.5 * scale}rem`,
      padding: `${0.5 * scale}rem`,
      fontSize: `${scaledFontSize}rem`,
    };

    return (
      <div className={styles.cards_container}>
        <div
          className={`${styles.card} rounded-lg inline-block`}
          style={cardStyle}
          onClick={this.handleClick}
          onKeyDown={(e) => this.handleKeyDown(e)}
          role="button"
          tabIndex={0}
        >
          <div className={styles.graphics}>
            <img className={styles.hexagon} src={image} alt="NFTimage" />
          </div>
          <p className={styles.title} style={{ fontSize: `${scaledFontSize * 1.2}rem` }}>{title}</p>

          <div className={children !== undefined ? styles.hiddeable : styles.content}>
            <div className={styles.details} style={{ fontSize: `${scaledFontSize}rem` }}>
              <p>Type: {type}</p>
              <p>Power: {value}%</p>
            </div>

            {children !== undefined ? (
              <div className={styles.price} style={{ fontSize: `${scaledFontSize}rem` }}>
                <p className={styles.priceText}>${price} TVara</p>
              </div>
            ) : null}
          </div>

          {children !== undefined ? (
            <div className={styles.button_container} style={{ fontSize: `${scaledFontSize}rem` }}>
              {children}
            </div>
          ) : null}
        </div>
        {/* {dialogOpen && (
          <Modal onClose={this.handleClose}>
            <CardDialog
              isOpen={dialogOpen}
              onClose={this.handleClose}
              image={image}
              title={title}
              type={type}
              value={value}
              price={price ?? 0}
            >
              {children}
            </CardDialog>
          </Modal>
        )} */}
      </div>
    );
  }
}

export { Card };
