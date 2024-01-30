
import {Disclosure, Transition} from "@headlessui/react";
// import "bootstrap/dist/css/bootstrap.min.css";
import "./home_styles.css";
import {ChevronUpIcon} from "@heroicons/react/20/solid";

const FAQs = [
  {
    "question": "How do I create an account?",
    "answer": "To create an account, click on the \"Sign Up\" button on the homepage and follow the instructions to provide the required information."
  },
  {
    "question": "How can I purchase NFTs?",
    "answer": "To purchase NFTs, navigate to the \"Marketplace\" section and browse through the available collections. Select the NFT you want to purchase and follow the prompts to complete the transaction."
  },
  {
    "question": "Can I trade or sell my owned NFTs?",
    "answer": "Yes, you can trade or sell your owned NFTs on our platform. Visit the \"My Collection\" section and select the NFT you want to trade or sell. Follow the provided options to initiate the transaction."
  },
  {
    "question": "How do I participate in battles?",
    "answer": "Battles are scheduled events where players can compete using their owned NFTs. To participate, navigate to the \"Battles\" section, select the available battle, and follow the instructions to enter your NFTs into the battle."
  },
  {
    "question": "What happens if I win a battle?",
    "answer": "Winning a battle rewards you with additional NFTs and in-game rewards. The conquered NFTs from defeated opponents become part of your collection, symbolizing your victory."
  }
]


function FAQ() {
  return (
    <div className="w-full px-4 pt-4">
      <div className="mx-auto w-full rounded-2xl bg-background p-2">
      {
        FAQs.map((faq) => (

          <Disclosure as='div' className='mt-2'>
            {({ open }) => (
              <>


                <Disclosure.Button className={'flex w-full justify-between rounded-lg bg-background-gradient px-4 py-2 text-left text-sm font-medium text-light-blue hover:bg-acrylic focus:outline-none focus-visible:ring focus-visible:ring-purple-500/75'}>
                  <span>{faq.question} </span>
                  <ChevronUpIcon
                    className={`${
                      open ? 'rotate-180 transform' : ''
                    } h-5 w-5 text-sky-500`}
                  />
                </Disclosure.Button>
                <Transition

                  show={open}
                  enter="transition-all duration-500"
                  enterFrom="opacity-0"
                  enterTo="opacity-100"
                  leave="transition-all duration-500"
                  leaveFrom="opacity-100"
                  leaveTo="opacity-0"
                >
                  <Disclosure.Panel className={`px-4 pb-2 pt-4 text-sm text-gray-500 transition-all duration-300 ${open ? 'opacity-100 visible' : 'opacity-0 invisible'}`}>
                    {faq.answer}
                  </Disclosure.Panel>
              </Transition>
              </>
            )}

          </Disclosure>

        ))
      }
      </div>
    </div>
  );
}

export { FAQ };
