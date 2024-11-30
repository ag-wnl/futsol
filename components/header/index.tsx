import { Button } from "../ui/button";

export const AppHeader = () => {
  return (
    <header className="w-full flex flex-row justify-between items-center px-20 py-10">
      <section>
        <h1 className="text-3xl font-bold">futsol</h1>
      </section>

      <section className="flex gap-4 items-center">
        <Button>Play</Button>

        <Button>About</Button>

        <Button>Connect</Button>
      </section>
    </header>
  );
};
