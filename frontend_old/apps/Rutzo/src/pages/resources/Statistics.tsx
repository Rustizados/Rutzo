import { TbTimelineEvent } from "react-icons/tb";
import { FaUsers } from "react-icons/fa";
import { RiNftFill } from "react-icons/ri";

const data = [
    {title: "Founded", value: "2023", icon: TbTimelineEvent},
    {title: "Total Users", value: "10+", icon: FaUsers},
    {title: "NFTs", value: "100", icon: RiNftFill}
];

function Statistics() {
    const getItems = () =>
        data.map(({ title, value, icon: Icon }) => (
            <div className="flex flex-col items-center m-5 sm:m-0">
                <Icon className="text-4xl text-green-500" />
                <h2 className="text-2xl font-bold mt-2 text-center">{title}</h2>
                <p>{value}</p>
            </div>
        ));

    return (
        <div className="m-5 sm:m-20 justify-center bg-gray-950 rounded-2xl">
            <div className="block justify-between mx-5 sm:mx-20 p-5 sm:p-10 sm:flex">
                {getItems()}
            </div>
        </div>
    );
}

export { Statistics };