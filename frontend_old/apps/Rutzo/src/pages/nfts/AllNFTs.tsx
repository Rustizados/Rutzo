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

function AllNFTs() {
  return (
    <div>
      <h1 className="text-3xl md:text-5xl font-semibold p-10 md:p-16 text-center">
        The complete <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">NFT Collection</span>
      </h1>
      <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
        {images.map((imageUrl, index) => (
          <div key={index}>
            <img
              className="h-auto max-w-full rounded-lg object-cover object-center"
              src={imageUrl}
              alt={`gallery-${index}`}
            />
          </div>
        ))}
      </div>
      <p className="text-center text-xl m-10">And more coming soon...</p>
    </div>
  );
}

export { AllNFTs };
