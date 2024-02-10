import React, { useState, useEffect } from "react";
import { Features } from "./Features";
import { HowToStart } from "./HowToStart/HowToStart";
import { FAQ } from "./FAQ";
import "./Home.scss";
import { Play } from '@/components/play/Play';
import { Carousel } from '@/components/layout/carousel/Carousel';
import { Member } from "./Members/Member";
import { Members } from "./Members/Members";
import { Community } from "../resources/Community";

function Home() {

  const [currentImageIndex, setCurrentImageIndex] = useState(0);

  const bannerImages = [
    "https://home.rutzo.studio/NFT/poison/quetzal_poison.jpg",
    "https://home.rutzo.studio/NFT/lightning/nova_lighting.jpg",
    "https://home.rutzo.studio/NFT/poison/angel_of_death_poison.jpg",
  ];

useEffect(() => {
  const intervalId = setInterval(() => {
    setCurrentImageIndex((prevIndex) =>
      prevIndex === bannerImages.length - 1 ? 0 : prevIndex + 1
    );
  }, 3000);

  return () => clearInterval(intervalId); // Cleanup the interval on component unmount
}, [currentImageIndex]);

return (
<div className="About">
  <div className="Main">
    <div className="MainTitle">
      <h1>
        Challenge and conquer the <span className="GradientTitle">World of NFTs</span>
      </h1>
      <p>Play epic battles on Rutzo and become the king of NFT cards. As long as you win more battles, you´ll win more
        cards.</p>
      < Play style={{ marginTop: "40px" }} id="action_button" link="/play" />
    </div>
    <div className="Banner">
      <img src={bannerImages[(currentImageIndex+1)%3]} alt="NFTs"
        style={{ transform: 'rotate(20deg)' }} />
      <img src={bannerImages[(currentImageIndex+2)%3]} alt="NFTs"
        style={{ transform: 'rotate(10deg)' }} />
      <img src={bannerImages[currentImageIndex]} alt="NFTs" />
    </div>
  </div>

  <div className="section">
    <h1 id="features" className="title text-4xl font-extrabold dark:text-white">Features</h1>
    <Features />
  </div>  

  <div className="centered">

    <div className="section">
      <div className="section-title" id="explore">
        <h1>Explore the marketplace</h1>
      </div>
      <div className="section-text">
        <p>
          Play epic battles on Rutzo and become the king of NFT cards. As long as you win more battles, you'll win more
          cards.
        </p>
      </div>
      <Carousel />

      <div className="marketplace_button">
        <a href="/marketplace">Go to Marketplace</a>
      </div>
    </div>

    <div className="section" id="how-to-start">
      <div className="section-title">
        <h1>How to start</h1>
      </div>
      <div className="section-text">
        <p>
          Wanna get started? It's easy! Just follow these steps and you'll be ready to play in no time.
        </p>
      </div>
      <HowToStart />
    </div>

    <div className="section" id="team">
      <div className="section-title">
        <h1>Our team</h1>
      </div>
      <div className="section-text">
        <p>
        We’re a dynamic group of individuals who are passionate about what we do and dedicated to delivering the best results for our clients.
        </p>
      </div>
      < Members />
    </div>

    <div className="section" id="faq">
      <div className="section-title">
        <h1>FAQ</h1>
      </div>
      <div className="section-text">
        <p>
          Do you have any questions? Check out our FAQ section and find the answers you're looking for.
        </p>
      </div>
      <FAQ />
    </div>

  </div>

  <Community />

</div>
);
}

export { Home };
