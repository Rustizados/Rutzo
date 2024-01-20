import { Features } from "./Features";
import { HowToStart } from "./HowToStart";
import { FAQ } from "./FAQ";
import "./Home.scss";
import { Play } from '@/components/play/Play';
import { Carousel } from '@/components/layout/carousel/Carousel';

function Home() {

  const images = [
    'https://home.rutzo.studio/NFT/lightning/nova_lighting.jpg',
    'https://home.rutzo.studio/NFT/lightning/nova_lighting.jpg',
    'https://home.rutzo.studio/NFT/lightning/nova_lighting.jpg',
    'https://home.rutzo.studio/NFT/lightning/nova_lighting.jpg',
  ];

  const images2 = [
    'https://home.rutzo.studio/NFT/poison/angel_of_death_poison.jpg',
    'https://home.rutzo.studio/NFT/poison/angel_of_death_poison.jpg',
    'https://home.rutzo.studio/NFT/poison/angel_of_death_poison.jpg',
  ];

  return (
    <div className="About">
      <div className="Main">
        <div className="MainTitle">
          <h1>
          Challenge and conquer the <span className="GradientTitle">World of NFTs</span>
          </h1>
          <p>Play epic battles on Rutzo and become the king of NFT cards. As long as you win more battles, you´ll win more cards.</p>
          < Play style={{marginTop: "40px"}} id="action_button"/>
        </div>
        <div className="Banner">
          <img src="https://home.rutzo.studio/NFT/poison/quetzal_poison.jpg" alt="NFTs" style={{transform: 'rotate(20deg)'}}/>
          <img src="https://home.rutzo.studio/NFT/lightning/nova_lighting.jpg" alt="NFTs" style={{transform: 'rotate(10deg)'}}/>
          <img src="https://home.rutzo.studio/NFT/poison/angel_of_death_poison.jpg" alt="NFTs"/>
        </div>
        </div>

      <h1
        id="features"
        className="title"
      >
        Features
      </h1>
      <Features />

      <div className="explore">

        <h1
          id="marketplace"
          className="title"
        >
          Explore the marketplace
        </h1>
        <p className="subtitle">
        Play epic battles on Rutzo and become the king of NFT cards. As long as you win more battles, you'll win more cards.
        </p>
        <Carousel images={images} style={{marginInline:"100px"}} />
        <Carousel images={images2} style={{marginInline:"150px"}}/>

        <div className="marketplace_button">
          <a href="/marketplace">Go to Marketplace</a>
        </div>

      </div>

      <h1
        id="how-to-start"
        className="title"
      >
        How to start
      </h1>
      <HowToStart />
      <h1
        id="faq"
        className="title"
      >
        FAQ
      </h1>
      <FAQ />
    </div>
  );
}

export { Home };
