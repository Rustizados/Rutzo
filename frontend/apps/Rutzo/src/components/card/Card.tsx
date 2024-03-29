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
    const { image, title, type, value, price } = this.props;
    const { children } = this.props;

    return (
      <div className={styles.cards_container}>
        <div
          className={styles.card + ' w-52 h-80 rounded-lg inline-block'}
          onClick={this.handleClick}
          onKeyDown={(e) => this.handleKeyDown(e)}
          role="button"
          tabIndex={0}
        >
           <div className={styles.graphics}>
            <img className={styles.hexagon} src={image} alt="NFTimage" />
          </div>
          <p className={styles.title}>{title}</p>

          <div className={children !== undefined ? styles.hiddeable : styles.content}>

            <div className={styles.details}>
             <p>Type: {type}</p>
             <p>Power: {value}%</p>
            </div>

            {children !== undefined 
            ?
            <div className={styles.price}>
              <p className={styles.priceText}>${price} TVara</p>
            </div>
            :
            null
            }
            
          </div>

          {children !== undefined ?
          <div className={styles.button_container}>
          { children }
          </div>
          :
          null
        }

          
        </div>
        {/*{dialogOpen && (
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
              { children }
            </CardDialog>
          </Modal>
        )}*/}
      </div>
    );
  }
}

export { Card };
