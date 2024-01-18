import { Features } from "./Features";
import { HowToStart } from "./HowToStart";
import { FAQ } from "./FAQ";
import "./Home.scss";
import { Play } from '@/components/play/Play';

function Home() {
  return (
    <div className="About">
      <div className="Main">
        <div className="MainTitle">
          <h1>
          Challenge and conquer the <span className="GradientTitle">World of NFTs</span>
          </h1>
          <p>Play epic battles on Rutzo and become the king of NFT cards. As long as you win more battles, you´ll win more cards.</p>
          < Play style={{marginTop: "40px"}}/>
        </div>
        <div className="Banner">
          <img src="https://home.rutzo.studio/NFT/poison/quetzal_poison.jpg" alt="NFTs" style={{transform: 'rotate(20deg)'}}/>
          <img src="https://home.rutzo.studio/NFT/lightning/nova_lightning.jpg" alt="NFTs" style={{transform: 'rotate(10deg)'}}/>
          <img src="https://home.rutzo.studio/NFT/poison/quetzal_poison.jpg" alt="NFTs"/>
        </div>
      </div>

      <h1
        style={{ textAlign: "center", margin: "5%" }}
        id="features"
        className="Title"
      >
        Features
      </h1>
      <Features />
      <h1
        style={{ textAlign: "center", margin: "5%" }}
        id="how-to-start"
        className="title"
      >
        How to start
      </h1>
      <HowToStart />
      <h1
        style={{ textAlign: "center", margin: "5%" }}
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
