import {
  TbHexagonNumber1,
  TbHexagonNumber2,
  TbHexagonNumber3
} from "react-icons/tb";
import extension from "@/assets/images/howto/extension.png";
import signin from "@/assets/images/howto/signin.png";
import marketplace from "@/assets/images/marketplace.svg";
import { Step } from "./Step";

function HowToStart() {
  return (
    <div>

      <Step icon={TbHexagonNumber1}
       title="Download the"
       url_text=" Polkadot extension"
       url="https://polkadot.js.org/extension/"
       content="Create and sign transactions with your accounts" 
       image={extension}/>

      <Step icon={TbHexagonNumber2}
       title="Connect your account with Rutzo"
       url=""
       url_text=""
       content="Select the account to save your NFTs and start playing" 
       image={signin}/>

      <Step icon={TbHexagonNumber3}
       title="Enter the "
       url_text="Marketplace"
       url="/marketplace"
       content="And get amazing NFTs" 
       image={marketplace}/>

    </div>
  );
}

export { HowToStart };
