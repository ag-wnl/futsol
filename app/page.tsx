import { AppHeader } from "@/components/header";

const Hero = () => {
  return (
    <section className="flex flex-col items-center justify-center">
      <div className="flex flex-col gap-4 items-center">
        <h1 className="text-5xl font-bold">Predictions market for sports</h1>
        <span>Vouch for your team and be a part of the victory</span>
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
