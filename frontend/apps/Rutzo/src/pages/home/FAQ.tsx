import { Disclosure, Transition } from '@headlessui/react';
import { ChevronUpIcon } from '@heroicons/react/20/solid';

const FAQs = [
  {
    question: 'How do I create an account?',
    answer:
      'To create an account, first download the Polkadot.js browser extension. Once installed, open it and follow the prompts to create your wallet. Ensure you save your recovery phrase in a secure location to avoid losing access to your account. Once your wallet is created, navigate to the "Sign in" section on our platform and connect your wallet to create your account. ',
  },
  {
    question: 'How can I purchase NFTs?',
    answer:
      'To purchase NFTs, navigate to the "Marketplace" section and browse through the available collections. Select the NFT you want to purchase and follow the prompts to complete the transaction.',
  },
  {
    question: 'Can I trade or sell my owned NFTs?',
    answer:
      'At the moment, you can\'t trade or sell your owned NFTs on our platform. We are working on implementing this feature in the near future.',
  },
  {
    question: 'How do I participate in battles?',
    answer:
    'Battles are the heart of Rutzo, where players can compete using their owned NFTs. To participate, make sure you have at least 3 NFTs in your collection, navigate to the "Play" section, select your best cards, and follow the instructions to enter the battle.',
  },
  {
    question: 'What happens if I win a battle?',
    answer:
      'Winning a battle rewards you with additional NFTs and in-game rewards. The conquered NFTs from defeated opponents become part of your collection, symbolizing your victory.',
  },
];

function FAQ() {
  return (
    <div className="w-full px-4 pt-4">
      <div className="mx-auto w-full rounded-2xl bg-background p-2">
        {FAQs.map((faq) => (
          <Disclosure as="div" className="mt-2">
            {({ open }) => (
              <>
                <Disclosure.Button
                  className={`flex w-full justify-between rounded-lg px-4 py-2 text-left text-sm font-medium text-white hover:bg-gradient-to-r from-purple-800 to-green-500 focus:outline-none focus-visible:ring focus-visible:ring-purple-500/75 ${
                    open ? 'bg-gradient-to-r from-purple-800 to-green-500' : ''
                  }`}>
                  <span className="text-base">{faq.question} </span>
                  <ChevronUpIcon
                    className={`${
                      open ? 'rotate-180 transform' : ''
                    } h-5 w-5 text-slate-50 transition-all duration-300`}
                  />
                </Disclosure.Button>
                <Transition
                  show={open}
                  enter="transition-all duration-200"
                  enterFrom="opacity-0"
                  enterTo="opacity-100"
                  leave="transition-all duration-200"
                  leaveFrom="opacity-100"
                  leaveTo="opacity-0">
                  <Disclosure.Panel
                    className={`px-8 pb-2 pt-4 text-sm text-gray-50 transition-all duration-300 ${
                      open ? 'opacity-100 visible' : 'opacity-0 invisible'
                    }`}>
                    {faq.answer}
                  </Disclosure.Panel>
                </Transition>
              </>
            )}
          </Disclosure>
        ))}
      </div>
    </div>
  );
}

export { FAQ };
