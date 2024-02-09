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
              alt={`gallery-photo-${index}`}
            />
          </div>
        ))}
      </div>
      <p className="text-center text-xl m-10">And more coming soon...</p>
    </div>
  );
}

export { AllNFTs };
