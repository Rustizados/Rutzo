import { useState, useEffect } from "react";
import { ReactComponent as CopyIcon } from "@/assets/images/copy.svg";
import { useNavigate } from "react-router-dom";
import { GearKeyring } from '@gear-js/api';

function SignlessDialog({ onClose }: { onClose: () => void }) {
  const [walletAccount, setWalletAccount] = useState("0x123456789");
  const [copied, setCopied] = useState(false);
  const [duration, setDuration] = useState("");
  const [password, setPassword] = useState("");
  const navigate = useNavigate();

  const handleCopy = () => {
    navigator.clipboard.writeText(walletAccount);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const generateRandomAccount = async (passphrase: string) => {
    try {
      const mnemonic = GearKeyring.generateMnemonic();
      const { seed } = GearKeyring.generateSeed(mnemonic);
      const keyring = await GearKeyring.fromSeed(seed, passphrase);
      return "0x"+keyring.address;
    } catch (error) {
      console.error("Error generating the account:", error);
      return "0x0000000000000000000000000000000000000000";
    }
  };

  useEffect(() => {
    const fetchAccount = async () => {
      const account = await generateRandomAccount(password);
      setWalletAccount(account);
    };
    if (true) {
      fetchAccount();
    }
  }, [password]);

  const validateFields = () => {
    if (!duration) {
      alert("Please select a duration.");
      return false;
    }
    if (!password) {
      alert("Please enter a password.");
      return false;
    }
    return true;
  };

  const handleSubmit = () => {
    if (validateFields()) {
      onClose();
      navigate("/selection");
    }
  };

  return (
    <div
      className="fixed z-10 inset-0 overflow-y-auto"
      aria-labelledby="modal-title"
      role="dialog"
      aria-modal="true"
    >
      <div className="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        <div
          className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
          aria-hidden="true"
        ></div>

        <span
          className="hidden sm:inline-block sm:align-middle sm:h-screen"
          aria-hidden="true"
        >
          &#8203;
        </span>

        <div className="inline-block align-bottom bg-black rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-96">
          <div className="flex justify-end mt-5 mr-5">
            <button onClick={onClose}>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                className="h-6 w-6"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>
          <div className="px-4 pb-4 sm:p-8 sm:pt-0">
            <div className="flex">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                viewBox="0 0 24 24"
                stroke="currentColor"
                className="h-6 w-6 mr-2"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M13 10V3L4 14h7v7l9-11h-7z"
                />
              </svg>
              <p className="mr-10 text-base font-bold">
                Enable Signless Session
              </p>
            </div>
            <p className="mr-10 mt-5 text-base font-bold">
              Randomly generated account
            </p>
            <div className="flex mt-2 justify-between items-center">
              <input
                type="text"
                readOnly
                value={walletAccount}
                className="mr-2 bg-black flex-grow mono text-xs"
              />
              <button onClick={handleCopy} className="text-right">
                <CopyIcon />
                {copied && <p className="text-xs text-gray-400">Copied</p>}
              </button>
            </div>
            <div className="flex justify-between items-center mt-5">
              <p className="text-base font-bold">Voucher to issue</p>
              <p className="text-green-500 mono">0 TVara</p>
            </div>

            <div className="mt-5">
              <label htmlFor="duration" className="block mb-2">
                Duration
              </label>
              <select
                id="duration"
                value={duration}
                onChange={(e) => setDuration(e.target.value)}
                className="w-full bg-black border-purple-800 border-2 rounded-lg p-1"
              >
                <option value="">Select duration</option>
                <option value="5">5 minutes</option>
                <option value="10">10 minutes</option>
                <option value="20">20 minutes</option>
                <option value="40">40 minutes</option>
                <option value="60">1 hour</option>
              </select>
            </div>

            <div className="mt-5">
              <label htmlFor="password" className="block mb-2">
                Password
              </label>
              <input
                type="password"
                id="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="w-full bg-black text-white border-purple-800 border-2 rounded-lg p-1"
              />
            </div>

            <div className="flex justify-center items-center">
              <button
                onClick={handleSubmit}
                className="mt-5 bg-gradient-to-r from-purple-800 to-green-500 rounded-full p-2 w-52"
              >
                Create Signless Session
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export { SignlessDialog };
