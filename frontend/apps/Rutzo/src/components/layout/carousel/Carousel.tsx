import { useState, useEffect, useCallback } from 'react';

const images = [
  'https://rutzo.studio/NFT/fighting/atlacatl_fighting.jpg',
  'https://rutzo.studio/NFT/fighting/azteca_fighting.jpg',
  'https://rutzo.studio/NFT/fire/chile_fire.jpg',
  'https://rutzo.studio/NFT/fire/paisaje_fire.jpg',
  'https://rutzo.studio/NFT/fire/tezcatli_fire.jpg',
  'https://rutzo.studio/NFT/ghost/jaguar_ghost.jpg',
  'https://rutzo.studio/NFT/ice/ixchel_ice.jpg',
  'https://rutzo.studio/NFT/ice/tlaloc_ice.jpg',
  'https://rutzo.studio/NFT/lightning/nova_lighting.jpg',
  'https://rutzo.studio/NFT/normal/huitzilopochtli_normal.jpg',
  'https://rutzo.studio/NFT/normal/jaguar_normal.jpg',
  'https://rutzo.studio/NFT/poison/angel_of_death_poison.jpg',
  'https://rutzo.studio/NFT/poison/ghoul_poison.jpg',
  'https://rutzo.studio/NFT/poison/quetzal_poison.jpg',
  'https://rutzo.studio/NFT/rock/maya_calavera_rock.jpg',
  'https://rutzo.studio/NFT/rock/zacualpan_rock.jpg',
  'https://rutzo.studio/NFT/water/ajolote_water.jpg',
  'https://rutzo.studio/NFT/water/chinampa_water.jpg',
  'https://rutzo.studio/NFT/water/vaquita_water.jpg',
  'https://rutzo.studio/NFT/wind/aguila_wind.jpg',
  'https://rutzo.studio/NFT/wind/ehecatl_wind.jpg',
  'https://rutzo.studio/NFT/wind/quetzal_wind.jpg',
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
