import { useState, useEffect, useCallback } from 'react';

const images = [
  'https://www.rutzo.tech/NFT/fighting/atlacatl_fighting.jpg',
  'https://www.rutzo.tech/NFT/fighting/azteca_fighting.jpg',
  'https://www.rutzo.tech/NFT/fire/chile_fire.jpg',
  'https://www.rutzo.tech/NFT/fire/paisaje_fire.jpg',
  'https://www.rutzo.tech/NFT/fire/tezcatli_fire.jpg',
  'https://www.rutzo.tech/NFT/ghost/jaguar_ghost.jpg',
  'https://www.rutzo.tech/NFT/ice/ixchel_ice.jpg',
  'https://www.rutzo.tech/NFT/ice/tlaloc_ice.jpg',
  'https://www.rutzo.tech/NFT/lightning/nova_lighting.jpg',
  'https://www.rutzo.tech/NFT/normal/huitzilopochtli_normal.jpg',
  'https://www.rutzo.tech/NFT/normal/jaguar_normal.jpg',
  'https://www.rutzo.tech/NFT/poison/angel_of_death_poison.jpg',
  'https://www.rutzo.tech/NFT/poison/ghoul_poison.jpg',
  'https://www.rutzo.tech/NFT/poison/quetzal_poison.jpg',
  'https://www.rutzo.tech/NFT/rock/maya_calavera_rock.jpg',
  'https://www.rutzo.tech/NFT/rock/zacualpan_rock.jpg',
  'https://www.rutzo.tech/NFT/water/ajolote_water.jpg',
  'https://www.rutzo.tech/NFT/water/chinampa_water.jpg',
  'https://www.rutzo.tech/NFT/water/vaquita_water.jpg',
  'https://www.rutzo.tech/NFT/wind/aguila_wind.jpg',
  'https://www.rutzo.tech/NFT/wind/ehecatl_wind.jpg',
  'https://www.rutzo.tech/NFT/wind/quetzal_wind.jpg',
];

function Carousel() {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [displayImages, setDisplayImages] = useState<string[]>([]);

  const getNextImages = useCallback(() => {
    const startIndex = currentIndex % images.length;
    const endIndex = (startIndex + 6) % images.length;
    if (endIndex > startIndex) {
      return images.slice(startIndex, endIndex);
    } else {
      return [...images.slice(startIndex), ...images.slice(0, endIndex)];
    }
  }, [currentIndex]);
  
  const rotateImages = useCallback(() => {
    const newDisplayImages = getNextImages();
    setCurrentIndex((prevIndex) => (prevIndex + 6) % images.length);
    setDisplayImages(newDisplayImages);
  }, [getNextImages]);

  useEffect(() => {
    const interval = setInterval(() => {
      rotateImages();
    }, 3000);

    return () => clearInterval(interval);
  }, [rotateImages]);

  

  return (
    <div className="grid grid-cols-6 gap-4">
      {displayImages.map((image, index) => (
        <div key={index}>
          <img src={image} alt="NFTs" className="rounded-md" />
        </div>
      ))}
    </div>
  );
}

export { Carousel };
