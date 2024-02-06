import { TwitterTweetEmbed, TwitterFollowButton } from 'react-twitter-embed';

function Story() {
  return (
    <div className=" text-center rounded-3xl flow-root">
      <h1 className="text-3xl sm:text-5xl font-semibold m-10 sm:m-16">
      <span className="bg-gradient-to-r from-purple-800 to-green-500 rounded-xl">Our Story</span></h1>
      <div className="flex flex-col-reverse sm:flex md:flex-row">
        <div className="w-full sm:w-1/2 p-0 md:px-20">
          <TwitterTweetEmbed tweetId={'1676410571124625409'} />
          <TwitterFollowButton screenName={'rustizados'} />
        </div>
        <div className="w-full sm:w-1/2 p-5 md:px-20">
          <p className="text-left">
            In June 2023, Vara Network and Gear organized the Main Hackathon 2023 at the ITAM facilities where more than
            20 teams took place to compete creating innovative projects in the blockchain and Web3 fields.
            <br />
            <br />
            Among those teams was the Rustizados team, a team made up of 5 students from the Escuela Superior de Cómputo
            del Instituto Politécnico Nacional. Throughout the almost 48 hours of the hackathon they developed their
            idea non-stop. The goal was still a little fuzzy, they wanted to create something innovative and with social
            impact, but they also wanted to have fun and build something different from the traditional. This is why
            they chose to compete in the GameFi category and create a game that used NFTs as cards for each other,
            making reference to the traditional Mexican tazos in their childhood.
            <br />
            <br />
            After 48 hours, it was time to present the project, just a few minutes to reflect all the effort and
            sleepless hours. The presentation passed and the only thing left to do was to wait for the result. After
            some activities to relax from all the work done, it was time to know the result, we won the third place of
            the hackathon!
            <br />
            <br />
            It was unexpected, but all the effort made paid off. After the hackathon, they could not abandon the project
            that had led them to victory, so they continued working on it and some time later they entered the second
            season of the Varathon, a virtual hackathon with participants from all over the world. It was the second
            time they showed the project, now more advanced and ready for the next level.
            <br />
            <br />
            Fortunately among about 10 participating teams, they managed to establish themselves in third place, it was
            another victory for Rustizados and Rutzo. Now, after a few months of development, Rutzo is finally here,
            ready for the next stage and growing every day.
          </p>
        </div>
      </div>
    </div>
  );
}

export { Story };
