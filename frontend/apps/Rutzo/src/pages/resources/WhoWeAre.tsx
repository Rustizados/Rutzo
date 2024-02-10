function WhoWeAre() {
  return (
    <div className="bg-slate-50 text-center bg-opacity-5 rounded-3xl flow-root">
      <h1 className="text-3xl sm:text-5xl font-semibold m-10 sm:m-16">Who is Rutzo</h1>
      <p className="mx-10 sm:mx-20">
        Allow us to introduce ourselves! Hear from our team, about who we are, how we started, and where we're headed.
      </p>

      <iframe
        width="900"
        height="500"
        src="https://www.youtube.com/embed/vA6TmxxRA04?si=S1xSTJRMKBPH48tY"
        title="YouTube video player"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        allowFullScreen
        className="mx-auto m-10 h-60 w-10/12 sm:h-screen sm:w-full sm:p-10"></iframe>
    </div>
  );
}

export { WhoWeAre };
