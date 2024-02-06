import { PoweredBy } from './PoweredBy';
import { Statistics } from './Statistics';
import { Story } from './Story';
import { WhoWeAre } from './WhoWeAre';

function AboutUs() {
  return (
    <div className="h-auto">
      <div className="flex-col md:flex md:flex-row">
        <div className="w-full md:w-1/2 p-5 md:p-10 ">
          <h1 className=" text-3xl md:text-5xl font-semibold mb-6 ">
            Evolving <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">Web3 gaming</span>
          </h1>
          <p>
            Rutzo is a blockchain-based gaming platform that allows users to play epic battles and become the king of
            NFT cards. As long as you win more battles, you´ll win more cards.
            <br />
            <br />
            Our platform is designed to provide a unique gaming experience that leverages the benefits of blockchain
            technology. We are committed to creating a decentralized gaming ecosystem that is secure, transparent, and
            fair for all users.
            <br />
            <br />
            Our platform is built on the Vara network, which ensures that all in-game assets are secure and
            tamper-proof. We are dedicated to providing a gaming experience that is both entertaining and rewarding for
            our users.
          </p>
        </div>

        <div className="w-full md:w-1/2">
          <img
            src="https://media.licdn.com/dms/image/D4E22AQHEOZcvBrxjhQ/feedshare-shrink_1280/0/1694568627416?e=1710374400&v=beta&t=Ku-7IiRCL-ZKPyytYLU0czZflgJfqQlspqywQGyGaQ8"
            alt="NFTs"
            className="h-96 rounded-3xl m-auto md:h-96 md:w-96 md:object-cover md:object-center md:shadow-2"
          />
        </div>
      </div>

      <Statistics />

      <WhoWeAre />

      <Story />

      <PoweredBy />
    </div>
  );
}

export { AboutUs };
