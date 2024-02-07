const founders = [
  {
    name: 'Alan Gomez',
    title: 'NFT designer & Developer',
    imageUrl: 'https://avatars.githubusercontent.com/u/92237260?v=4',
    github: 'https://github.com/Alangh0011',
    active: false,
  },
  {
    name: 'Brandon Herrera',
    title: 'Frontend Developer & UI/UX Designer',
    imageUrl: 'https://avatars.githubusercontent.com/u/39093860?v=4',
    github: 'https://github.com/brandonhxrr',
    active: true,
  },
  {
    name: 'Juan M. Hernández',
    title: 'Fullstack Developer',
    imageUrl: 'https://avatars.githubusercontent.com/u/61924317?v=4',
    github: 'https://github.com/JuanH44',
    active: true,
  },
  {
    name: 'David Hernández',
    title: 'Backend Developer',
    imageUrl: 'https://avatars.githubusercontent.com/u/67880616?v=4',
    github: 'https://github.com/David-HernandezM',
    active: true,
  },
  {
    name: 'Ricardo Mora',
    title: 'Backend Developer',
    imageUrl: 'https://avatars.githubusercontent.com/u/13629744?v=4',
    github: 'https://github.com/RicardoUMC',
    active: false,
  },
];

function Founders() {
  return (
    <div className="m-5 lg:m-20 justify-center bg-slate-50 bg-opacity-5 rounded-2xl text-center">
      <h1 className="text-3xl md:text-5xl font-semibold p-10 md:p-16">
        Meet our <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">founders</span>
      </h1>
      <div className="block justify-between mx-5 p-5 lg:p-5 lg:flex">
        {founders.map((founder) => (
          <a href={founder.github} className="cursor-pointer" target="_blank">
            <div className="m-5 w-40 lg:w-auto mx-auto lg:m-10">
              <img
                src={founder.imageUrl}
                alt={founder.name}
                className={`rounded-full ${founder.active ? '' : 'grayscale'}`}
              />
              <h2 className="mt-5 m-2">{founder.name}</h2>
              <p>{founder.title}</p>
            </div>
          </a>
        ))}
      </div>
    </div>
  );
}

export { Founders };
