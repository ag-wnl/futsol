import { AppHeader } from "@/components/header";

const Hero = () => {
  return (
    <section
      className={
        "mx-auto max-w-7xl px-[32px] relative flex items-center justify-between mt-16 mb-12"
      }
    >
      <div className={"text-center w-full "}>
        <h1
          className={
            "text-[48px] leading-[48px] md:text-[80px] md:leading-[80px] tracking-[-1.6px] font-bold"
          }
        >
          Become part of
          <br />
          the game
        </h1>
        <p
          className={
            "mt-6 text-[18px] leading-[27px] md:text-[20px] md:leading-[30px]"
          }
        >
          Cheer for your team, and be part of the victory.
        </p>
      </div>
    </section>
  );
};

export default function Home() {
  return (
    <main className="w-full max-w-[3xl] items-center  flex flex-col gap-10">
      <AppHeader />
      <Hero />
    </main>
  );
}
