import { Socials } from '@/components/layout/footer/socials';

function Community() {
  return (
    <div className="m-5 lg:m-20 justify-center text-center">
      <h1 className="text-3xl md:text-5xl font-semibold p-10 md:p-16">
        Join the <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">community</span>
      </h1>
      <div className="flex">
        <Socials className="text-4xl mx-auto md:space-x-24" />
      </div>
    </div>
  );
}

export { Community };
