import styles from './Carousel.module.scss';

interface CarouselProps {
  images: string[];
  style: React.CSSProperties;
}


function Carousel({ images, style}: CarouselProps) {
  
  return (
      <div className={styles.carousel_container} style={{ ...style}}>
        {images.map((image, index) => (
          <div className={styles.carousel_item} key={index}>
            <img src={image} alt="NFTs" />
          </div>
        ))}
      </div>
  );
}

export { Carousel };
